use std::process::Command;

use super::config;

pub fn commands_runner(conf: &config::Config, c: CommandOption) -> Result<(), CommandRepoErr> {
    match c {
        CommandOption::Swtich => {
            let child = Command::new("nixos-rebuild")
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
                .stderr(std::process::Stdio::piped())
                .output()
                .map_err(|e| CommandRepoErr::ErrorRunningCommad(e.to_string()))?;

            let e = String::from_utf8_lossy(&child.stderr);
            println!("{}", e);

            if !child.status.success() {
                return Err(CommandRepoErr::NonZeroStatus);
            };
        }

        CommandOption::Update => {
            let mut child = Command::new("nix")
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
                .stderr(std::process::Stdio::piped())
                .spawn()
                .map_err(|e| CommandRepoErr::ErrorRunningCommad(e.to_string()))?;

            let st = child
                .wait()
                .map_err(|e| CommandRepoErr::ErrorWhileWaiting(e.to_string()))?;

            if !st.success() {
                return Err(CommandRepoErr::NonZeroStatus);
            };
        }
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

    #[error("Command exited with non-0 value")]
    NonZeroStatus,
}
