use std::path;

use git2;
use thiserror::Error;

pub enum Run {
    AddAll(path::PathBuf, String),
}

pub fn run(cmd: Run) -> Result<(), RunErr> {
    match cmd {
        Run::AddAll(p, cm) => add_all(p, cm)?,
    };
    Ok(())
}

#[derive(Debug, Error)]
pub enum RunErr {
    #[error("problem runnign add_all inside git: {0}")]
    AddAllErr(#[from] AddAllErr),
}

fn add_all(repo_path: path::PathBuf, commmit_message: String) -> Result<(), AddAllErr> {
    // --- adding --- //

    // first get the repo in-question
    let repo = get_repo(repo_path)?;

    // get the current index of files
    let mut index = repo.index()?;

    // add all files matching the globe
    index.add_all(vec!["*"].iter(), git2::IndexAddOption::default(), None)?;

    // writing the added files into index
    index.write()?;

    // --- creating a diff ---//

    // --- creating a commit --- //

    let hea = repo.head()?;
    let last_commit = repo.reflog(s)?.get(0)?;
    println!("{:#?}", last_commit);

    // let lc = repo.reflog(wt)?.iter().next().unwrap().id_old();
    //
    // let l = repo.commit(
    //     repo.head()?,
    //     &repo.signature()?,
    //     &repo.signature()?,
    //     &commmit_message,
    //     wt,
    //     repo.find_commit(lc)?,
    // );

    Ok(())
}

#[derive(Debug, thiserror::Error)]
pub enum AddAllErr {
    #[error("problem opening git: {0}")]
    GetRepoErr(#[from] GetRepoErr),
    #[error("could not open repo {0}")]
    FailedToOpen(#[from] git2::Error),
}

fn get_repo(repo_path: path::PathBuf) -> Result<git2::Repository, GetRepoErr> {
    Ok(git2::Repository::open(repo_path)?)
}

#[derive(Debug, thiserror::Error)]
pub enum GetRepoErr {
    #[error("could not open repo {0}")]
    FailedToOpen(#[from] git2::Error),
}
