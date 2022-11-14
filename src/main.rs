mod commands;
mod config;
mod git;
mod privilaged;

use commands::*;
use std::{error::Error, path};

fn main() -> Result<(), Box<dyn Error>> {
    // this block will check if the executible has sudo privilages.
    // if case it doesn't it will run a child process of it self.
    // and retusn Ok() itself.
    if !privilaged::am_sudo() {
        privilaged::this_program(privilaged::Privilager::Sudo)?;
        return Ok(());
    };

    // checks and gets configurations
    let conf = config::get_user_info()?;

    // ----- using configurations we decide what needs to be done --- //

    // do we need to update the flake
    if conf.get_should_flake_update() {
        commands_runner(&conf, CommandOption::Update)?;
    }

    // do we need to switch to the new build
    if conf.get_should_switch() {
        commands_runner(&conf, CommandOption::Swtich)?;
    }

    // do we need to make commits
    if conf.get_should_commit() {
        git::run(git::Run::AddAll(
            conf.get_nix_flake_path(),
            conf.get_commit_message(),
        ))?;
    }

    // do we need to push commits to remote
    if conf.get_should_push() {
        // git::run(git::Run::AddAll(conf.get_nix_flake_path()))?;
    }

    Ok(())
}
