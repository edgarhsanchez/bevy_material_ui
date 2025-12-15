"""
Bevy Material UI - Quick Test Script
====================================

Runs targeted tests on specific components and reports findings for AI iteration.
Now with telemetry support - reads component state from telemetry.json
Includes visual regression testing via screenshot comparison.
"""

from __future__ import annotations

import subprocess
import time
import sys
import json
import argparse
from pathlib import Path
from datetime import datetime
from dataclasses import dataclass

try:
    import pyautogui
    from PIL import ImageGrab
except ImportError:
    subprocess.run([sys.executable, "-m", "pip", "install", "pyautogui", "pillow", "numpy"], check=True)
    import pyautogui
    from PIL import ImageGrab

# Import our visual diff module
from visual_diff import compare_with_baseline, generate_report, save_baseline

pyautogui.FAILSAFE = False
pyautogui.PAUSE = 0.1

# Stop the automation run on the first actionable failure.
FAIL_FAST = True


class TestFailure(RuntimeError):
    pass

OUTPUT_DIR = Path(__file__).parent / "test_output"
OUTPUT_DIR.mkdir(exist_ok=True)
WORKSPACE_DIR = Path(__file__).parent.parent.parent
TELEMETRY_FILE = WORKSPACE_DIR / "telemetry.json"
LAST_FAILURE_FILE = OUTPUT_DIR / "last_failure.json"


def _normalize_section_token(value: str) -> str:
    return (value or "").strip().lower().replace(" ", "").replace("_", "")


def save_last_failure(payload: dict, path: Path = LAST_FAILURE_FILE) -> None:
    try:
        payload = dict(payload)
        payload.setdefault("timestamp", datetime.now().isoformat())
        path.write_text(json.dumps(payload, indent=2), encoding="utf-8")
    except Exception:
        # Never block test runs if failure-state persistence fails.
        pass


def load_last_failure(path: Path = LAST_FAILURE_FILE) -> dict | None:
    try:
        if not path.exists():
            return None
        return json.loads(path.read_text(encoding="utf-8"))
    except Exception:
        return None


def clear_last_failure(path: Path = LAST_FAILURE_FILE) -> None:
    try:
        if path.exists():
            path.unlink()
    except Exception:
        pass


def _filter_sections_for_run(
    sections: list[tuple[str, str, list[str]]],
    start_from: str | None,
    only: list[str] | None,
) -> list[tuple[str, str, list[str]]]:
    ordered = list(sections)

    if start_from:
        start_tok = _normalize_section_token(start_from)
        start_index = None
        for i, (section_name, nav_id, _req) in enumerate(ordered):
            if _normalize_section_token(section_name) == start_tok:
                start_index = i
                break
            if _normalize_section_token(nav_id) == start_tok:
                start_index = i
                break
        if start_index is None:
            raise TestFailure(f"[FAIL] Unknown --start-from section '{start_from}'")
        ordered = ordered[start_index:]

    if only:
        only_toks = {_normalize_section_token(v) for v in only if v.strip()}
        filtered: list[tuple[str, str, list[str]]] = []
        for section_name, nav_id, req in ordered:
            if _normalize_section_token(section_name) in only_toks or _normalize_section_token(nav_id) in only_toks:
                filtered.append((section_name, nav_id, req))
        if not filtered:
            raise TestFailure(f"[FAIL] --only did not match any sections: {only}")
        ordered = filtered

    return ordered


def _filter_sizes_for_resume(
    sizes: list[WindowSize],
    resume_size_name: str | None,
    resume_all_sizes: bool,
) -> list[WindowSize]:
    if not resume_size_name:
        return sizes

    tok = _normalize_section_token(resume_size_name)
    idx = None
    for i, s in enumerate(sizes):
        if _normalize_section_token(s.name) == tok:
            idx = i
            break

    if idx is None:
        return sizes

    if resume_all_sizes:
        return sizes[idx:]
    return [sizes[idx]]

# Track visual regression results
visual_results = []

# Track active window bounds for click validation
_window_bounds = None  # (left, top, right, bottom) of the application window
_client_origin = None  # (x, y) of the client area origin
_bevy_hwnd = None  # Win32 HWND for the application window (Windows only)
_showcase_log_handles: list[object] = []
_last_showcase_stdout_log: Path | None = None
_last_showcase_stderr_log: Path | None = None


@dataclass(frozen=True)
class WindowSize:
    name: str
    width: int
    height: int


SIZE_PRESETS: dict[str, WindowSize] = {
    # Chosen to fit on most displays while stressing responsive layout.
    "phone": WindowSize("phone", 480, 800),
    "tablet": WindowSize("tablet", 768, 1024),
    "desktop": WindowSize("desktop", 1280, 720),
}


def _tail_text_file(path: Path, max_bytes: int = 4000) -> str:
    try:
        data = path.read_bytes()
        if len(data) > max_bytes:
            data = data[-max_bytes:]
        return data.decode(errors="replace")
    except Exception:
        return ""


def _ensure_showcase_built(env: dict) -> Path:
    exe = WORKSPACE_DIR / "target" / "release" / "examples" / "showcase.exe"
    if exe.exists():
        return exe

    print("  Building showcase.exe (release)…")
    subprocess.run(
        ["cargo", "build", "--example", "showcase", "--release"],
        cwd=WORKSPACE_DIR,
        env=env,
        check=True,
    )
    return exe


def launch_showcase(env: dict) -> subprocess.Popen:
    """Launch the showcase app.

    Uses the built example binary directly so the OS window PID matches the
    spawned process. Logs stdout/stderr to files to avoid PIPE buffer deadlocks.
    """
    global _showcase_log_handles, _last_showcase_stdout_log, _last_showcase_stderr_log

    exe = _ensure_showcase_built(env)
    cmd = [str(exe)]

    ts = datetime.now().strftime("%Y%m%d_%H%M%S")
    _last_showcase_stdout_log = OUTPUT_DIR / f"showcase_{ts}.stdout.log"
    _last_showcase_stderr_log = OUTPUT_DIR / f"showcase_{ts}.stderr.log"

    # Keep handles alive for the duration of the child process.
    out_f = open(_last_showcase_stdout_log, "wb")
    err_f = open(_last_showcase_stderr_log, "wb")
    _showcase_log_handles = [out_f, err_f]

    return subprocess.Popen(
        cmd,
        cwd=WORKSPACE_DIR,
        stdout=out_f,
        stderr=err_f,
        env=env,
    )


def focus_bevy_window():
    """Best-effort: keep the Bevy window in the foreground before input."""
    global _bevy_hwnd
    if _bevy_hwnd is None:
        return
    try:
        import ctypes
        import ctypes.wintypes

        user32 = ctypes.windll.user32

        def get_thread_id(hwnd):
            pid = ctypes.wintypes.DWORD(0)
            tid = user32.GetWindowThreadProcessId(hwnd, ctypes.byref(pid))
            return tid

        foreground = user32.GetForegroundWindow()
        foreground_tid = get_thread_id(foreground) if foreground else 0
        target_tid = get_thread_id(_bevy_hwnd)

        # Allow foreground change by temporarily attaching input.
        if foreground_tid and target_tid and foreground_tid != target_tid:
            user32.AttachThreadInput(foreground_tid, target_tid, True)

        user32.SetForegroundWindow(_bevy_hwnd)

        # Refresh bounds in case the window changed size/position.
        class RECT(ctypes.Structure):
            _fields_ = [("left", ctypes.c_long), ("top", ctypes.c_long),
                        ("right", ctypes.c_long), ("bottom", ctypes.c_long)]
        rect = RECT()
        if user32.GetWindowRect(_bevy_hwnd, ctypes.byref(rect)):
            client_point = ctypes.wintypes.POINT(0, 0)
            user32.ClientToScreen(_bevy_hwnd, ctypes.byref(client_point))
            set_window_bounds((rect.left, rect.top, rect.right, rect.bottom), (client_point.x, client_point.y))

        if foreground_tid and target_tid and foreground_tid != target_tid:
            user32.AttachThreadInput(foreground_tid, target_tid, False)

        time.sleep(0.02)
    except Exception:
        return


def _try_resize_window(hwnd: int, client_width: int, client_height: int) -> bool:
    """Resize the window so the *client area* is approximately (client_width x client_height)."""
    try:
        import ctypes
        import ctypes.wintypes

        user32 = ctypes.windll.user32

        GWL_STYLE = -16
        GWL_EXSTYLE = -20
        SW_SHOWNORMAL = 1

        style = user32.GetWindowLongW(hwnd, GWL_STYLE)
        ex_style = user32.GetWindowLongW(hwnd, GWL_EXSTYLE)

        class RECT(ctypes.Structure):
            _fields_ = [
                ("left", ctypes.c_long),
                ("top", ctypes.c_long),
                ("right", ctypes.c_long),
                ("bottom", ctypes.c_long),
            ]

        rect = RECT(0, 0, int(client_width), int(client_height))
        if not user32.AdjustWindowRectEx(ctypes.byref(rect), style, False, ex_style):
            return False

        outer_width = rect.right - rect.left
        outer_height = rect.bottom - rect.top

        # Ensure we are in a resizable "normal" window state
        user32.ShowWindow(hwnd, SW_SHOWNORMAL)

        SWP_NOZORDER = 0x0004
        SWP_NOACTIVATE = 0x0010
        if not user32.SetWindowPos(hwnd, 0, 0, 0, outer_width, outer_height, SWP_NOZORDER | SWP_NOACTIVATE):
            return False

        time.sleep(0.25)
        return True
    except Exception:
        return False


def set_bevy_window_client_size(width: int, height: int) -> bool:
    """Resize the active Bevy window; refresh global bounds afterwards."""
    global _bevy_hwnd
    if _bevy_hwnd is None:
        return False
    if not _try_resize_window(_bevy_hwnd, width, height):
        return False

    # Refresh bounds/client origin
    try:
        import ctypes
        import ctypes.wintypes

        user32 = ctypes.windll.user32

        class RECT(ctypes.Structure):
            _fields_ = [
                ("left", ctypes.c_long),
                ("top", ctypes.c_long),
                ("right", ctypes.c_long),
                ("bottom", ctypes.c_long),
            ]

        rect = RECT()
        if user32.GetWindowRect(_bevy_hwnd, ctypes.byref(rect)):
            client_point = ctypes.wintypes.POINT(0, 0)
            user32.ClientToScreen(_bevy_hwnd, ctypes.byref(client_point))
            set_window_bounds((rect.left, rect.top, rect.right, rect.bottom), (client_point.x, client_point.y))
    except Exception:
        pass

    return True


def parse_sizes_arg(s: str) -> list[WindowSize]:
    """Parse a comma-separated size list like 'phone,tablet,1280x720'."""
    if not s:
        return [SIZE_PRESETS["phone"], SIZE_PRESETS["tablet"], SIZE_PRESETS["desktop"]]

    sizes: list[WindowSize] = []
    for part in [p.strip().lower() for p in s.split(",") if p.strip()]:
        if part in SIZE_PRESETS:
            sizes.append(SIZE_PRESETS[part])
            continue
        if "x" in part:
            w_str, h_str = part.split("x", 1)
            try:
                w = int(w_str)
                h = int(h_str)
                sizes.append(WindowSize(part, w, h))
                continue
            except ValueError:
                raise ValueError(f"Invalid size '{part}'. Use preset (phone/tablet/desktop) or WxH.")
        raise ValueError(f"Invalid size '{part}'. Use preset (phone/tablet/desktop) or WxH.")

    return sizes


def set_window_bounds(window_rect, client_origin):
    """Set the window bounds for click validation"""
    global _window_bounds, _client_origin
    _window_bounds = window_rect
    _client_origin = client_origin
    if window_rect:
        print(f"  Click bounds set: ({window_rect[0]}, {window_rect[1]}) to ({window_rect[2]}, {window_rect[3]})")


def is_click_in_bounds(x: float, y: float) -> tuple[bool, str]:
    """Check if a click coordinate is within the application window bounds.
    Returns (is_valid, reason_if_invalid)"""
    if _window_bounds is None:
        return True, ""  # No bounds set, allow click
    
    left, top, right, bottom = _window_bounds
    
    # Add small margin (5px) to account for window decorations
    margin = 5
    
    if x < left + margin:
        return False, f"x={x:.0f} is left of window (left bound: {left})"
    if x > right - margin:
        return False, f"x={x:.0f} is right of window (right bound: {right})"
    if y < top + margin:
        return False, f"y={y:.0f} is above window (top bound: {top})"
    if y > bottom - margin:
        return False, f"y={y:.0f} is below window (bottom bound: {bottom})"
    
    return True, ""


def safe_click(x: float, y: float, description: str = "") -> bool:
    """Perform a click only if coordinates are within window bounds.
    Returns True if click was performed, False if blocked."""
    is_valid, reason = is_click_in_bounds(x, y)
    
    if not is_valid:
        print(f"  [BLOCKED] Click at ({x:.0f}, {y:.0f}) would be outside application: {reason}")
        if description:
            print(f"            Attempted action: {description}")
        return False
    
    focus_bevy_window()
    pyautogui.moveTo(x, y, duration=0.1)
    time.sleep(0.05)
    pyautogui.click()
    return True


_last_successful_nav_id: str | None = None


