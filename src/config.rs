use std::{env::var, path};

use inquire::{validator, Confirm, Text};
use thiserror::Error;

#[derive(Debug, Clone)]
pub struct Config {
    should_flake_update: bool,
    should_switch: bool,
    should_proxy: bool,
    proxy_port: u64,
    nix_flake_path: path::PathBuf,
    should_commit: bool,
    commit_message: String,
    should_push: bool,
}

#[derive(Debug, Error)]
pub enum ConfigBuildErr {
    #[error("could not find home directory")]
    HomeErr(#[from] FindHomeErr),
}

fn get_home() -> Result<path::PathBuf, FindHomeErr> {
    let home = path::PathBuf::from(var("HOME")?);
    if !home.exists() {
        return Err(FindHomeErr::CouldNotFindHome);
    }
    Ok(home)
}

#[derive(Debug, Error)]
pub enum FindHomeErr {
    #[error("could not find HOME variable: {0}")]
    NoHomeVar(#[from] std::env::VarError),
    #[error("could not find home directory")]
    CouldNotFindHome,
}

impl Config {
    pub fn build() -> Result<Self, ConfigBuildErr> {
        let h = get_home()?;
        let config = Config {
            should_commit: true,
            commit_message: String::from("feat(): make daily changes"),
            should_push: true,
            should_flake_update: false,
            should_switch: true,
            should_proxy: false,
            proxy_port: 1080,
            nix_flake_path: path::PathBuf::from(h.join("willow")),
        };
        Ok(config)
    }

    pub fn set_flake_update(&mut self, shoud_it: bool) -> &mut Self {
        self.should_flake_update = shoud_it;
        return self;
    }
    pub fn set_should_commit(&mut self, shoud_it: bool) -> &mut Self {
        self.should_commit = shoud_it;
        return self;
    }
    pub fn set_should_push(&mut self, shoud_it: bool) -> &mut Self {
        self.should_push = shoud_it;
        return self;
    }
    pub fn set_commit_message(&mut self, message: &str) -> &mut Self {
        self.commit_message = String::from(message);
        return self;
    }

    pub fn set_should_switch(&mut self, shoud_it: bool) -> &mut Self {
        self.should_switch = shoud_it;
        return self;
    }

    pub fn set_should_proxy(&mut self, shoud_it: bool) -> &mut Self {
        self.should_proxy = shoud_it;
        return self;
    }

    pub fn set_proxy_port(&mut self, port: u64) -> &mut Self {
        self.proxy_port = port;
        return self;
    }

    pub fn set_nix_flake_path(&mut self, path_name: String) -> Result<(), SetNixPathErr> {
        let h = get_home()?;
        let buf = h.join(path_name);
        if !buf.exists() {
            return Err(SetNixPathErr::PathNotFound(buf));
        }

        self.nix_flake_path = buf;

        Ok(())
    }
    pub fn get_should_flake_update(&self) -> bool {
        self.should_flake_update
    }

    pub fn get_commit_message(&self) -> String {
        self.commit_message.to_owned()
    }

    pub fn get_should_commit(&self) -> bool {
        self.should_commit
    }

    pub fn get_should_push(&self) -> bool {
        self.should_push
    }

    pub fn get_should_switch(&self) -> bool {
        self.should_switch
    }

    pub fn get_should_proxy(&self) -> bool {
        self.should_proxy
    }

    pub fn get_proxy_port(&self) -> u64 {
        self.proxy_port
    }

    pub fn get_nix_flake_path(&self) -> path::PathBuf {
        self.nix_flake_path.to_owned()
    }
}

#[derive(Error, Debug)]
pub enum SetNixPathErr {
    #[error("error finding the given path: {0}")]
    PathNotFound(path::PathBuf),
    #[error("error getting home path: {0}")]
    HomePathErr(#[from] FindHomeErr),
}

#[derive(Error, Debug)]
pub enum GetInfoErr {
    #[error("error building config: {0}")]
    BuildErr(#[from] ConfigBuildErr),

    #[error("error getting home path: {0}")]
    HomePathErr(#[from] FindHomeErr),

    #[error("error getting home path: {0}")]
    NixPathErr(#[from] SetNixPathErr),
}

pub fn get_user_info() -> Result<Config, GetInfoErr> {
    let mut c = Config::build()?;
    c.set_flake_update(
        Confirm::new("Update Flake?")
            .with_default(false)
            .prompt()
            .unwrap(),
    );

    c.set_should_switch(Confirm::new("Switch?").with_default(true).prompt().unwrap());

    c.set_should_proxy(
        Confirm::new("use proxy?")
            .with_default(false)
            .prompt()
            .unwrap(),
    );

    if c.get_should_proxy() == true {
        let buf = Text::new("Port:").with_default("1080").prompt().unwrap();
        let res = buf.parse::<u64>().map_or_else(|e| panic!("{}", e), |v| v);
        c.set_proxy_port(res);
    };

    c.set_nix_flake_path(
        Text::new("Flake Path:")
            .with_default("dofi")
            .with_validator(|input: &str| match get_home()?.join(input).is_dir() {
                true => Ok(validator::Validation::Valid),
                false => Ok(validator::Validation::Invalid(
                    validator::ErrorMessage::Custom(String::from(format!(
                        "file does not exist: {}",
                        get_home()?.join(input).to_str().unwrap()
                    ))),
                )),
            })
            .prompt()
            .unwrap(),
    )?;

    c.set_should_commit(Confirm::new("Commit?").with_default(true).prompt().unwrap());

    if c.get_should_commit() {
        c.set_commit_message(
            &Text::new("commit message:")
                .with_default("feat(): make quick daily changes")
                .prompt()
                .unwrap(),
        );
    };

    c.set_should_push(Confirm::new("push?").with_default(true).prompt().unwrap());

    Ok(c)
}
