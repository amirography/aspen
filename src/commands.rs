use std::process::Command;

use super::config;

pub fn commands_runner(conf: &config::Config, c: CommandOption) -> Result<(), CommandRepoErr> {
    match c {
        CommandOption::Swtich => Command::new("nixos-rebuild")
            .args(vec![
                "switch",
                "--flake",
                &format!(
                    "{}#",
                    conf.get_nix_flake_path()
                        .to_str()
                        .ok_or(CommandRepoErr::ErrorInMakingString)?
                ),
            ])
            .stdout(std::process::Stdio::piped())
            // .stderr(std::process::Stdio::piped())
            .spawn()
            .map_err(|e| CommandRepoErr::ErrorRunningCommad(e.to_string()))?
            .wait()
            .map_err(|e| CommandRepoErr::ErrorWhileWaiting(e.to_string()))?,

        CommandOption::Update => Command::new("nix")
            .args(vec![
                "flake",
                "update",
                &format!(
                    "{}",
                    conf.get_nix_flake_path()
                        .to_str()
                        .ok_or(CommandRepoErr::ErrorInMakingString)?
                ),
            ])
            .stdout(std::process::Stdio::piped())
            // .stderr(std::process::Stdio::piped())
            .spawn()
            .map_err(|e| CommandRepoErr::ErrorRunningCommad(e.to_string()))?
            .wait()
            .map_err(|e| CommandRepoErr::ErrorWhileWaiting(e.to_string()))?,
    };

    Ok(())
}

pub enum CommandOption {
    Swtich,
    Update,
}

#[derive(Debug, thiserror::Error)]
pub enum CommandRepoErr {
    #[error("error getting user configurations:{0}")]
    NoConfigCouldBeMade(#[from] config::GetInfoErr),

    #[error("error turning path into string")]
    ErrorInMakingString,

    #[error("error running command")]
    ErrorRunningCommad(String),

    #[error("could not run command")]
    ErrorWhileWaiting(String),
}
