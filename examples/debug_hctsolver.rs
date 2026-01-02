//! Debug test for HctSolver

use bevy_material_ui::color::Hct;

fn main() {
    println!("Testing HctSolver with green colors...\n");

    // Simple test
    println!("Test 1: Material Green (H:140, C:60, T:50)");
    let green = Hct::new(140.0, 60.0, 50.0);
    println!(
        "Result: Hue={:.2}, Chroma={:.2}, Tone={:.2}",
        green.hue(),
        green.chroma(),
        green.tone()
    );
    println!("ARGB: #{:08X}\n", green.to_argb());

    // Test with blue (should work)
    println!("Test 2: Blue (H:240, C:60, T:50)");
    let blue = Hct::new(240.0, 60.0, 50.0);
    println!(
        "Result: Hue={:.2}, Chroma={:.2}, Tone={:.2}",
        blue.hue(),
        blue.chroma(),
        blue.tone()
    );
    println!("ARGB: #{:08X}\n", blue.to_argb());

    // Test with red (should work)
    println!("Test 3: Red (H:0, C:60, T:50)");
    let red = Hct::new(0.0, 60.0, 50.0);
    println!(
        "Result: Hue={:.2}, Chroma={:.2}, Tone={:.2}",
        red.hue(),
        red.chroma(),
        red.tone()
    );
    println!("ARGB: #{:08X}\n", red.to_argb());
}