def _scroll_sidebar_wheel(client_origin, ticks: int, anchor_test_id: str | None = None) -> None:
    """Scroll the sidebar using the mouse wheel.

    Args:
        client_origin: (x, y) client origin from find_bevy_window.
        ticks: Positive scrolls up, negative scrolls down.
    """
    if not client_origin:
        return

    # Prefer hovering a pickable nav item so Bevy picking hover routing can find the nearest
    # ScrollContainer ancestor.
    anchor = get_element_bounds(anchor_test_id) if anchor_test_id else None

    if anchor:
        sidebar_x = client_origin[0] + anchor["x"] + anchor["width"] / 2
        sidebar_y = client_origin[1] + anchor["y"] + anchor["height"] / 2
    else:
        # Fall back to hovering the scroll container area.
        container = get_element_bounds("sidebar_scroll_container")
        if container:
            sidebar_x = client_origin[0] + container["x"] + container["width"] / 2
            sidebar_y = client_origin[1] + container["y"] + container["height"] / 2
        else:
            telemetry = read_telemetry()
            window_height = 800
            if telemetry:
                try:
                    window_height = int(float(telemetry.get("states", {}).get("window_height", window_height)))
                except Exception:
                    window_height = 800

            sidebar_x = client_origin[0] + 150
            sidebar_y = client_origin[1] + int(window_height * 0.4)

    focus_bevy_window()
    pyautogui.moveTo(sidebar_x, sidebar_y, duration=0.1)
    time.sleep(0.05)
    pyautogui.scroll(int(ticks))
    time.sleep(0.25)


def click_nav_element_with_auto_scroll(nav_id: str, client_origin, max_scroll_attempts: int = 12) -> bool:
    """Click a sidebar nav item, scrolling the sidebar if it is off-screen."""
    container = get_element_bounds("sidebar_scroll_container")

    for attempt in range(1, max_scroll_attempts + 1):
        bounds = get_element_bounds(nav_id)
        if not bounds:
            print(f"  [MISS] Element '{nav_id}' not found in telemetry")
            return False

        # Bounds in telemetry are already in scrolled/screen space. For scroll containers, nav items
        # can be partially clipped; click within the visible intersection when possible.
        click_x_local = bounds["x"] + bounds["width"] / 2
        click_y_local = bounds["y"] + bounds["height"] / 2
        visibility_reason: str | None = None

        if container:
            cont_top = container["y"]
            cont_bottom = container["y"] + container["height"]
            cont_left = container["x"]
            cont_right = container["x"] + container["width"]

            elem_top = bounds["y"]
            elem_bottom = bounds["y"] + bounds["height"]
            elem_left = bounds["x"]
            elem_right = bounds["x"] + bounds["width"]

            vis_left = max(elem_left, cont_left)
            vis_right = min(elem_right, cont_right)
            vis_top = max(elem_top, cont_top)
            vis_bottom = min(elem_bottom, cont_bottom)

            if vis_right - vis_left >= 4.0:
                click_x_local = (vis_left + vis_right) / 2.0
            else:
                if elem_right <= cont_left or elem_left < cont_left:
                    visibility_reason = "left of container"
                elif elem_left >= cont_right or elem_right > cont_right:
                    visibility_reason = "right of container"
                else:
                    visibility_reason = "right of container"

            if visibility_reason is None:
                if vis_bottom - vis_top >= 4.0:
                    click_y_local = (vis_top + vis_bottom) / 2.0
                else:
                    if elem_bottom <= cont_top or elem_top < cont_top:
                        visibility_reason = "above container"
                    elif elem_top >= cont_bottom or elem_bottom > cont_bottom:
                        visibility_reason = "below container"
                    else:
                        visibility_reason = "below container"

        center_x = client_origin[0] + click_x_local
        center_y = client_origin[1] + click_y_local

        is_valid, bounds_reason = is_click_in_bounds(center_x, center_y)
        if is_valid and visibility_reason is None:
            print(
                f"  Clicking '{nav_id}' at ({center_x:.0f}, {center_y:.0f}) "
                f"[bounds: x={bounds['x']:.0f}, y={bounds['y']:.0f}, w={bounds['width']:.0f}, h={bounds['height']:.0f}]"
            )
            return safe_click(center_x, center_y, f"click nav '{nav_id}'")

        reason = visibility_reason or bounds_reason

        # Scroll when blocked vertically.
        if "below window" in reason or "below container" in reason:
            thumb_id = "sidebar_scroll_container_scroll_thumb_v"
            if get_element_bounds(thumb_id):
                print(
                    f"  [SCROLL] '{nav_id}' below viewport (attempt {attempt}/{max_scroll_attempts}); "
                    f"dragging '{thumb_id}' down"
                )
                drag_scrollbar(thumb_id, amount=160, client_origin=client_origin)
                time.sleep(0.25)
            else:
                notches = 6
                print(
                    f"  [SCROLL] '{nav_id}' below viewport (attempt {attempt}/{max_scroll_attempts}); "
                    f"scrolling sidebar down ({notches} notches)"
                )
                _scroll_sidebar_wheel(client_origin, -notches, anchor_test_id=_last_successful_nav_id)
                time.sleep(0.25)
            continue

        if "above window" in reason or "above container" in reason:
            thumb_id = "sidebar_scroll_container_scroll_thumb_v"
            if get_element_bounds(thumb_id):
                print(
                    f"  [SCROLL] '{nav_id}' above viewport (attempt {attempt}/{max_scroll_attempts}); "
                    f"dragging '{thumb_id}' up"
                )
                drag_scrollbar(thumb_id, amount=-160, client_origin=client_origin)
                time.sleep(0.25)
            else:
                notches = 6
                print(
                    f"  [SCROLL] '{nav_id}' above viewport (attempt {attempt}/{max_scroll_attempts}); "
                    f"scrolling sidebar up ({notches} notches)"
                )
                _scroll_sidebar_wheel(client_origin, notches, anchor_test_id=_last_successful_nav_id)
                time.sleep(0.25)
            continue

        # Scroll when blocked horizontally (e.g. compact bottom-nav layout).
        if "right of window" in reason or "right of container" in reason:
            thumb_id = "sidebar_scroll_container_scroll_thumb_h"
            if get_element_bounds(thumb_id):
                print(
                    f"  [SCROLL] '{nav_id}' right of viewport (attempt {attempt}/{max_scroll_attempts}); "
                    f"dragging '{thumb_id}' right"
                )
                drag_scrollbar_horizontal(thumb_id, amount=220, client_origin=client_origin)
                time.sleep(0.25)
            else:
                print(
                    f"  [SCROLL] '{nav_id}' right of viewport (attempt {attempt}/{max_scroll_attempts}); "
                    "no horizontal thumb; trying hscroll"
                )
                focus_bevy_window()
                try:
                    pyautogui.hscroll(-80)
                except Exception:
                    pass
                time.sleep(0.25)
            continue

        if "left of window" in reason or "left of container" in reason:
            thumb_id = "sidebar_scroll_container_scroll_thumb_h"
            if get_element_bounds(thumb_id):
                print(
                    f"  [SCROLL] '{nav_id}' left of viewport (attempt {attempt}/{max_scroll_attempts}); "
                    f"dragging '{thumb_id}' left"
                )
                drag_scrollbar_horizontal(thumb_id, amount=-220, client_origin=client_origin)
                time.sleep(0.25)
            else:
                print(
                    f"  [SCROLL] '{nav_id}' left of viewport (attempt {attempt}/{max_scroll_attempts}); "
                    "no horizontal thumb; trying hscroll"
                )
                focus_bevy_window()
                try:
                    pyautogui.hscroll(80)
                except Exception:
                    pass
                time.sleep(0.25)
            continue

        # If it's blocked for some other reason, don't loop forever.
        print(f"  [BLOCKED] '{nav_id}' click blocked: {reason}")
        return False

    print(f"  [FAIL] Unable to bring '{nav_id}' into view after scrolling")
    return False


def read_telemetry(max_retries: int = 3):
    """Read component telemetry from the app with retry logic for race conditions"""
    if TELEMETRY_FILE.exists():
        for attempt in range(max_retries):
            try:
                with open(TELEMETRY_FILE, 'r') as f:
                    content = f.read()
                    if content.strip():  # Ensure file isn't empty
                        return json.loads(content)
                    # File was empty, wait and retry
                    time.sleep(0.05)
            except json.JSONDecodeError:
                # File was being written to, wait and retry
                time.sleep(0.05)
            except Exception as e:
                print(f"Error reading telemetry: {e}")
                return None
    return None


def wait_for_telemetry_state(key: str, expected_value: str, timeout: float = 2.0) -> bool:
    """Wait until telemetry shows expected state"""
    start = time.time()
    while time.time() - start < timeout:
        telemetry = read_telemetry()
        if telemetry and telemetry.get("states", {}).get(key) == expected_value:
            return True
        time.sleep(0.1)
    return False


def verify_telemetry_state(key: str, expected_value: str, wait_first: bool = True) -> dict:
    """Verify a telemetry state matches expected value, with optional wait"""
    if wait_first:
        # Wait briefly for the state to settle (up to 1 second)
        wait_for_telemetry_state(key, expected_value, timeout=1.0)
    
    telemetry = read_telemetry()
    actual = telemetry.get("states", {}).get(key) if telemetry else None
    passed = actual == expected_value
    result = {
        "key": key,
        "expected": expected_value,
        "actual": actual,
        "passed": passed,
        "message": f"{'[PASS]' if passed else '[FAIL]'} {key}: expected '{expected_value}', got '{actual}'"
    }

    if FAIL_FAST and not passed:
        raise TestFailure(result["message"])

    return result


def navigate_and_verify(section_name: str, nav_id: str, click_pos, retries: int = 3, settle: float = 0.8) -> dict:
    """Click a sidebar nav item and verify selected_section, retrying if needed."""
    global _last_successful_nav_id
    last = None
    for attempt in range(1, retries + 1):
        print(f"  Navigate attempt {attempt}/{retries}: {nav_id} -> {section_name}")
        if nav_id.startswith("nav_"):
            click_nav_element_with_auto_scroll(nav_id, click_pos)
        else:
            click_element(nav_id, click_pos)
        time.sleep(settle)
        last = verify_telemetry_state("selected_section", section_name)
        print(f"  {last['message']}")
        if last["passed"]:
            _last_successful_nav_id = nav_id
            return last
        time.sleep(0.2)

    result = last if last else {
        "key": "selected_section",
        "expected": section_name,
        "actual": None,
        "passed": False,
        "message": f"[FAIL] selected_section: expected '{section_name}', got None",
    }

    if FAIL_FAST and not result.get("passed", False):
        raise TestFailure(result["message"])

    return result


def require_click_element(test_id: str, click_pos) -> None:
    """Click an element; fail fast if missing/click blocked."""
    if click_element(test_id, click_pos):
        return
    # If the showcase detail panel is scrollable, try to bring the element into view.
    if click_main_element_with_auto_scroll(test_id, click_pos):
        return
    raise TestFailure(f"[FAIL] Required element '{test_id}' not found/click failed")


def _scroll_main_wheel(client_origin, ticks: int, anchor_test_id: str | None = None) -> None:
    """Scroll the main content area using the mouse wheel."""
    if not client_origin:
        return

    anchor = get_element_bounds(anchor_test_id) if anchor_test_id else None
    if anchor:
        x = client_origin[0] + anchor["x"] + anchor["width"] / 2
        y = client_origin[1] + anchor["y"] + anchor["height"] / 2
    else:
        container = get_element_bounds("main_scroll_container")
        if not container:
            return
        x = client_origin[0] + container["x"] + container["width"] / 2
        y = client_origin[1] + container["y"] + container["height"] / 2

    focus_bevy_window()
    pyautogui.moveTo(x, y, duration=0.1)
    time.sleep(0.05)
    pyautogui.scroll(int(ticks))
    time.sleep(0.25)


def click_main_element_with_auto_scroll(test_id: str, client_origin, max_scroll_attempts: int = 12) -> bool:
    """Click an element in the scrollable detail panel, scrolling if it is clipped."""
    container = get_element_bounds("main_scroll_container")
    if not container:
        return False

    for attempt in range(1, max_scroll_attempts + 1):
        bounds = get_element_bounds(test_id)
        if not bounds:
            return False

        click_x_local = bounds["x"] + bounds["width"] / 2
        click_y_local = bounds["y"] + bounds["height"] / 2
        visibility_reason: str | None = None

        cont_left = container["x"]
        cont_right = container["x"] + container["width"]
        cont_top = container["y"]
        cont_bottom = container["y"] + container["height"]

        elem_left = bounds["x"]
        elem_right = bounds["x"] + bounds["width"]
        elem_top = bounds["y"]
        elem_bottom = bounds["y"] + bounds["height"]

        vis_left = max(elem_left, cont_left)
        vis_right = min(elem_right, cont_right)
        vis_top = max(elem_top, cont_top)
        vis_bottom = min(elem_bottom, cont_bottom)

        if vis_right - vis_left >= 4.0:
            click_x_local = (vis_left + vis_right) / 2.0
        else:
            if elem_right <= cont_left or elem_left < cont_left:
                visibility_reason = "left of container"
            elif elem_left >= cont_right or elem_right > cont_right:
                visibility_reason = "right of container"
            else:
                visibility_reason = "right of container"

        if vis_bottom - vis_top >= 4.0:
            click_y_local = (vis_top + vis_bottom) / 2.0
        else:
            if elem_bottom <= cont_top or elem_top < cont_top:
                visibility_reason = visibility_reason or "above container"
            elif elem_top >= cont_bottom or elem_bottom > cont_bottom:
                visibility_reason = visibility_reason or "below container"
            else:
                visibility_reason = visibility_reason or "below container"

        center_x = client_origin[0] + click_x_local
        center_y = client_origin[1] + click_y_local
        is_valid, bounds_reason = is_click_in_bounds(center_x, center_y)
        if is_valid and visibility_reason is None:
            print(
                f"  Clicking '{test_id}' at ({center_x:.0f}, {center_y:.0f}) "
                f"[bounds: x={bounds['x']:.0f}, y={bounds['y']:.0f}, w={bounds['width']:.0f}, h={bounds['height']:.0f}]"
            )
            return safe_click(center_x, center_y, f"click element '{test_id}'")

        reason = visibility_reason or bounds_reason
        thumb_v = "main_scroll_container_scroll_thumb_v"
        thumb_h = "main_scroll_container_scroll_thumb_h"

        if "right of window" in reason or "right of container" in reason:
            if get_element_bounds(thumb_h):
                print(
                    f"  [SCROLL] '{test_id}' right of viewport (attempt {attempt}/{max_scroll_attempts}); "
                    f"dragging '{thumb_h}' right"
                )
                drag_scrollbar_horizontal(thumb_h, amount=220, client_origin=client_origin)
                time.sleep(0.25)
                continue
            return False

        if "left of window" in reason or "left of container" in reason:
            if get_element_bounds(thumb_h):
                print(
                    f"  [SCROLL] '{test_id}' left of viewport (attempt {attempt}/{max_scroll_attempts}); "
                    f"dragging '{thumb_h}' left"
                )
                drag_scrollbar_horizontal(thumb_h, amount=-220, client_origin=client_origin)
                time.sleep(0.25)
                continue
            return False

        if "below window" in reason or "below container" in reason:
            if get_element_bounds(thumb_v):
                print(
                    f"  [SCROLL] '{test_id}' below viewport (attempt {attempt}/{max_scroll_attempts}); "
                    f"dragging '{thumb_v}' down"
                )
                drag_scrollbar(thumb_v, amount=200, client_origin=client_origin)
            else:
                notches = 8
                print(
                    f"  [SCROLL] '{test_id}' below viewport (attempt {attempt}/{max_scroll_attempts}); "
                    f"scrolling main content down ({notches} notches)"
                )
                _scroll_main_wheel(client_origin, -notches, anchor_test_id=test_id)
            time.sleep(0.25)
            continue

        if "above window" in reason or "above container" in reason:
            if get_element_bounds(thumb_v):
                print(
                    f"  [SCROLL] '{test_id}' above viewport (attempt {attempt}/{max_scroll_attempts}); "
                    f"dragging '{thumb_v}' up"
                )
                drag_scrollbar(thumb_v, amount=-200, client_origin=client_origin)
            else:
                notches = 8
                print(
                    f"  [SCROLL] '{test_id}' above viewport (attempt {attempt}/{max_scroll_attempts}); "
                    f"scrolling main content up ({notches} notches)"
                )
                _scroll_main_wheel(client_origin, notches, anchor_test_id=test_id)
            time.sleep(0.25)
            continue

        return False

    return False


