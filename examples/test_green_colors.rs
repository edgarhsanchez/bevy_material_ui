//! Test green color generation with the new HctSolver

use bevy_material_ui::color::Hct;

fn main() {
    // Test with pure green (#00FF00)
    let green = Hct::from_argb(0xFF00FF00);
    println!("Pure Green (#00FF00):");
    println!("  Hue: {:.2}°", green.hue());
    println!("  Chroma: {:.2}", green.chroma());
    println!("  Tone: {:.2}", green.tone());
    println!("  Back to ARGB: #{:08X}", green.to_argb());

    // Test with Material You green (from palette)
    let green2 = Hct::new(140.0, 60.0, 50.0);
    println!("\nMaterial Green (H:140, C:60, T:50):");
    println!("  Hue: {:.2}°", green2.hue());
    println!("  Chroma: {:.2}", green2.chroma());
    println!("  Tone: {:.2}", green2.tone());
    println!("  ARGB: #{:08X}", green2.to_argb());

    // Test various green tones
    println!("\nGreen Tonal Palette (H:140, C:60):");
    for tone in [0, 10, 20, 30, 40, 50, 60, 70, 80, 90, 95, 100] {
        let color = Hct::new(140.0, 60.0, tone as f64);
        println!(
            "  Tone {:3}: #{:08X} (actual chroma: {:.2})",
            tone,
            color.to_argb(),
            color.chroma()
        );
    }

    // Test lime green (highly chromatic)
    println!("\nLime Green (H:120, C:80, T:70):");
    let lime = Hct::new(120.0, 80.0, 70.0);
    println!("  ARGB: #{:08X}", lime.to_argb());
    println!("  Actual chroma: {:.2}", lime.chroma());

    // Test forest green
    println!("\nForest Green (H:130, C:40, T:30):");
    let forest = Hct::new(130.0, 40.0, 30.0);
    println!("  ARGB: #{:08X}", forest.to_argb());
    println!("  Actual chroma: {:.2}", forest.chroma());
}
