mod commands;
mod config;
mod git;
mod privilaged;

use commands::*;
use crossterm::style::Stylize;
use std::error::Error;

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
        let mut sp =
            spinners::Spinner::new(spinners::Spinners::Dots2, String::from("Updating flake..."));

        commands_runner(&conf, CommandOption::Update)?;

        sp.stop_with_message(String::from(format!(
            "{}",
            "✔ Done updating flake.".on_green()
        )));
    }

    // do we need to switch to the new build
    if conf.get_should_switch() {
        let mut sp = spinners::Spinner::with_timer(
            spinners::Spinners::Dots2,
            String::from("Building and switching to the new build..."),
        );
        commands_runner(&conf, CommandOption::Swtich)?;
        sp.stop_with_message(String::from(format!("{}", "✔ Done switching.".on_green())));
    }

    // do we need to make commits
    if conf.get_should_commit() {
        match git::run(git::Run::AddAll(
            conf.get_nix_flake_path(),
            conf.get_commit_message(),
        )) {
            Ok(_) => (),
            Err(git::RunErr::AddAllErr(git::AddAllErr::NoFileChange)) => {
                println!(
                    "{}",
                    "✔ No file has been changed. So I think we don't need a new commit.".on_green()
                )
            }
            Err(e) => return Err(Box::new(e)),
        };
    }

    // do we need to push commits to remote
    if conf.get_should_push() {
        git::run(git::Run::Push(conf.get_nix_flake_path()))?;
    }

    Ok(())
}