def require_drag_element(test_id: str, delta_x: float, delta_y: float, click_pos, duration: float = 0.3) -> None:
    """Drag an element; fail fast if missing/drag blocked."""
    if not drag_element(test_id, delta_x, delta_y, click_pos, duration=duration):
        raise TestFailure(f"[FAIL] Required element '{test_id}' not found/drag failed")


def get_element_bounds(test_id: str) -> dict:
    """Get element bounds by test_id from telemetry"""
    telemetry = read_telemetry()
    if not telemetry:
        return None
    elements = telemetry.get("elements", [])
    for elem in elements:
        if elem.get("test_id") == test_id:
            return elem
    return None


def _element_center_screen(test_id: str, click_pos) -> tuple[float, float] | None:
    bounds = get_element_bounds(test_id)
    if not bounds:
        return None

    # Keep coordinate math consistent with click_element().
    if _client_origin:
        win_x, win_y = _client_origin
    elif click_pos:
        win_x, win_y = click_pos[0], click_pos[1]
    else:
        win_x, win_y = 0, 0

    return (
        win_x + bounds["x"] + bounds["width"] / 2,
        win_y + bounds["y"] + bounds["height"] / 2,
    )


def is_element_clickable(test_id: str, click_pos) -> bool:
    center = _element_center_screen(test_id, click_pos)
    if not center:
        return False
    valid, _ = is_click_in_bounds(center[0], center[1])
    return bool(valid)


def click_element(test_id: str, window_rect=None) -> bool:
    """Click an element by its test_id using bounds from telemetry.
    Uses safe_click to prevent clicking outside the application."""
    bounds = get_element_bounds(test_id)
    if not bounds:
        print(f"  [MISS] Element '{test_id}' not found in telemetry")
        return False
    
    # Get window offset - prefer client origin if available, fallback to window_rect
    if _client_origin:
        win_x, win_y = _client_origin
    elif window_rect:
        win_x, win_y = window_rect[0], window_rect[1]
    else:
        win_x, win_y = 0, 0
    
    # Calculate center of element (bounds are already in screen coordinates)
    center_x = win_x + bounds["x"] + bounds["width"] / 2
    center_y = win_y + bounds["y"] + bounds["height"] / 2
    
    print(f"  Clicking '{test_id}' at ({center_x:.0f}, {center_y:.0f}) [bounds: x={bounds['x']:.0f}, y={bounds['y']:.0f}, w={bounds['width']:.0f}, h={bounds['height']:.0f}]")
    
    # Use safe_click to prevent clicking outside window
    if not safe_click(center_x, center_y, f"click element '{test_id}'"):
        return False
    
    time.sleep(0.3)
    return True


def drag_element(test_id: str, delta_x: float, delta_y: float, window_rect=None, duration=0.3) -> bool:
    """Drag an element by its test_id with given delta.
    Validates both start and end positions are within window bounds."""
    bounds = get_element_bounds(test_id)
    if not bounds:
        print(f"  [MISS] Element '{test_id}' not found in telemetry")
        return False
    
    # Use client origin if available, fallback to window_rect
    if _client_origin:
        win_x, win_y = _client_origin
    elif window_rect:
        win_x, win_y = window_rect[0], window_rect[1]
    else:
        win_x, win_y = 0, 0
    
    start_x = win_x + bounds["x"] + bounds["width"] / 2
    start_y = win_y + bounds["y"] + bounds["height"] / 2
    end_x = start_x + delta_x
    end_y = start_y + delta_y
    
    # Validate both start and end are in bounds
    start_valid, start_reason = is_click_in_bounds(start_x, start_y)
    end_valid, end_reason = is_click_in_bounds(end_x, end_y)
    
    if not start_valid:
        print(f"  [BLOCKED] Drag start at ({start_x:.0f}, {start_y:.0f}) outside application: {start_reason}\")")
        return False
    if not end_valid:
        print(f"  [BLOCKED] Drag end at ({end_x:.0f}, {end_y:.0f}) would be outside application: {end_reason}\")")
        return False
    
    print(f"  Dragging '{test_id}' from ({start_x:.0f}, {start_y:.0f}) by ({delta_x:.0f}, {delta_y:.0f})")
    focus_bevy_window()
    pyautogui.moveTo(start_x, start_y)
    pyautogui.drag(delta_x, delta_y, duration=duration)
    time.sleep(0.3)
    return True


def drag_scrollbar(scrollbar_id: str, amount: float, client_origin=None) -> bool:
    """Drag a scrollbar thumb by a given amount (positive = down, negative = up).
    
    Args:
        scrollbar_id: TestId of the scrollbar thumb (e.g., 'sidebar_scroll_thumb', 'main_scroll_thumb')
        amount: Pixels to drag (positive = scroll down, negative = scroll up)
        client_origin: Client origin from find_bevy_window
    
    Returns:
        True if drag was performed, False if blocked or element not found
    """
    bounds = get_element_bounds(scrollbar_id)
    if not bounds:
        print(f"  [MISS] Scrollbar '{scrollbar_id}' not found in telemetry")
        return False
    
    # Use client origin if available
    if client_origin and _client_origin:
        win_x, win_y = _client_origin
    elif client_origin:
        win_x, win_y = client_origin
    else:
        win_x, win_y = 0, 0
    
    # Calculate center of scrollbar thumb
    center_x = win_x + bounds["x"] + bounds["width"] / 2
    center_y = win_y + bounds["y"] + bounds["height"] / 2
    
    # Calculate end position
    end_y = center_y + amount
    
    # Validate both positions are in bounds
    start_valid, start_reason = is_click_in_bounds(center_x, center_y)
    end_valid, end_reason = is_click_in_bounds(center_x, end_y)
    
    if not start_valid:
        print(f"  [BLOCKED] Scrollbar at ({center_x:.0f}, {center_y:.0f}) outside window: {start_reason}")
        return False
    if not end_valid:
        # Clamp to window bounds instead of blocking
        if _window_bounds:
            end_y = max(_window_bounds[1] + 10, min(_window_bounds[3] - 10, end_y))
            print(f"  [CLAMPED] Scrollbar drag end clamped to ({center_x:.0f}, {end_y:.0f})")
    
    print(f"  Dragging scrollbar '{scrollbar_id}' from y={center_y:.0f} to y={end_y:.0f} (delta={amount:.0f})")
    
    pyautogui.moveTo(center_x, center_y)
    time.sleep(0.1)
    pyautogui.mouseDown()
    time.sleep(0.05)
    pyautogui.moveTo(center_x, end_y, duration=0.3)
    pyautogui.mouseUp()
    time.sleep(0.3)
    
    return True


def drag_scrollbar_horizontal(scrollbar_id: str, amount: float, client_origin=None) -> bool:
    """Drag a horizontal scrollbar thumb by a given amount (positive = right, negative = left)."""
    bounds = get_element_bounds(scrollbar_id)
    if not bounds:
        print(f"  [MISS] Scrollbar '{scrollbar_id}' not found in telemetry")
        return False

    if client_origin and _client_origin:
        win_x, win_y = _client_origin
    elif client_origin:
        win_x, win_y = client_origin
    else:
        win_x, win_y = 0, 0

    center_x = win_x + bounds["x"] + bounds["width"] / 2
    center_y = win_y + bounds["y"] + bounds["height"] / 2

    end_x = center_x + amount

    start_valid, start_reason = is_click_in_bounds(center_x, center_y)
    end_valid, _end_reason = is_click_in_bounds(end_x, center_y)

    if not start_valid:
        print(f"  [BLOCKED] Scrollbar at ({center_x:.0f}, {center_y:.0f}) outside window: {start_reason}")
        return False

    if not end_valid and _window_bounds:
        end_x = max(_window_bounds[0] + 10, min(_window_bounds[2] - 10, end_x))
        print(f"  [CLAMPED] Scrollbar drag end clamped to ({end_x:.0f}, {center_y:.0f})")

    print(
        f"  Dragging scrollbar '{scrollbar_id}' from x={center_x:.0f} to x={end_x:.0f} (delta={amount:.0f})"
    )

    pyautogui.moveTo(center_x, center_y)
    time.sleep(0.1)
    pyautogui.mouseDown()
    time.sleep(0.05)
    pyautogui.moveTo(end_x, center_y, duration=0.3)
    pyautogui.mouseUp()
    time.sleep(0.3)

    return True


def get_scroll_position(scroll_key: str = "sidebar_scroll_y") -> float:
    """Get current scroll position from telemetry.
    
    Args:
        scroll_key: Telemetry key - 'sidebar_scroll_y' or 'main_scroll_y'
    
    Returns:
        Scroll position in pixels, or 0.0 if not found
    """
    telemetry = read_telemetry()
    if telemetry:
        value = telemetry.get("states", {}).get(scroll_key, "0.0")
        try:
            return float(value)
        except (ValueError, TypeError):
            return 0.0
    return 0.0


def list_available_elements():
    """Print all available elements with test_ids"""
    telemetry = read_telemetry()
    if not telemetry:
        print("No telemetry available")
        return
    elements = telemetry.get("elements", [])
    print(f"\nAvailable elements ({len(elements)} total):")
    for elem in sorted(elements, key=lambda e: e.get("test_id", "")):
        print(f"  - {elem.get('test_id')}: ({elem.get('x'):.0f}, {elem.get('y'):.0f}) {elem.get('width'):.0f}x{elem.get('height'):.0f}")


