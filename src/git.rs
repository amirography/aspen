use std::path;

use git2;
use thiserror::Error;

pub enum Run {
    AddAll(path::PathBuf, String),
    Push(path::PathBuf),
}

pub fn run(cmd: Run) -> Result<(), RunErr> {
    match cmd {
        Run::AddAll(p, cm) => add_all(p, cm)?,
        Run::Push(p) => push(p)?,
    };
    Ok(())
}

#[derive(Debug, Error)]
pub enum RunErr {
    #[error("problem runnign add_all inside git: {0}")]
    AddAllErr(#[from] AddAllErr),

    #[error("problem pushing: {0}")]
    PushErr(#[from] PushErr),
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

    // make a new tree from the written indexes
    let new_tree = repo.find_tree(index.write_tree()?)?;

    // --- creating a commit --- //

    // get a reflog and from that the last commit
    let reflog = repo.reflog(
        repo.head()?
            .name()
            .ok_or_else(|| AddAllErr::CouldNotRepoHeadName)?,
    )?;

    // find out the last commit so to use it as the parent
    let last_commit = repo.find_commit(
        reflog
            .get(0)
            .ok_or_else(|| AddAllErr::NoLastCommit)?
            .id_new(),
    )?;

    // check if we actually need a new commit
    let diff = repo.diff_tree_to_index(Some(&last_commit.tree()?), None, None)?;

    if diff.stats()?.files_changed() == 0 {
        return Err(AddAllErr::NoFileChange);
    };

    // make a new commit
    repo.commit(
        Some("HEAD"),
        &repo.signature()?,
        &repo.signature()?,
        &commmit_message,
        &new_tree,
        &vec![&last_commit],
    )?;

    Ok(())
}

#[derive(Debug, thiserror::Error)]
pub enum AddAllErr {
    #[error("problem opening git: {0}")]
    GetRepoErr(#[from] GetRepoErr),

    #[error("could not open repo {0}")]
    FailedToOpen(#[from] git2::Error),

    #[error("could not open repo {0}")]
    HomeNotFound(#[from] std::env::VarError),

    #[error("no file change to commit")]
    NoFileChange,

    #[error("could not find the name of the head in the repo")]
    CouldNotRepoHeadName,

    #[error("could not find the last commit")]
    NoLastCommit,
}

fn push(repo_path: path::PathBuf) -> Result<(), PushErr> {
    // for this I used the command directly, because I could not figure out for the life of me,
    // where was the null pointer inside libgit2
    let mut g = std::process::Command::new("git");

    g.args(vec![
        "-C",
        repo_path.to_str().ok_or_else(|| PushErr::ConversionErr)?,
        "push",
        "origin",
        "main",
    ]);

    // running the command
    let mut child = g.spawn()?;
    child.wait()?;

    Ok(())
}

#[derive(Debug, thiserror::Error)]
pub enum PushErr {
    #[error("problem converting the repo path")]
    ConversionErr,

    #[error("problem spawning a child process: {0}")]
    SpawnProblem(#[from] std::io::Error),
}

fn get_repo(repo_path: path::PathBuf) -> Result<git2::Repository, GetRepoErr> {
    Ok(git2::Repository::open(repo_path)?)
}

#[derive(Debug, thiserror::Error)]
pub enum GetRepoErr {
    #[error("could not open repo {0}")]
    FailedToOpen(#[from] git2::Error),
}
