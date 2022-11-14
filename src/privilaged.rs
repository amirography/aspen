use std::{env, process::Command};
use thiserror::Error;

fn find_this_program() -> Vec<String> {
    // gathering arguments
    let mut args: Vec<String> = env::args().collect();

    // replacing the first argument which is basename of the executible with the absolute path to the current executible
    if let Some(absolute_path) = env::current_exe()
        .ok()
        .and_then(|p| p.to_str().map(|p| p.to_string()))
    {
        args[0] = absolute_path;
    };

    return args;
}

fn add_sudo(privilager: Privilager, program: Vec<String>) -> Command {
    let mut command: Command = Command::new("/usr/bin/env");
    match privilager {
        Privilager::Sudo => command.args(vec!["sudo", "-E"]),
        Privilager::Doas => command.arg("doas"),
    };
    command.args(program);
    command
}

pub enum Privilager {
    Sudo,
    Doas,
}

pub fn this_program(privilager: Privilager) -> Result<(), RunningError> {
    let mut child = add_sudo(privilager, find_this_program())
        .spawn()
        .map_err(|e| RunningError::Spawning(e.to_string()))?;

    child
        .wait()
        .map_err(|e| RunningError::BadStatus(e.to_string()))?;
    Ok(())
}

#[derive(Debug, Error)]
pub enum RunningError {
    #[error("problem spawning a child process: {0}")]
    Spawning(String),

    #[error("exited with none-zero status: {0}")]
    BadStatus(String),
}

pub fn am_sudo() -> bool {
    let running_as: is_sudo::RunningAs = is_sudo::check();

    match running_as {
        is_sudo::RunningAs::Root => true,
        is_sudo::RunningAs::User => false,
    }
}
