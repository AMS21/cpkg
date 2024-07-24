use cpkg::prelude::*;
use cpkg::run_main;

fn main() -> Result<()> {
    // Install error handler
    color_eyre::install()?;

    // Run main function
    run_main()
}
