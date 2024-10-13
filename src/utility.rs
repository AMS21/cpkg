use crate::prelude::*;

#[must_use]
pub fn file_exists(path: &str) -> bool {
    std::path::Path::new(path).exists()
}

pub fn command_to_full_string(command: &std::process::Command) -> Result<String> {
    // Push the called program name
    let program_string = command.get_program();
    let mut command_string = command
        .get_program()
        .to_str()
        .ok_or(Error::OsStringConversion {
            original: program_string.into(),
        })?
        .to_owned();

    // Separate by a space
    command_string.push(' ');

    // Push all the arguments separated with a whitespace
    for argument in command.get_args() {
        command_string.push_str(argument.to_str().ok_or(Error::OsStringConversion {
            original: argument.into(),
        })?);
        command_string.push(' ');
    }

    // remove trailing space
    command_string.pop();

    Ok(command_string)
}
