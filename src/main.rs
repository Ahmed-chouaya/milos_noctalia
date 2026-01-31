use color_eyre::install;
use milos_niri::generator::{self, UserConfig};
use std::env;
use std::path::PathBuf;

fn main() -> color_eyre::Result<()> {
    // Install color-eyre FIRST - before any errors can occur
    install()?;

    // Check for --generate flag (CLI mode for testing generation)
    if env::args().any(|arg| arg == "--generate") {
        return run_generate_command();
    }

    // Run the installer wizard
    if let Err(e) = milos_niri::wizard::run_wizard() {
        eprintln!("Wizard error: {}", e);
    }

    Ok(())
}

/// Run the configuration generator in CLI mode (for testing).
/// Creates example configuration files from sample user data.
fn run_generate_command() -> color_eyre::Result<()> {
    println!("=== milos_niri Configuration Generator (CLI Mode) ===\n");

    // Create sample user configuration for testing
    let config = UserConfig {
        hostname: "niri-host".to_string(),
        username: "milgraph".to_string(),
        full_name: "Milgraph User".to_string(),
        git_username: "milgraph".to_string(),
        git_email: "milgraph@example.com".to_string(),
        timezone: "America/New_York".to_string(),
        keyboard_layout: "us".to_string(),
        wallpaper_dir: "~/Pictures/Wallpapers".to_string(),
        avatar_path: None,
        screenshot_dir: "~/Pictures/Screenshots".to_string(),
    };

    // Determine output directory (default: milos-output in current dir)
    let output_dir: PathBuf = env::args()
        .nth(1)
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("milos-output"));

    println!("Output directory: {}", output_dir.display());
    println!("Generating configuration files...\n");

    // Generate all configuration files
    match generator::generate_all(&config, &output_dir) {
        Ok(generated_paths) => {
            println!("✅ Successfully generated {} configuration files:\n", generated_paths.len());

            // Print each generated file path
            for (i, path) in generated_paths.iter().enumerate() {
                println!("  {}. {}", i + 1, path.display());
            }

            println!("\n📁 All files written to: {}", output_dir.display());
            println!("\nYou can now review the generated configuration files before applying them.");

            Ok(())
        }
        Err(e) => {
            eprintln!("❌ Generation failed: {}", e);
            Err(e.into())
        }
    }
}
