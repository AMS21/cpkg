use clap::ValueEnum;
use clap_complete::generate_to;
use clap_complete::Shell;
use clap_complete_fig::Fig;
use clap_complete_nushell::Nushell;
use std::env;
use std::io::Error;

include!("src/cli.rs");

fn main() -> Result<(), Error> {
    let out_directory =
        std::path::PathBuf::from(std::env::var_os("OUT_DIR").ok_or(std::io::ErrorKind::NotFound)?);

    let mut command_line = get_command_line();

    // Generate shell completions
    for &shell in Shell::value_variants() {
        generate_to(shell, &mut command_line, crate_name!(), &out_directory)?;
    }

    generate_to(Fig, &mut command_line, crate_name!(), &out_directory)?;
    generate_to(Nushell, &mut command_line, crate_name!(), &out_directory)?;

    // Generate man pages
    let man = clap_mangen::Man::new(command_line);
    let mut buffer: Vec<u8> = Vec::default();

    man.render(&mut buffer)?;

    std::fs::write(out_directory.join(format!("{}.1", crate_name!())), buffer)?;

    Ok(())
}
