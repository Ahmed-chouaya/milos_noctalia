use color_eyre::install;
use milos_niri::wizard::run_wizard;

fn main() -> color_eyre::Result<()> {
    // Install color-eyre FIRST - before any errors can occur
    install()?;

    // Run the installer wizard
    if let Err(e) = run_wizard() {
        eprintln!("Wizard error: {}", e);
    }

    Ok(())
}
