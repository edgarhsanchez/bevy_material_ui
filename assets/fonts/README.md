# Font Setup for International Text Support

The Bevy Material UI showcase supports multiple languages including:
- Latin scripts (English, Spanish, French, German)
- CJK (Chinese, Japanese, Korean)
- Hebrew
- And more

## Required Font

To properly display all international characters, you need a font with comprehensive Unicode coverage.

### Recommended: Noto Sans

**Download**: https://fonts.google.com/noto/specimen/Noto+Sans

1. Visit the link above
2. Click "Get font" or "Download family"
3. Extract the downloaded ZIP file
4. Copy `NotoSans-Regular.ttf` to this directory (`assets/fonts/`)

### Alternative Fonts

Any font with broad Unicode support will work, for example:
- **Noto Sans** (recommended - covers most scripts)
- **Arial Unicode MS** (if available on your system)
- **Source Sans 3** (good Latin + some extended scripts)
- **Roboto** (good for Latin + Cyrillic + Greek, limited CJK)

## File Structure

```
assets/
  fonts/
    NotoSans-Regular.ttf   <-- Place the font file here
    README.md              <-- This file
```

## Without the Font

If you don't add a font file, the showcase will still run but:
- English, Spanish, French, German text will display correctly
- Chinese, Japanese, Korean characters will show as boxes □
- Hebrew text will show as boxes □

## Testing

After adding the font:
1. Run the showcase: `cargo run --example showcase`
2. Navigate to the "Translations" section
3. Select different languages from the dropdown
4. Verify that all characters display correctly

## Troubleshooting

**Problem**: Characters still showing as boxes after adding font

**Solutions**:
1. Verify the font file is named exactly `NotoSans-Regular.ttf`
2. Verify the font file is in the correct directory
3. Restart the application (hot-reload doesn't work for fonts)
4. Check console logs for font loading messages

**Problem**: Application warns about missing font

This is expected if you haven't added the font yet. The application will fall back to Bevy's default font, which only supports basic Latin characters.