def find_bevy_window(
    maximize: bool = True,
    client_size: tuple[int, int] | None = None,
    pid: int | None = None,
    process_name: str | None = None,
):
    """Find, focus, and optionally maximize/resize the Bevy window.

    If pid is provided, only consider windows owned by that process.
    Returns (client_origin, window_rect).
    """
    global _bevy_hwnd
    try:
        import ctypes
        import ctypes.wintypes
        user32 = ctypes.windll.user32
        
        # Constants for ShowWindow
        SW_MAXIMIZE = 3
        SW_RESTORE = 9
        SW_SHOWNORMAL = 1
        
        # Get all window handles
        def get_windows():
            windows = []
            def callback(hwnd, _):
                if user32.IsWindowVisible(hwnd):
                    length = user32.GetWindowTextLengthW(hwnd) + 1
                    buffer = ctypes.create_unicode_buffer(length)
                    user32.GetWindowTextW(hwnd, buffer, length)
                    # Keep empty titles too; PID filtering will disambiguate.
                    windows.append((hwnd, buffer.value))
                return True
            
            WNDENUMPROC = ctypes.WINFUNCTYPE(ctypes.c_bool, ctypes.wintypes.HWND, ctypes.wintypes.LPARAM)
            user32.EnumWindows(WNDENUMPROC(callback), 0)
            return windows
        
        def get_process_basename(process_id: int) -> str | None:
            try:
                PROCESS_QUERY_LIMITED_INFORMATION = 0x1000
                kernel32 = ctypes.windll.kernel32
                handle = kernel32.OpenProcess(PROCESS_QUERY_LIMITED_INFORMATION, False, int(process_id))
                if not handle:
                    return None
                try:
                    # QueryFullProcessImageNameW is in kernel32
                    size = ctypes.wintypes.DWORD(260)
                    buf = ctypes.create_unicode_buffer(size.value)
                    if kernel32.QueryFullProcessImageNameW(handle, 0, buf, ctypes.byref(size)):
                        path = buf.value
                        return path.split("\\")[-1].lower() if path else None
                finally:
                    kernel32.CloseHandle(handle)
            except Exception:
                return None
            return None

        windows = get_windows()
        debug_candidates: list[tuple[int, str, int | None, str | None]] = []
        for hwnd, title in windows:
            owner_pid_val: int | None = None
            owner_exe: str | None = None

            # Resolve owner pid/exe for filtering
            try:
                owner_pid = ctypes.wintypes.DWORD(0)
                user32.GetWindowThreadProcessId(hwnd, ctypes.byref(owner_pid))
                owner_pid_val = int(owner_pid.value)
                owner_exe = get_process_basename(owner_pid_val)
            except Exception:
                owner_pid_val = None
                owner_exe = None

            # Hard exclude common false positives
            if owner_exe in {"code.exe", "powershell.exe", "python.exe"}:
                continue

            # Prefer a strong PID match when available.
            if pid is not None:
                try:
                    if owner_pid_val is None or int(owner_pid_val) != int(pid):
                        continue
                except Exception:
                    continue

                if not title:
                    title = f"<pid {pid} window>"
                print(f"Found Bevy window: {title}")
                _bevy_hwnd = hwnd

                # Restore only if minimized; restoring an already-maximized window can
                # sometimes flip it back to normal size on some Windows setups.
                try:
                    if user32.IsIconic(hwnd):
                        user32.ShowWindow(hwnd, SW_RESTORE)
                        time.sleep(0.2)
                except Exception:
                    user32.ShowWindow(hwnd, SW_RESTORE)
                    time.sleep(0.2)

                # Size policy
                if client_size is not None:
                    user32.ShowWindow(hwnd, SW_SHOWNORMAL)
                    print(f"  Window: RESIZE client={client_size[0]}x{client_size[1]}")
                elif maximize:
                    user32.ShowWindow(hwnd, SW_MAXIMIZE)
                    print("  Window: MAXIMIZED")
                else:
                    user32.ShowWindow(hwnd, SW_SHOWNORMAL)
                    print("  Window: NORMAL")
                time.sleep(0.3)

                # Focus window
                user32.SetForegroundWindow(hwnd)
                time.sleep(0.3)

                # Get window rect
                class RECT(ctypes.Structure):
                    _fields_ = [("left", ctypes.c_long), ("top", ctypes.c_long),
                               ("right", ctypes.c_long), ("bottom", ctypes.c_long)]
                rect = RECT()
                user32.GetWindowRect(hwnd, ctypes.byref(rect))

                # Get client area position for accurate clicking
                client_point = ctypes.wintypes.POINT(0, 0)
                user32.ClientToScreen(hwnd, ctypes.byref(client_point))

                window_rect = (rect.left, rect.top, rect.right, rect.bottom)
                client_origin = (client_point.x, client_point.y)

                print(f"  Window rect: {window_rect}")
                print(f"  Client origin (for element clicks): {client_origin}")

                set_window_bounds(window_rect, client_origin)

                if client_size is not None:
                    set_bevy_window_client_size(client_size[0], client_size[1])
                    return _client_origin, _window_bounds

                return client_origin, window_rect

            # Next best: match by owning executable name. When we request
            # process_name filtering, accept any visible top-level window for it.
            if process_name is not None:
                debug_candidates.append((int(hwnd), title or "", owner_pid_val, owner_exe))

                if owner_exe != process_name.lower():
                    continue

                if not title:
                    title = f"<{process_name} window>"
                print(f"Found Bevy window: {title}")
                _bevy_hwnd = hwnd

                # Restore only if minimized; restoring an already-maximized window can
                # sometimes flip it back to normal size on some Windows setups.
                try:
                    if user32.IsIconic(hwnd):
                        user32.ShowWindow(hwnd, SW_RESTORE)
                        time.sleep(0.2)
                except Exception:
                    user32.ShowWindow(hwnd, SW_RESTORE)
                    time.sleep(0.2)

                # Size policy
                if client_size is not None:
                    user32.ShowWindow(hwnd, SW_SHOWNORMAL)
                    print(f"  Window: RESIZE client={client_size[0]}x{client_size[1]}")
                elif maximize:
                    user32.ShowWindow(hwnd, SW_MAXIMIZE)
                    print("  Window: MAXIMIZED")
                else:
                    user32.ShowWindow(hwnd, SW_SHOWNORMAL)
                    print("  Window: NORMAL")
                time.sleep(0.3)

                # Focus window
                user32.SetForegroundWindow(hwnd)
                time.sleep(0.3)

                # Get window rect
                class RECT(ctypes.Structure):
                    _fields_ = [("left", ctypes.c_long), ("top", ctypes.c_long),
                               ("right", ctypes.c_long), ("bottom", ctypes.c_long)]
                rect = RECT()
                user32.GetWindowRect(hwnd, ctypes.byref(rect))

                # Get client area position for accurate clicking
                client_point = ctypes.wintypes.POINT(0, 0)
                user32.ClientToScreen(hwnd, ctypes.byref(client_point))

                window_rect = (rect.left, rect.top, rect.right, rect.bottom)
                client_origin = (client_point.x, client_point.y)

                print(f"  Window rect: {window_rect}")
                print(f"  Client origin (for element clicks): {client_origin}")

                set_window_bounds(window_rect, client_origin)

                if client_size is not None:
                    set_bevy_window_client_size(client_size[0], client_size[1])
                    return _client_origin, _window_bounds

                return client_origin, window_rect

            title_l = title.lower() if title else ""
            if (
                "material design 3" in (title or "")
                or "material ui" in title_l
                or ("showcase" in title_l and "visual studio code" not in title_l)
                or "bevy app" in title_l
            ):
                print(f"Found Bevy window: {title}")
                _bevy_hwnd = hwnd
                
                # Restore only if minimized; restoring an already-maximized window can
                # sometimes flip it back to normal size on some Windows setups.
                try:
                    if user32.IsIconic(hwnd):
                        user32.ShowWindow(hwnd, SW_RESTORE)
                        time.sleep(0.2)
                except Exception:
                    user32.ShowWindow(hwnd, SW_RESTORE)
                    time.sleep(0.2)
                
                # Size policy
                if client_size is not None:
                    user32.ShowWindow(hwnd, SW_SHOWNORMAL)
                    print(f"  Window: RESIZE client={client_size[0]}x{client_size[1]}")
                elif maximize:
                    user32.ShowWindow(hwnd, SW_MAXIMIZE)
                    print("  Window: MAXIMIZED")
                else:
                    user32.ShowWindow(hwnd, SW_SHOWNORMAL)
                    print("  Window: NORMAL")
                time.sleep(0.3)
                
                # Focus window
                user32.SetForegroundWindow(hwnd)
                time.sleep(0.3)
                
                # Get window rect
                class RECT(ctypes.Structure):
                    _fields_ = [("left", ctypes.c_long), ("top", ctypes.c_long),
                               ("right", ctypes.c_long), ("bottom", ctypes.c_long)]
                rect = RECT()
                user32.GetWindowRect(hwnd, ctypes.byref(rect))
                
                # Get client area position for accurate clicking
                client_point = ctypes.wintypes.POINT(0, 0)
                user32.ClientToScreen(hwnd, ctypes.byref(client_point))
                
                window_rect = (rect.left, rect.top, rect.right, rect.bottom)
                client_origin = (client_point.x, client_point.y)
                
                print(f"  Window rect: {window_rect}")
                print(f"  Client origin (for element clicks): {client_origin}")
                
                # Set global window bounds for click validation
                set_window_bounds(window_rect, client_origin)

                # If requested, resize after initial discovery (keeps title matching logic simple)
                if client_size is not None:
                    set_bevy_window_client_size(client_size[0], client_size[1])
                    # Refresh return values from globals
                    return _client_origin, _window_bounds
                
                return client_origin, window_rect
    except Exception as e:
        print(f"Window detection error: {e}")

    # Diagnostics for hard-to-find windows
    try:
        if process_name is not None:
            wanted = process_name.lower()
            matches = [c for c in debug_candidates if c[3] == wanted]
            if matches:
                print(f"Window discovery: found {len(matches)} hwnd(s) for {wanted} but none were accepted")
                for hwnd, title, owner_pid_val, owner_exe in matches[:10]:
                    print(f"  candidate hwnd={hwnd} pid={owner_pid_val} title='{title}' exe={owner_exe}")
            else:
                # Show a few candidates to help identify the real exe/title.
                print(f"Window discovery: no hwnds matched exe '{wanted}'. Sample visible windows (exe/title):")
                for hwnd, title, owner_pid_val, owner_exe in debug_candidates[:15]:
                    print(f"  hwnd={hwnd} pid={owner_pid_val} exe={owner_exe} title='{title}'")
    except Exception:
        pass

    return None, None


def wait_for_element(test_id: str, timeout: float = 1.2) -> bool:
    start = time.time()
    while time.time() - start < timeout:
        if get_element_bounds(test_id):
            return True
        time.sleep(0.05)
    return False


def require_element_present(test_id: str, timeout: float = 1.2) -> None:
    if not wait_for_element(test_id, timeout=timeout):
        raise TestFailure(f"[FAIL] Required element '{test_id}' not present in telemetry")


def require_layout_telemetry() -> None:
    """Sanity-check that core layout regions are present in telemetry.

    This validates that layout components (e.g. Scaffold regions) expose stable
    `TestId`s for automation.
    """
    # Scaffold regions
    for tid in ("scaffold_root", "scaffold_navigation", "scaffold_content"):
        require_element_present(tid, timeout=2.0)
        b = get_element_bounds(tid)
        if not b:
            raise TestFailure(f"[FAIL] Layout element '{tid}' missing bounds")
        if b.get("width", 0) < 8 or b.get("height", 0) < 8:
            raise TestFailure(
                f"[FAIL] Layout element '{tid}' has invalid size: w={b.get('width')}, h={b.get('height')}"
            )

    # The main detail scroller is a key reachability primitive.
    require_element_present("main_scroll_container", timeout=2.0)


def wait_for_bevy_window(
    timeout: float = 15.0,
    maximize: bool = False,
    client_size: tuple[int, int] | None = None,
    pid: int | None = None,
    process_name: str | None = "showcase.exe",
) -> tuple[tuple[int, int] | None, tuple[int, int, int, int] | None]:
    """Wait until the showcase window is discoverable."""
    start = time.time()
    while time.time() - start < timeout:
        # Try an exact PID match first; fall back to exe-name filtering.
        result = find_bevy_window(maximize=maximize, client_size=client_size, pid=pid)
        if not result or not result[0]:
            result = find_bevy_window(maximize=maximize, client_size=client_size, process_name=process_name)
        if result and result[0]:
            return result
        time.sleep(0.25)

    return None, None


SECTION_SMOKE_REQUIREMENTS: list[tuple[str, str, list[str]]] = [
    ("Buttons", "nav_buttons", ["button_0"]),
    ("Checkboxes", "nav_checkboxes", ["checkbox_0"]),
    ("Switches", "nav_switches", ["switch_0"]),
    ("RadioButtons", "nav_radio_buttons", ["radio_0"]),
    ("Chips", "nav_chips", ["chip_0"]),
    ("FAB", "nav_fab", ["fab_0"]),
    ("Badges", "nav_badges", ["badge_0"]),
    ("Progress", "nav_progress", ["progress_linear_0"]),
    ("Cards", "nav_cards", ["card_0"]),
    ("Dividers", "nav_dividers", ["divider_0"]),
    ("Lists", "nav_lists", ["list_scroll_area", "list_item_0"]),
    ("Icons", "nav_icons", ["icon_0"]),
    ("IconButtons", "nav_icon_buttons", ["icon_button_0"]),
    ("Sliders", "nav_sliders", ["slider_thumb_0"]),
    ("TextFields", "nav_text_fields", ["text_field_0"]),
    ("Dialogs", "nav_dialogs", ["dialog_open_0"]),
    ("Menus", "nav_menus", ["menu_trigger_0"]),
    ("Tabs", "nav_tabs", ["tabs_primary"]),
    ("Select", "nav_select", ["select_0"]),
    ("Snackbar", "nav_snackbar", ["snackbar_trigger_0"]),
    ("Tooltips", "nav_tooltips", ["tooltip_demo_0"]),
    ("AppBar", "nav_app_bar", ["app_bar_icon_0"]),
    ("Layouts", "nav_layouts", ["layout_bottom_content", "layout_list_primary"]),
    ("ThemeColors", "nav_theme_colors", ["theme_mode_dark", "theme_seed_purple"]),
]


def smoke_interactions(client_origin) -> None:
    """Minimal interaction smoke: verify key inputs still respond."""
    # Checkboxes
    navigate_and_verify("Checkboxes", "nav_checkboxes", client_origin, retries=3, settle=0.8)
    require_click_element("checkbox_0", client_origin)
    time.sleep(0.3)

    # Sliders
    navigate_and_verify("Sliders", "nav_sliders", client_origin, retries=3, settle=0.8)
    slider_bounds = get_element_bounds("slider_thumb_0")
    if slider_bounds:
        win_x, win_y = (_client_origin if _client_origin else client_origin)
        start_x = win_x + slider_bounds["x"] + slider_bounds["width"] / 2
        start_y = win_y + slider_bounds["y"] + slider_bounds["height"] / 2
        start_valid, start_reason = is_click_in_bounds(start_x, start_y)
        if not start_valid:
            print(f"  [SKIP] slider drag not interactable at this size: {start_reason}")
        else:
            # Adapt drag distance to available horizontal room.
            if _window_bounds is not None:
                left, _top, right, _bottom = _window_bounds
            else:
                left, right = 0, start_x + 200

            margin = 10
            max_right = (right - margin) - start_x
            max_left = (left + margin) - start_x

            delta_x = 120
            # Prefer dragging right; if not enough room, drag left.
            if delta_x > max_right:
                delta_x = max_right
            if delta_x < 20:
                delta_x = -80
                if delta_x < max_left:
                    delta_x = max_left

            if abs(delta_x) < 20:
                print("  [SKIP] slider drag not interactable at this size (insufficient room)")
            else:
                require_drag_element("slider_thumb_0", float(delta_x), 0, client_origin, duration=0.25)
    else:
        raise TestFailure("[FAIL] Required element 'slider_thumb_0' not present in telemetry")
    time.sleep(0.3)

    # Tabs
    navigate_and_verify("Tabs", "nav_tabs", client_origin, retries=3, settle=0.9)
    clicked_tab = False
    for tab_id in ("tab_2", "tab_1", "tab_0"):
        if click_main_element_with_auto_scroll(tab_id, client_origin) or click_element(tab_id, client_origin):
            clicked_tab = True
            break

    if not clicked_tab:
        print("  [SKIP] tabs click not interactable at this size")
    time.sleep(0.3)

    # Lists
    navigate_and_verify("Lists", "nav_lists", client_origin, retries=4, settle=1.1)
    clicked_item = False
    for item_id in ("list_item_2", "list_item_1", "list_item_0"):
        if click_main_element_with_auto_scroll(item_id, client_origin) or click_element(item_id, client_origin):
            clicked_item = True
            break

    if not clicked_item:
        print("  [SKIP] list item click not interactable at this size")
    time.sleep(0.3)

    # ThemeColors (reactivity check)
    navigate_and_verify("ThemeColors", "nav_theme_colors", client_origin, retries=4, settle=1.1)
    # Try to click whichever mode buttons are reachable (auto-scroll horizontally if needed).
    clickable_modes: list[str] = []
    for mode_id in ("theme_mode_light", "theme_mode_dark"):
        if click_main_element_with_auto_scroll(mode_id, client_origin) or click_element(mode_id, client_origin):
            clickable_modes.append(mode_id)

    if not clickable_modes:
        print("  [SKIP] theme mode toggle not interactable at this size")
        return

    telemetry_before = read_telemetry()
    base_events = telemetry_before.get("events", []) if telemetry_before else []
    base_len = len(base_events)

    mode_changed = False
    for mode_id in clickable_modes:
        # Already clicked above to establish reachability; click again to attempt a toggle.
        if not (click_main_element_with_auto_scroll(mode_id, client_origin) or click_element(mode_id, client_origin)):
            continue
        time.sleep(0.4)
        telemetry_after = read_telemetry()
        events_after = telemetry_after.get("events", []) if telemetry_after else []
        if any("Theme: mode changed" in e for e in events_after[base_len:]):
            mode_changed = True
            break
        base_len = len(events_after)

    if not mode_changed:
        if len(clickable_modes) >= 2:
            raise TestFailure("[FAIL] ThemeColors: expected a theme mode change event")
        print("  [SKIP] theme mode could not toggle (only one option clickable)")


def run_size_matrix(
    component: str,
    sizes: list[WindowSize],
    *,
    start_from: str | None = None,
    only: list[str] | None = None,
    resume: bool = False,
    resume_file: Path = LAST_FAILURE_FILE,
    resume_all_sizes: bool = False,
) -> None:
    """Run nav+smoke checks across multiple window sizes."""
    global _showcase_log_handles
    print("=" * 60)
    print("RESPONSIVE SIZE MATRIX")
    print("=" * 60)

    # Delete old telemetry file
    resume_payload = load_last_failure(resume_file) if resume else None
    if resume_payload:
        start_from = start_from or resume_payload.get("section") or resume_payload.get("nav_id")
        sizes = _filter_sizes_for_resume(
            sizes,
            resume_payload.get("size"),
            resume_all_sizes,
        )

    # Delete old telemetry file
    if TELEMETRY_FILE.exists():
        TELEMETRY_FILE.unlink()

    import os
    env = os.environ.copy()
    env['BEVY_TELEMETRY'] = '1'

    print("\nStarting Bevy showcase...")
    proc = launch_showcase(env)

    # Wait for app to start
    print("  Waiting for application to start...")
    max_wait = 120
    waited = 0
    while waited < max_wait:
        if TELEMETRY_FILE.exists():
            try:
                with open(TELEMETRY_FILE, 'r') as f:
                    content = f.read()
                    if content.strip() and '"elements"' in content:
                        print(f"  Application started after {waited}s")
                        break
            except Exception:
                pass
        if proc.poll() is not None:
            print("\n  ERROR: Application failed to start!")
            if _last_showcase_stderr_log is not None:
                print(f"  stderr log: {_last_showcase_stderr_log}")
                tail = _tail_text_file(_last_showcase_stderr_log, max_bytes=3000)
                if tail.strip():
                    print("\n  --- stderr tail ---")
                    print(tail)
            if _last_showcase_stdout_log is not None:
                print(f"  stdout log: {_last_showcase_stdout_log}")
            sys.exit(1)
        time.sleep(1)
        waited += 1

    if waited >= max_wait:
        print("  ERROR: Timeout")
        proc.terminate()
        sys.exit(1)

    time.sleep(2)

    # Discover the window once (it can appear slightly after telemetry is ready)
    result = wait_for_bevy_window(timeout=20.0, maximize=False, pid=proc.pid)
    if not result or not result[0]:
        print("Could not find Bevy window!")
        if _last_showcase_stdout_log is not None:
            print(f"  stdout log: {_last_showcase_stdout_log}")
        if _last_showcase_stderr_log is not None:
            print(f"  stderr log: {_last_showcase_stderr_log}")
        proc.terminate()
        sys.exit(1)

    try:
        for size in sizes:
            print("\n" + "-" * 60)
            print(f"SIZE: {size.name} ({size.width}x{size.height})")
            print("-" * 60)

            if not set_bevy_window_client_size(size.width, size.height):
                raise TestFailure(f"[FAIL] Could not resize Bevy window to {size.width}x{size.height}")

            client_origin = _client_origin
            if client_origin is None:
                raise TestFailure("[FAIL] Missing client origin after resize")

            try:
                require_layout_telemetry()
            except TestFailure as e:
                save_last_failure(
                    {
                        "mode": "matrix",
                        "size": size.name,
                        "section": "layout_telemetry",
                        "step": "require_layout_telemetry",
                        "message": str(e),
                    },
                    path=resume_file,
                )
                raise

            # Verify required content per section (covers 'all components' at a basic level)
            scroll_sidebar_to_top(client_origin)
            reset_sidebar_scroll()
            time.sleep(0.4)

            sections_to_run = _filter_sections_for_run(SECTION_SMOKE_REQUIREMENTS, start_from, only)

            for section_name, nav_id, required_ids in sections_to_run:
                try:
                    navigate_and_verify(section_name, nav_id, client_origin, retries=5, settle=1.0)
                except TestFailure as e:
                    save_last_failure(
                        {
                            "mode": "matrix",
                            "size": size.name,
                            "section": section_name,
                            "nav_id": nav_id,
                            "step": "navigate",
                            "message": str(e),
                        },
                        path=resume_file,
                    )
                    raise

                for rid in required_ids:
                    try:
                        require_element_present(rid, timeout=1.5)
                    except TestFailure as e:
                        save_last_failure(
                            {
                                "mode": "matrix",
                                "size": size.name,
                                "section": section_name,
                                "nav_id": nav_id,
                                "step": "require_element_present",
                                "required_id": rid,
                                "message": str(e),
                            },
                            path=resume_file,
                        )
                        raise

            try:
                smoke_interactions(client_origin)
            except TestFailure as e:
                save_last_failure(
                    {
                        "mode": "matrix",
                        "size": size.name,
                        "section": "smoke_interactions",
                        "step": "smoke_interactions",
                        "message": str(e),
                    },
                    path=resume_file,
                )
                raise

            print("  [PASS] Size matrix iteration")

        # If we got here, the run fully succeeded.
        clear_last_failure(resume_file)
    finally:
        proc.terminate()
        try:
            proc.wait(timeout=5)
        except Exception:
            pass
        for h in _showcase_log_handles:
            try:
                h.close()
            except Exception:
                pass
        _showcase_log_handles = []


def capture(name: str, rect=None, check_baseline: bool = False):
    """Capture screenshot, optionally comparing to baseline"""
    path = OUTPUT_DIR / f"{name}_{datetime.now().strftime('%H%M%S')}.png"
    if rect:
        img = ImageGrab.grab(bbox=rect)
    else:
        img = ImageGrab.grab()
    img.save(path)
    
    # Visual regression check - use higher threshold since window position varies
    if check_baseline:
        result = compare_with_baseline(name, img, threshold=0.40)  # 40% threshold for position variance
        visual_results.append(result)
        print(f"  Visual: {result['message']}")
    
    return path


def click_relative(rect, rel_x, rel_y):
    """Click at relative position within window.
    Uses safe_click to prevent clicking outside the application."""
    if not rect:
        return False
    x = rect[0] + int(rel_x * (rect[2] - rect[0]))
    y = rect[1] + int(rel_y * (rect[3] - rect[1]))
    
    if not safe_click(x, y, f"click at relative ({rel_x:.2f}, {rel_y:.2f})"):
        return False
    
    time.sleep(0.3)
    return True


def drag_relative(rect, start, end, duration=0.3):
    """Drag from start to end (relative coords).
    Validates both start and end positions are within window bounds."""
    if not rect:
        return False
    w = rect[2] - rect[0]
    h = rect[3] - rect[1]
    
    start_x = rect[0] + int(start[0] * w)
    start_y = rect[1] + int(start[1] * h)
    end_x = rect[0] + int(end[0] * w)
    end_y = rect[1] + int(end[1] * h)
    
    # Validate both start and end are in bounds
    start_valid, start_reason = is_click_in_bounds(start_x, start_y)
    end_valid, end_reason = is_click_in_bounds(end_x, end_y)
    
    if not start_valid:
        print(f"  [BLOCKED] Drag start at ({start_x}, {start_y}) outside application: {start_reason}\")")
        return False
    if not end_valid:
        print(f"  [BLOCKED] Drag end at ({end_x}, {end_y}) would be outside application: {end_reason}\")")
        return False
    
    focus_bevy_window()
    pyautogui.moveTo(start_x, start_y)
    pyautogui.drag(end_x - start_x, end_y - start_y, duration=duration)
    time.sleep(0.3)
    return True


def test_sliders(rect, client_origin=None):
    """Slider testing using element-based clicking"""
    print("\n=== SLIDER TESTS (Element-Based) ===")
    observations = []
    
    click_pos = client_origin if client_origin else rect
    
    # Navigate to Sliders section using element bounds
    print("\n[Setup] Navigating to Sliders...")
    navigate_and_verify("Sliders", "nav_sliders", click_pos, retries=3, settle=0.8)
    
    capture("slider_section", rect, check_baseline=True)
    
    # Test 1: Drag slider thumb using element bounds
    print("\n[Test 1] Continuous Slider Drag")
    require_drag_element("slider_thumb_0", 150, 0, click_pos, duration=0.5)
    time.sleep(0.4)
    telemetry = read_telemetry()
    slider_val = telemetry.get("states", {}).get("slider_0_value", "N/A") if telemetry else "N/A"
    print(f"  Slider 0 value: {slider_val}")
    capture("slider_continuous_after", rect, check_baseline=True)
    observations.append({
        "test": "Continuous Slider Drag",
        "action": "Dragged slider_thumb_0 by 150px",
        "value": slider_val,
        "verify": "Check if thumb moved and value display updated"
    })
    
    # Test 2: Drag second slider thumb
    print("\n[Test 2] Discrete Slider Drag")
    require_drag_element("slider_thumb_1", 100, 0, click_pos, duration=0.5)
    time.sleep(0.4)
    telemetry = read_telemetry()
    slider_val = telemetry.get("states", {}).get("slider_1_value", "N/A") if telemetry else "N/A"
    print(f"  Slider 1 value: {slider_val}")
    capture("slider_discrete_after", rect, check_baseline=True)
    observations.append({
        "test": "Discrete Slider Drag", 
        "action": "Dragged slider_thumb_1 by 100px",
        "value": slider_val,
        "verify": "Check if thumb snaps to tick positions"
    })
    
    # Test 3: Click on track
    print("\n[Test 3] Track Click")
    require_click_element("slider_track_0", click_pos)
    time.sleep(0.3)
    capture("slider_track_after", rect)
    observations.append({
        "test": "Track Click",
        "action": "Clicked directly on slider_track_0",
        "verify": "Slider should respond to track click"
    })
    
    return observations


def test_tabs(rect, client_origin=None):
    """Tab testing using element-based clicking"""
    print("\n=== TAB TESTS (Element-Based) ===")
    observations = []
    telemetry_checks = []
    
    click_pos = client_origin if client_origin else rect
    
    # Navigate to Tabs section using element bounds
    print("\n[Setup] Navigating to Tabs section...")
    nav_result = navigate_and_verify("Tabs", "nav_tabs", click_pos, retries=3, settle=0.9)
    telemetry_checks.append(nav_result)
    if not nav_result["passed"]:
        print(f"  {nav_result['message']}")
        print("  [SKIP] Failed to navigate to Tabs")
        return observations
    
    capture("tabs_section", rect, check_baseline=True)
    
    # Test: Verify initial tab state
    print("\n[Test] Verify initial tab state")
    result = verify_telemetry_state("tab_selected", "0")
    telemetry_checks.append(result)
    print(f"  {result['message']}")
    capture("tabs_initial", rect, check_baseline=True)
    
    # Test: Click Tab 2 using element bounds
    print("\n[Test] Select Tab 2")
    require_click_element("tab_2", click_pos)
    time.sleep(0.6)  # Slightly longer wait
    result = verify_telemetry_state("tab_selected", "1")
    telemetry_checks.append(result)
    print(f"  {result['message']}")
    capture("tabs_tab2_selected", rect, check_baseline=True)
    observations.append({
        "test": "Tab Selection",
        "action": "Clicked tab_2 using element bounds",
        "telemetry_verified": result["passed"],
        "verify": [
            "Tab 2 text should be primary color",
            "Tab 2 should have 3px bottom border indicator",
            "Tab 2 content panel should be visible"
        ]
    })
    
    # Test: Click Tab 3
    print("\n[Test] Select Tab 3")
    # Show tab positions for debugging
    tab2_bounds = get_element_bounds("tab_2")
    tab3_bounds = get_element_bounds("tab_3")
    if tab2_bounds:
        print(f"  tab_2 position: x={tab2_bounds['x']}, y={tab2_bounds['y']}, w={tab2_bounds['width']}, h={tab2_bounds['height']}")
    if tab3_bounds:
        print(f"  tab_3 position: x={tab3_bounds['x']}, y={tab3_bounds['y']}, w={tab3_bounds['width']}, h={tab3_bounds['height']}")
    
    if click_element("tab_3", click_pos):
        time.sleep(0.8)  # Longer wait to ensure state settles
        result = verify_telemetry_state("tab_selected", "2")
        telemetry_checks.append(result)
        print(f"  {result['message']}")
        capture("tabs_tab3_selected", rect, check_baseline=True)
    else:
        raise TestFailure("[FAIL] Required element 'tab_3' not found in telemetry")
    
    passed = sum(1 for c in telemetry_checks if c["passed"])
    print(f"\n  Telemetry checks: {passed}/{len(telemetry_checks)} passed")
    
    return observations


def test_nav_highlighting(rect, client_origin=None):
    """Test sidebar navigation highlighting using element-based clicking"""
    print("\n=== NAVIGATION HIGHLIGHTING TESTS (Element-Based) ===")
    observations = []
    telemetry_checks = []
    
    # Use client_origin for element clicking if provided
    focus_bevy_window()
    click_pos = client_origin if client_origin else rect
    
    # Test nav items that should be visible
    sections = [
        ("nav_buttons", "Buttons"),
        ("nav_checkboxes", "Checkboxes"),
        ("nav_chips", "Chips"),
        ("nav_fab", "FAB"),
    ]
    
    for nav_id, expected_section in sections:
        print(f"\n[Test] Navigate to {expected_section}")
        if not get_element_bounds(nav_id):
            print(f"  [SKIP] {nav_id} not found")
            continue
        result = navigate_and_verify(expected_section, nav_id, click_pos, retries=3, settle=0.9)
        telemetry_checks.append(result)
        capture(f"nav_{expected_section.lower()}", rect, check_baseline=True)
        time.sleep(0.3)  # Extra delay between nav clicks

    # Leave the app in a stable, known-good section for subsequent tests.
    print("\n[Teardown] Returning to Buttons section...")
    navigate_and_verify("Buttons", "nav_buttons", click_pos, retries=5, settle=1.0)
        
    observations.append({
        "test": "Navigation Highlighting",
        "action": "Clicked through multiple sidebar sections using element bounds",
        "telemetry_verified": all(c["passed"] for c in telemetry_checks) if telemetry_checks else False,
        "verify": [
            "Selected item should have secondary_container background color",
            "Other items should have transparent background",
            "Selection should persist until another item is clicked"
        ]
    })
    
    passed = sum(1 for c in telemetry_checks if c["passed"])
    print(f"\n  Telemetry checks: {passed}/{len(telemetry_checks)} passed")
    
    return observations


def test_checkboxes(rect, client_origin=None):
    """Test checkbox toggling using element-based clicking"""
    print("\n=== CHECKBOX TESTS (Element-Based) ===")
    observations = []
    
    click_pos = client_origin if client_origin else rect
    
    # Navigate to Checkboxes section
    print("\n[Setup] Navigating to Checkboxes...")
    navigate_and_verify("Checkboxes", "nav_checkboxes", click_pos, retries=3, settle=0.8)
    
    capture("checkbox_initial", rect, check_baseline=True)
    
    # Toggle first checkbox using its test_id
    print("\n[Test] Toggle checkbox_0")
    require_click_element("checkbox_0", click_pos)
    time.sleep(0.4)
    telemetry = read_telemetry()
    events = telemetry.get("events", []) if telemetry else []
    toggled = any("Checkbox" in e for e in events[-3:])
    print(f"  Checkbox toggled: {toggled}")
    capture("checkbox_toggled", rect, check_baseline=True)
    
    # Toggle again
    print("\n[Test] Toggle checkbox_0 again")
    require_click_element("checkbox_0", click_pos)
    time.sleep(0.4)
    capture("checkbox_untoggled", rect, check_baseline=True)
    
    observations.append({
        "test": "Checkbox Toggle",
        "action": "Clicked checkbox using element bounds",
        "verify": [
            "Checkbox should show checkmark icon when checked",
            "Checkbox should be empty when unchecked",
            "Background color should change based on state"
        ]
    })
    
    return observations


def test_menus(rect, client_origin=None):
    """Test menu interactions using element-based clicking"""
    print("\n=== MENU TESTS (Element-Based) ===")
    observations = []
    
    click_pos = client_origin if client_origin else rect
    
    # Scroll sidebar to show nav_menus
    print("\n[Setup] Scrolling sidebar to show Menus...")
    scroll_sidebar_to_top(click_pos)
    time.sleep(0.2)
    # Menus is further down; scroll until it's visible (or we give up)
    for _ in range(12):
        if is_element_visible("nav_menus"):
            break
        pyautogui.moveTo(click_pos[0] + 150, click_pos[1] + 400)
        pyautogui.scroll(-6)  # Scroll down
        time.sleep(0.25)
    
    # Navigate to Menus section using element bounds
    print("\n[Setup] Navigating to Menus section...")
    time.sleep(0.5)  # Allow previous section to settle
    nav_result = navigate_and_verify("Menus", "nav_menus", click_pos, retries=3, settle=1.1)
    print(f"  {nav_result['message']}")
    if not nav_result["passed"]:
        print("  [SKIP] Failed to navigate to Menus")
        return observations
    
    capture("menu_section", rect, check_baseline=True)
    
    # Menu interactions still use relative coordinates since menu items
    # may not have TestId yet - but we'll improve this later
    window_width = rect[2] - rect[0]
    window_height = rect[3] - rect[1]
    sidebar_width_px = 220
    header_height_px = 50
    
    content_start_x = sidebar_width_px / window_width
    menu_trigger_x = content_start_x + 0.15
    menu_trigger_y = (header_height_px + 120) / window_height
    
    print("\n[Test] Open Menu")
    print(f"  Clicking trigger at ({menu_trigger_x:.3f}, {menu_trigger_y:.3f})")
    click_relative(rect, menu_trigger_x, menu_trigger_y)
    time.sleep(0.4)
    capture("menu_open", rect, check_baseline=True)
    
    # Click menu item
    menu_item_y = menu_trigger_y + 0.08
    print("\n[Test] Select Menu Item") 
    click_relative(rect, menu_trigger_x, menu_item_y)
    time.sleep(0.3)
    capture("menu_selected", rect, check_baseline=True)
    
    observations.append({
        "test": "Menu Dropdown",
        "action": "Opened menu and selected item",
        "verify": [
            "Menu should appear below trigger",
            "Menu items should be visible and clickable",
            "Menu should close after selection"
        ]
    })
    
    # Test keyboard shortcut
    print("\n[Test] Keyboard Shortcut")
    pyautogui.hotkey('ctrl', 'c')
    time.sleep(0.5)
    capture("menu_shortcut_result", rect, check_baseline=True)
    
    observations.append({
        "test": "Menu Keyboard Shortcut",
        "action": "Pressed Ctrl+C",
        "verify": "Snackbar should appear with 'Copy action triggered' message"
    })
    
    return observations


def test_lists(rect, client_origin=None):
    """Test list selection and scrolling using element-based clicking"""
    print("\n=== LIST TESTS (Element-Based) ===")
    observations = []
    telemetry_checks = []
    
    click_pos = client_origin if client_origin else rect
    
    # Scroll sidebar back to top to ensure nav_lists is visible
    print("\n[Setup] Scrolling sidebar to top...")
    scroll_sidebar_to_top(click_pos)
    
    # Navigate to Lists section
    print("\n[Setup] Navigating to Lists section...")
    time.sleep(0.5)
    nav_result = navigate_and_verify("Lists", "nav_lists", click_pos, retries=4, settle=1.3)
    telemetry_checks.append(nav_result)
    print(f"  {nav_result['message']}")
    if not nav_result["passed"]:
        print("  [SKIP] Failed to navigate to Lists")
        return observations

    # List elements showing in telemetry
    telemetry = read_telemetry()
    list_elements = [e for e in telemetry.get("elements", []) if "list_item" in e.get("test_id", "")] if telemetry else []
    print(f"  List elements found: {len(list_elements)}")
    
    capture("list_section", rect, check_baseline=True)
    
    # Test 1: Single selection - click first item
    print("\n[Test 1] Select list_item_0 (single selection mode)")
    require_click_element("list_item_0", click_pos)
    time.sleep(0.5)
    telemetry = read_telemetry()
    selected = telemetry.get("states", {}).get("list_selected_count", "0") if telemetry else "0"
    print(f"  Selected count: {selected}")
    observations.append({
        "test": "List Single Selection",
        "action": "Click list_item_0",
        "passed": selected == "1",
        "verify": "One item should be selected"
    })
    
    # Test 2: Select different item (should deselect previous in single mode)
    print("\n[Test 2] Select list_item_2 (should replace previous selection)")
    require_click_element("list_item_2", click_pos)
    time.sleep(0.5)
    telemetry = read_telemetry()
    selected_items = telemetry.get("states", {}).get("list_selected_items", "[]") if telemetry else "[]"
    print(f"  Selected items: {selected_items}")
    observations.append({
        "test": "List Selection Replace",
        "action": "Click list_item_2",
        "passed": "list_item_2" in selected_items and "list_item_0" not in selected_items,
        "verify": "Only list_item_2 should be selected (single mode)"
    })
    
    capture("list_single_selected", rect, check_baseline=True)
    
    # Test 3: Scroll the list by dragging
    print("\n[Test 3] Scroll list by dragging")
    list_bounds = get_element_bounds("list_scroll_area")
    if list_bounds:
        # Calculate scroll area center
        scroll_x = click_pos[0] + list_bounds["x"] + list_bounds["width"] / 2
        scroll_y = click_pos[1] + list_bounds["y"] + list_bounds["height"] / 2
        
        print(f"  Scrolling at ({scroll_x:.0f}, {scroll_y:.0f})")
        pyautogui.moveTo(scroll_x, scroll_y, duration=0.1)
        time.sleep(0.1)
        # Use scroll wheel to scroll down
        pyautogui.scroll(-3)  # Scroll down
        time.sleep(0.5)
        
        observations.append({
            "test": "List Scroll",
            "action": "Scroll wheel down in list area",
            "verify": "List should scroll to show more items"
        })
    
    capture("list_scrolled", rect, check_baseline=True)
    
    # Test 4: Select an item that was scrolled into view
    print("\n[Test 4] Select list_item_6 (item after scroll)")
    require_click_element("list_item_6", click_pos)
    time.sleep(0.5)
    telemetry = read_telemetry()
    selected_items = telemetry.get("states", {}).get("list_selected_items", "[]") if telemetry else "[]"
    print(f"  Selected items: {selected_items}")
    observations.append({
        "test": "List Select After Scroll",
        "action": "Click list_item_6",
        "passed": "list_item_6" in selected_items,
        "verify": "list_item_6 should be selected"
    })
    
    passed = sum(1 for o in observations if o.get("passed", False))
    print(f"\n  Tests passed: {passed}/{len(observations)}")
    
    return observations


def is_element_visible(test_id: str, window_rect=None) -> bool:
    """Check if an element is within the visible window area"""
    bounds = get_element_bounds(test_id)
    if not bounds:
        return False
    telemetry = read_telemetry()
    if not telemetry:
        return False
    states = telemetry.get("states", {})
    win_height = float(states.get("window_height", 800))
    win_width = float(states.get("window_width", 1400))
    
    # Element is visible if its center is within the window bounds
    center_y = bounds["y"] + bounds["height"] / 2
    center_x = bounds["x"] + bounds["width"] / 2
    return 0 <= center_x <= win_width and 0 <= center_y <= win_height


def scroll_sidebar(window_rect, direction="down", amount=300):
    """Scroll the sidebar to reveal more nav items"""
    # Move to sidebar area and scroll
    win_x = window_rect[0] if window_rect else 0
    win_y = window_rect[1] if window_rect else 0
    sidebar_x = win_x + 150  # Middle of sidebar
    sidebar_y = win_y + 400  # Middle of window
    
    pyautogui.moveTo(sidebar_x, sidebar_y)
    time.sleep(0.1)
    scroll_amount = -amount if direction == "down" else amount
    pyautogui.scroll(scroll_amount // 100)  # pyautogui.scroll uses "clicks" not pixels


def scroll_sidebar_to_top(client_origin):
    """Scroll the sidebar back to the top"""
    # Move to sidebar area
    sidebar_x = client_origin[0] + 150  # Middle of sidebar
    sidebar_y = client_origin[1] + 400  # Middle of visible area
    
    pyautogui.moveTo(sidebar_x, sidebar_y)
    time.sleep(0.1)
    # Scroll up a lot to get back to top
    pyautogui.scroll(30)  # Positive = scroll up
    time.sleep(0.5)


# Track cumulative sidebar scroll offset
_sidebar_scroll_offset = 0

def reset_sidebar_scroll():
    """Reset scroll tracking"""
    global _sidebar_scroll_offset
    _sidebar_scroll_offset = 0


def scroll_sidebar_to_show_element(client_origin, element_y: float, window_height: float = 1369):
    """Scroll the sidebar to show an element at a given y position. Returns scroll offset.
    
    The sidebar has nav items starting at y=95 in layout coordinates.
    When scrolled, the visual position of an element = layout_y - scroll_offset.
    We need to scroll such that the element appears on screen (between y~100 and y~window_height-100).
    """
    global _sidebar_scroll_offset
    
    sidebar_x = client_origin[0] + 150
    sidebar_y = client_origin[1] + int(window_height * 0.4)  # 40% down the window
    
    pyautogui.moveTo(sidebar_x, sidebar_y)
    time.sleep(0.1)
    
    # Calculate where the element would appear visually with current scroll
    visual_y = element_y - _sidebar_scroll_offset
    
    # Target visible range - avoid too close to top or bottom
    visible_min = 150  # Don't click too close to top
    visible_max = window_height - 150  # Don't click too close to bottom
    
    if visual_y > visible_max:
        # Element is below visible area - need to scroll DOWN (content moves up)
        scroll_needed = visual_y - visible_max + 100  # Some padding
        scroll_clicks = max(1, int(scroll_needed / 40))  # ~40px per scroll click
        print(f"  Scrolling sidebar DOWN ({scroll_clicks} ticks) - element at visual_y={visual_y:.0f} is below {visible_max:.0f}")
        pyautogui.scroll(-scroll_clicks)  # Negative = scroll down
        _sidebar_scroll_offset += scroll_clicks * 40
        time.sleep(0.5)
    elif visual_y < visible_min:
        # Element is above visible area - need to scroll UP (content moves down)
        scroll_needed = visible_min - visual_y + 50
        scroll_clicks = max(1, int(scroll_needed / 40))
        print(f"  Scrolling sidebar UP ({scroll_clicks} ticks) - element at visual_y={visual_y:.0f} is above {visible_min:.0f}")
        pyautogui.scroll(scroll_clicks)  # Positive = scroll up
        _sidebar_scroll_offset -= scroll_clicks * 40
        _sidebar_scroll_offset = max(0, _sidebar_scroll_offset)
        time.sleep(0.5)
    
    return _sidebar_scroll_offset


def click_element_with_scroll_offset(test_id: str, client_origin, scroll_offset: float = 0) -> bool:
    """Click an element, accounting for scroll offset.
    Uses safe_click to prevent clicking outside the application."""
    bounds = get_element_bounds(test_id)
    if not bounds:
        print(f"  [MISS] Element '{test_id}' not found in telemetry")
        return False
    
    center_x = client_origin[0] + bounds["x"] + bounds["width"] / 2
    # Subtract scroll offset because scrolling moves content up on screen
    center_y = client_origin[1] + bounds["y"] + bounds["height"] / 2 - scroll_offset
    
    print(f"  Clicking '{test_id}' at ({center_x:.0f}, {center_y:.0f}) [original_y={bounds['y']:.0f}, scroll_offset={scroll_offset:.0f}]")
    
    # Use safe_click to prevent clicking outside window
    if not safe_click(center_x, center_y, f"click element '{test_id}' with scroll offset"):
        return False
    
    time.sleep(0.3)
    return True


def test_with_element_bounds(rect):
    """Test using element bounds from telemetry for precise clicking"""
    print("\n=== ELEMENT BOUNDS TESTING ===")
    observations = []
    
    # First, list available elements
    list_available_elements()
    
    # Get window dimensions from telemetry
    telemetry = read_telemetry()
    win_height = float(telemetry.get("states", {}).get("window_height", 800)) if telemetry else 800
    print(f"\nWindow height: {win_height}px")
    
    # Test 1: Navigate to VISIBLE nav items first (Checkboxes is near the top)
    print("\n[Test 1] Navigate to Checkboxes (visible nav item)")
    if is_element_visible("nav_checkboxes", rect):
        if click_element("nav_checkboxes", rect):
            time.sleep(0.5)
            result = verify_telemetry_state("selected_section", "Checkboxes")
            print(f"  {result['message']}")
            observations.append({
                "test": "Nav Click via TestId",
                "action": "Click nav_checkboxes",
                "element": "nav_checkboxes",
                "passed": result["passed"],
                "verify": "Navigation should work using element bounds"
            })
    else:
        print("  [SKIP] nav_checkboxes not visible")
    
    # Test 2: Click checkbox (should now be visible after nav)
    print("\n[Test 2] Click checkbox_0")
    time.sleep(0.5)  # Wait for telemetry update and UI to settle
    if click_element("checkbox_0", rect):
        time.sleep(0.5)
        # Check telemetry for checkbox state change
        telemetry = read_telemetry()
        events = telemetry.get("events", []) if telemetry else []
        checkbox_toggled = any("Checkbox" in e for e in events[-5:])
        observations.append({
            "test": "Checkbox Click via TestId",
            "action": "Click checkbox_0",
            "element": "checkbox_0",
            "passed": checkbox_toggled,
            "verify": "Checkbox should toggle when clicked"
        })
    
    # Test 3: Navigate to Switches (visible)
    print("\n[Test 3] Navigate to Switches")
    time.sleep(0.5)  # Extra delay before next nav
    if is_element_visible("nav_switches", rect):
        if click_element("nav_switches", rect):
            time.sleep(0.7)  # Longer wait for navigation
            result = verify_telemetry_state("selected_section", "Switches")
            print(f"  {result['message']}")
            observations.append({
                "test": "Nav to Switches",
                "action": "Click nav_switches",
                "element": "nav_switches",
                "passed": result["passed"],
                "verify": "Should navigate to Switches section"
            })
    
    # Test 4: Click switch
    print("\n[Test 4] Click switch_0")
    time.sleep(0.5)  # Wait for switch elements to appear
    if click_element("switch_0", rect):
        time.sleep(0.5)
        telemetry = read_telemetry()
        events = telemetry.get("events", []) if telemetry else []
        switch_toggled = any("Switch" in e for e in events[-5:])
        observations.append({
            "test": "Switch Click via TestId",
            "action": "Click switch_0",
            "element": "switch_0",
            "passed": switch_toggled,
            "verify": "Switch should toggle when clicked"
        })
    
    # Test 5: Navigate to Radio Buttons (visible)
    print("\n[Test 5] Navigate to Radio Buttons")
    time.sleep(1.0)  # Longer delay before next nav - ensure UI has fully settled
    # Move mouse to sidebar first to ensure focus
    pyautogui.moveTo(100, 300)
    time.sleep(0.2)
    if is_element_visible("nav_radio_buttons", rect):
        if click_element("nav_radio_buttons", rect):
            time.sleep(0.8)  # Longer wait for navigation to complete
            result = verify_telemetry_state("selected_section", "RadioButtons")
            print(f"  {result['message']}")
            observations.append({
                "test": "Nav to RadioButtons",
                "action": "Click nav_radio_buttons",
                "element": "nav_radio_buttons",
                "passed": result["passed"],
                "verify": "Should navigate to RadioButtons section"
            })
    
    # Test 6: Click radio button
    print("\n[Test 6] Click radio_1")
    time.sleep(0.5)  # Wait for radio elements to appear
    if click_element("radio_1", rect):
        time.sleep(0.5)
        telemetry = read_telemetry()
        events = telemetry.get("events", []) if telemetry else []
        radio_selected = any("Radio" in e for e in events[-5:])
        observations.append({
            "test": "Radio Click via TestId",
            "action": "Click radio_1",
            "element": "radio_1",
            "passed": radio_selected,
            "verify": "Radio button should be selected"
        })
    
    # Test 7: Navigate to Sliders using element bounds
    print("\n[Test 7] Navigate to Sliders")
    time.sleep(0.7)  # Longer delay before nav
    if click_element("nav_sliders", rect):
        time.sleep(0.8)  # Longer wait for navigation
        result = verify_telemetry_state("selected_section", "Sliders")
        print(f"  {result['message']}")
        observations.append({
            "test": "Nav to Sliders",
            "action": "Click nav_sliders",
            "element": "nav_sliders",
            "passed": result["passed"],
            "verify": "Should navigate to Sliders section"
        })
    
    # Test 8: Drag slider thumb
    print("\n[Test 8] Drag slider_thumb_0")
    time.sleep(0.5)
    if drag_element("slider_thumb_0", 100, 0, rect):
        time.sleep(0.5)
        telemetry = read_telemetry()
        slider_val = telemetry.get("states", {}).get("slider_0_value", "0") if telemetry else "0"
        slider_changed = float(slider_val) > 10  # Check if slider moved from initial
        observations.append({
            "test": "Slider Drag via TestId",
            "action": "Drag slider_thumb_0 by 100px",
            "element": "slider_thumb_0",
            "passed": slider_changed,
            "verify": f"Slider value should change (got {slider_val})"
        })
    
    return observations


def run_all_tests():
    """Run all component tests"""
    global _showcase_log_handles
    print("=" * 60)
    print("BEVY MATERIAL UI - COMPONENT TESTING")
    print("=" * 60)
    
    # Delete old telemetry file
    if TELEMETRY_FILE.exists():
        TELEMETRY_FILE.unlink()
    
    # Start the app with telemetry enabled
    import os
    env = os.environ.copy()
    env['BEVY_TELEMETRY'] = '1'
    
    print("\nStarting Bevy showcase with telemetry enabled...")
    print("  (This may take a while if compilation is needed)")
    proc = launch_showcase(env)
    
    # Wait for telemetry file to appear (indicates app has started)
    print("  Waiting for application to start...")
    max_wait = 120  # 2 minutes max for compilation + startup
    waited = 0
    while waited < max_wait:
        if TELEMETRY_FILE.exists():
            # Verify it has content
            try:
                with open(TELEMETRY_FILE, 'r') as f:
                    content = f.read()
                    if content.strip() and '"elements"' in content:
                        print(f"  Application started after {waited}s")
                        break
            except:
                pass
        
        # Check if process failed
        if proc.poll() is not None:
            print("\n  ERROR: Application failed to start!")
            if _last_showcase_stdout_log is not None:
                print(f"  stdout log: {_last_showcase_stdout_log}")
            if _last_showcase_stderr_log is not None:
                print(f"  stderr log: {_last_showcase_stderr_log}")
                tail = _tail_text_file(_last_showcase_stderr_log, max_bytes=3000)
                if tail.strip():
                    print("\n  --- stderr tail ---")
                    print(tail)
            return []
        
        time.sleep(1)
        waited += 1
        if waited % 10 == 0:
            print(f"  Still waiting... ({waited}s)")
    
    if waited >= max_wait:
        print("  ERROR: Timeout waiting for application to start")
        proc.terminate()
        return []
    
    # Give the UI a moment to fully render
    time.sleep(2)
    
    # Find window - returns (client_origin, window_rect)
    result = wait_for_bevy_window(timeout=20.0, maximize=True)
    if not result:
        print("Could not find Bevy window!")
        proc.terminate()
        return []
    
    client_origin, window_rect = result
    
    # Use window_rect for screenshots, client_origin for element clicking
    rect = window_rect  # For backward compatibility with screenshot functions
    
    print(f"Window rect: {rect}")
    print(f"Client origin for clicks: {client_origin}")
    
    all_observations = []
    
    try:
        # Run element bounds test first (uses test_id to find elements dynamically)
        # Pass client_origin for accurate element clicking
        all_observations.extend(test_with_element_bounds(client_origin))
        
        # Then run element-based tests (pass both rect for screenshots and client_origin for clicking)
        all_observations.extend(test_nav_highlighting(rect, client_origin))
        all_observations.extend(test_checkboxes(rect, client_origin))
        all_observations.extend(test_sliders(rect, client_origin))
        all_observations.extend(test_tabs(rect, client_origin))
        all_observations.extend(test_lists(rect, client_origin))
        all_observations.extend(test_menus(rect, client_origin))
        
        # Give time for telemetry to be written
        time.sleep(1)
        
        # Read telemetry
        telemetry = read_telemetry()
        if telemetry:
            print("\n" + "=" * 60)
            print("COMPONENT TELEMETRY")
            print("=" * 60)
            print(json.dumps(telemetry, indent=2))
        else:
            print("\nNo telemetry file found - check if BEVY_TELEMETRY env var is working")
            
    finally:
        proc.terminate()
        proc.wait(timeout=5)
        for h in _showcase_log_handles:
            try:
                h.close()
            except Exception:
                pass
        _showcase_log_handles = []
    
    # Print summary for AI iteration
    print("\n" + "=" * 60)
    print("TEST OBSERVATIONS FOR AI AGENT")
    print("=" * 60)
    
    for obs in all_observations:
        print(f"\n### {obs['test']}")
        print(f"Action: {obs.get('action', 'N/A')}")
        if 'verify' in obs:
            if isinstance(obs['verify'], list):
                print("Verify:")
                for v in obs['verify']:
                    print(f"  - {v}")
            else:
                print(f"Verify: {obs['verify']}")
        if 'passed' in obs:
            print(f"Passed: {obs['passed']}")
    
    print("\n" + "=" * 60)
    print("SCREENSHOTS SAVED TO:", OUTPUT_DIR)
    print("=" * 60)
    
    # Visual regression summary
    if visual_results:
        print("\n" + generate_report(visual_results))
    
    return all_observations


def run_single_component_test(component: str, maximized: bool = True):
    """Run test for a single component"""
    global _showcase_log_handles
    print("=" * 60)
    print(f"TESTING COMPONENT: {component.upper()}")
    print(f"Window mode: {'MAXIMIZED' if maximized else 'NORMAL'}")
    print("=" * 60)
    
    # Delete old telemetry file
    if TELEMETRY_FILE.exists():
        TELEMETRY_FILE.unlink()
    
    # Start the app
    import os
    env = os.environ.copy()
    env['BEVY_TELEMETRY'] = '1'
    
    print("\nStarting Bevy showcase...")
    proc = launch_showcase(env)
    
    # Wait for app to start
    print("  Waiting for application to start...")
    max_wait = 120
    waited = 0
    while waited < max_wait:
        if TELEMETRY_FILE.exists():
            try:
                with open(TELEMETRY_FILE, 'r') as f:
                    content = f.read()
                    if content.strip() and '"elements"' in content:
                        print(f"  Application started after {waited}s")
                        break
            except:
                pass
        
        if proc.poll() is not None:
            print("\n  ERROR: Application failed to start!")
            if _last_showcase_stdout_log is not None:
                print(f"  stdout log: {_last_showcase_stdout_log}")
            if _last_showcase_stderr_log is not None:
                print(f"  stderr log: {_last_showcase_stderr_log}")
                tail = _tail_text_file(_last_showcase_stderr_log, max_bytes=3000)
                if tail.strip():
                    print("\n  --- stderr tail ---")
                    print(tail)
            return []
        
        time.sleep(1)
        waited += 1
        if waited % 10 == 0:
            print(f"  Still waiting... ({waited}s)")
    
    if waited >= max_wait:
        print("  ERROR: Timeout")
        proc.terminate()
        return []
    
    time.sleep(2)
    
    # Find window
    result = wait_for_bevy_window(timeout=20.0, maximize=maximized)
    if not result:
        print("Could not find Bevy window!")
        proc.terminate()
        return []
    
    client_origin, window_rect = result
    rect = window_rect
    
    print(f"Window rect: {rect}")
    print(f"Client origin: {client_origin}")
    
    observations = []
    
    try:
        # Map component name to test function
        test_map = {
            'nav': lambda: test_nav_highlighting(rect, client_origin),
            'checkboxes': lambda: test_checkboxes(rect, client_origin),
            'sliders': lambda: test_sliders(rect, client_origin),
            'tabs': lambda: test_tabs(rect, client_origin),
            'lists': lambda: test_lists(rect, client_origin),
            'menus': lambda: test_menus(rect, client_origin),
            'bounds': lambda: test_with_element_bounds(client_origin),
        }
        
        if component == 'all':
            # Run all tests
            observations.extend(test_with_element_bounds(client_origin))
            observations.extend(test_nav_highlighting(rect, client_origin))
            observations.extend(test_checkboxes(rect, client_origin))
            observations.extend(test_sliders(rect, client_origin))
            observations.extend(test_tabs(rect, client_origin))
            observations.extend(test_lists(rect, client_origin))
            observations.extend(test_menus(rect, client_origin))
        elif component in test_map:
            observations.extend(test_map[component]())
        else:
            print(f"Unknown component: {component}")
            print(f"Available: {', '.join(test_map.keys())}, all")
        
        time.sleep(1)
        
        # Show telemetry
        telemetry = read_telemetry()
        if telemetry:
            print("\n" + "=" * 60)
            print("TELEMETRY SUMMARY")
            print("=" * 60)
            states = telemetry.get("states", {})
            print(f"  selected_section: {states.get('selected_section')}")
            print(f"  tab_selected: {states.get('tab_selected')}")
            print(f"  list_selected_items: {states.get('list_selected_items', '[]')}")
            print(f"  elements_with_bounds: {states.get('elements_with_bounds')}")
            
            # Show recent events
            events = telemetry.get("events", [])
            if events:
                print(f"\n  Recent events:")
                for e in events[-10:]:
                    print(f"    {e}")
    finally:
        proc.terminate()
        proc.wait(timeout=5)
        for h in _showcase_log_handles:
            try:
                h.close()
            except Exception:
                pass
        _showcase_log_handles = []
    
    # Print results
    print("\n" + "=" * 60)
    print("TEST RESULTS")
    print("=" * 60)
    
    passed = sum(1 for o in observations if o.get('passed', False))
    failed = sum(1 for o in observations if 'passed' in o and not o['passed'])
    
    for obs in observations:
        status = "[PASS]" if obs.get('passed', False) else "[FAIL]" if 'passed' in obs else "[INFO]"
        print(f"  {status} {obs['test']}")
    
    print(f"\nSummary: {passed} passed, {failed} failed")
    
    if visual_results:
        print("\n" + generate_report(visual_results))
    
    return observations


def navigate_to_section(section_name: str, client_origin) -> bool:
    """Navigate to a specific section and verify"""
    nav_id = f"nav_{section_name.lower().replace(' ', '_')}"
    
    # Scroll sidebar to top first
    scroll_sidebar_to_top(client_origin)
    time.sleep(0.3)
    
    print(f"\nNavigating to {section_name}...")
    if click_element(nav_id, client_origin):
        time.sleep(1.0)
        result = verify_telemetry_state("selected_section", section_name)
        print(f"  {result['message']}")
        return result['passed']
    else:
        print(f"  [MISS] {nav_id} not found")
        return False


def test_navigation_only(client_origin):
    """Test navigation to all sections using position-based clicking after scrolling."""
    print("\n=== NAVIGATION-ONLY TEST ===")
    
    # Map display name to TestId (nav_xxx format used in the showcase example)
    sections = [
        ("Buttons", "nav_buttons"),
        ("Checkboxes", "nav_checkboxes"),
        ("Switches", "nav_switches"),
        ("RadioButtons", "nav_radio_buttons"),
        ("Chips", "nav_chips"),
        ("FAB", "nav_fab"),
        ("Badges", "nav_badges"),
        ("Progress", "nav_progress"),
        ("Cards", "nav_cards"),
        ("Dividers", "nav_dividers"),
        ("Lists", "nav_lists"),
        ("Icons", "nav_icons"),
        ("IconButtons", "nav_icon_buttons"),
        ("Sliders", "nav_sliders"),
        ("TextFields", "nav_text_fields"),
        ("Dialogs", "nav_dialogs"),
        ("Menus", "nav_menus"),
        ("Tabs", "nav_tabs"),
        ("Select", "nav_select"),
        ("Snackbar", "nav_snackbar"),
        ("Tooltips", "nav_tooltips"),
        ("AppBar", "nav_app_bar"),
        ("Layouts", "nav_layouts"),
        ("ThemeColors", "nav_theme_colors"),
    ]
    
    # Reset sidebar to top
    scroll_sidebar_to_top(client_origin)
    time.sleep(0.5)
    
    # Get actual window dimensions from the window rect (not telemetry which is logical size)
    if _window_bounds:
        actual_window_height = _window_bounds[3] - _window_bounds[1]
        actual_client_height = _window_bounds[3] - client_origin[1]
    else:
        actual_client_height = 800
    
    print(f"Actual client height: {actual_client_height}px")
    
    # Visible area for clicking (leave margin at bottom for window chrome)
    visible_bottom = actual_client_height - 50
    
    results = []
    cumulative_scroll = 0
    
    for section_name, nav_id in sections:
        print(f"\nNavigating to {section_name}...")
        
        # Get element bounds
        bounds = get_element_bounds(nav_id)
        if not bounds:
            print(f"  [MISS] {nav_id} not found")
            results.append({"section": section_name, "passed": False})
            continue
        
        # Layout position from telemetry
        layout_y = bounds["y"]
        
        # Calculate visual position (where it appears on screen) accounting for any scroll
        visual_y = layout_y - cumulative_scroll
        
        # Calculate click position 
        click_x = client_origin[0] + bounds["x"] + bounds["width"] / 2
        click_y = client_origin[1] + visual_y + bounds["height"] / 2
        
        # Check if click would be in bounds using actual window bounds
        click_valid, click_reason = is_click_in_bounds(click_x, click_y)
        
        if not click_valid:
            print(f"  Click would be out of bounds: {click_reason}")
            
            # Calculate how much we need to scroll to bring element into view
            # The element is below the visible area, so we need to scroll down
            if _window_bounds:
                window_bottom = _window_bounds[3]
                scroll_needed = click_y - window_bottom + 100  # Scroll to bring element into view with margin
            else:
                scroll_needed = 200  # Default scroll amount
            
            # Use the scrollbar thumb to scroll
            # The scrollbar thumb drag amount is proportional to scroll amount
            thumb_drag_amount = scroll_needed * 0.3  # Approximate ratio (thumb moves less than content)
            
            print(f"  Using scrollbar: drag thumb by ~{thumb_drag_amount:.0f}px to scroll content ~{scroll_needed:.0f}px")
            
            # Get current scroll position before
            scroll_before = get_scroll_position("sidebar_scroll_y")
            
            if drag_scrollbar("sidebar_scroll_thumb", thumb_drag_amount, client_origin):
                time.sleep(0.3)
                # Check telemetry for new scroll position
                scroll_after = get_scroll_position("sidebar_scroll_y")
                actual_scroll = scroll_after - scroll_before
                print(f"  Scroll position changed: {scroll_before:.0f} -> {scroll_after:.0f} (delta={actual_scroll:.0f})")
                
                # Update cumulative scroll with actual amount
                if actual_scroll > 0:
                    cumulative_scroll = scroll_after
                else:
                    # Scrolling didn't work - the content probably doesn't overflow
                    print(f"  [INFO] Sidebar content may not overflow - no scroll possible")
            else:
                print(f"  [INFO] Scrollbar not available or drag failed")
            
            # Recalculate visual position and click coordinates after scroll attempt
            visual_y = layout_y - cumulative_scroll
            click_y = client_origin[1] + visual_y + bounds["height"] / 2
            print(f"  After scroll: visual_y={visual_y:.0f}, cumulative_scroll={cumulative_scroll:.0f}")
        
        # Calculate click position 
        click_x = client_origin[0] + bounds["x"] + bounds["width"] / 2
        click_y = client_origin[1] + visual_y + bounds["height"] / 2
        
        print(f"  Click at ({click_x:.0f}, {click_y:.0f}) [layout_y={layout_y:.0f}, scroll={cumulative_scroll:.0f}, visual_y={visual_y:.0f}]")
        
        # Use safe_click to verify bounds
        if safe_click(click_x, click_y, f"click {nav_id}"):
            time.sleep(0.8)
            result = verify_telemetry_state("selected_section", section_name)
            print(f"  {result['message']}")
            results.append({"section": section_name, "passed": result['passed']})
        else:
            print(f"  [FAIL] Click blocked - outside window bounds")
            results.append({"section": section_name, "passed": False})
        
        time.sleep(0.2)
    
    print("\n" + "=" * 60)
    print("NAVIGATION RESULTS")
    print("=" * 60)
    
    passed = sum(1 for r in results if r['passed'])
    for r in results:
        status = "[PASS]" if r['passed'] else "[FAIL]"
        print(f"  {status} {r['section']}")
    
    print(f"\nTotal: {passed}/{len(results)} sections navigated successfully")
    return results


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description="Bevy Material UI Component Testing")
    parser.add_argument('component', nargs='?', default='all',
                        help='Component to test: nav, checkboxes, sliders, tabs, lists, menus, bounds, all')
    parser.add_argument('--normal', action='store_true',
                        help='Use normal window size instead of maximized')
    parser.add_argument('--nav-only', action='store_true',
                        help='Only test navigation to all sections')
    parser.add_argument('--list-elements', action='store_true',
                        help='List all available TestId elements')
    parser.add_argument('--matrix', action='store_true',
                        help='Run responsive tests at multiple window sizes')
    parser.add_argument('--start-from', type=str, default='',
                        help='For --matrix runs: start at this section name or nav_test_id (e.g. "Sliders" or "nav_sliders")')
    parser.add_argument('--only', type=str, default='',
                        help='For --matrix runs: run only these sections (comma-separated section names or nav_test_ids)')
    parser.add_argument('--resume', action='store_true',
                        help='For --matrix runs: resume from the last failing section using test_output/last_failure.json')
    parser.add_argument('--resume-all-sizes', action='store_true',
                        help='With --resume: continue from the failing size through remaining sizes (default: only the failing size)')
    parser.add_argument('--resume-file', type=str, default=str(LAST_FAILURE_FILE),
                        help='Path to the resume state file (default: tests/ui_tests/test_output/last_failure.json)')
    parser.add_argument('--sizes', type=str, default='',
                        help='Comma-separated size presets or WxH list (e.g. phone,tablet,1280x720)')
    
    args = parser.parse_args()
    
    if args.matrix:
        try:
            sizes = parse_sizes_arg(args.sizes)
        except ValueError as e:
            print(str(e))
            sys.exit(2)

        only_list = [s.strip() for s in (args.only or "").split(",") if s.strip()]
        start_from = (args.start_from or "").strip() or None
        resume_file = Path(args.resume_file) if args.resume_file else LAST_FAILURE_FILE

        try:
            run_size_matrix(
                args.component,
                sizes,
                start_from=start_from,
                only=only_list or None,
                resume=bool(args.resume),
                resume_file=resume_file,
                resume_all_sizes=bool(args.resume_all_sizes),
            )
        except TestFailure as e:
            print(str(e))
            sys.exit(1)

    elif args.nav_only:
        # Special mode: just test navigation
        print("=" * 60)
        print("NAVIGATION TEST MODE")
        print("=" * 60)
        
        if TELEMETRY_FILE.exists():
            TELEMETRY_FILE.unlink()
        
        import os
        env = os.environ.copy()
        env['BEVY_TELEMETRY'] = '1'
        
        print("\nStarting Bevy showcase...")
        proc = launch_showcase(env)
        
        # Wait for start
        waited = 0
        while waited < 120:
            if TELEMETRY_FILE.exists():
                try:
                    with open(TELEMETRY_FILE, 'r') as f:
                        if '"elements"' in f.read():
                            break
                except:
                    pass
            time.sleep(1)
            waited += 1
        
        time.sleep(2)
        result = wait_for_bevy_window(timeout=20.0, maximize=not args.normal)
        if result:
            client_origin, _ = result
            test_navigation_only(client_origin)
        proc.terminate()
        
    elif args.list_elements:
        # List available elements from existing telemetry
        if TELEMETRY_FILE.exists():
            list_available_elements()
        else:
            print("No telemetry.json file found. Run a test first.")
    else:
        try:
            run_single_component_test(args.component, maximized=not args.normal)
        except TestFailure as e:
            print(str(e))
            sys.exit(1)
