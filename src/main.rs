mod repo;
use std::{env, fs, path::Path};

use repo::GetRepo;

fn main() {
    let command = std::env::args().nth(1).expect("no command given");
    let path = std::env::args().nth(2).unwrap_or(".".to_string());
    match command.as_str() {
        "init" => {
            cmd_init(path.as_str());
        }
        _ => {
            println!("Unknown command");
        }
    }

    fn cmd_init(path: &str) {
        repo_create(path);
    }
}

fn repo_create(path: &str) -> () {
    let worktree = Path::new(path);
    let gitdir_str = format!("{}/.git", path);
    let gitdir = Path::new(gitdir_str.as_str());
    if worktree.exists() {
        if !worktree.is_dir() {
            println!("{} is not a directory!", path);
            return;
        }
        if gitdir.exists()
            && gitdir
                .read_dir()
                .map(|mut i| i.next().is_none())
                .unwrap_or(false)
        {
            println!("{} is not empty!", path);
            return;
        }
    } else {
        match fs::create_dir_all(worktree) {
            Ok(_) => {}
            Err(_) => println!("Failed to create directory"),
        }
    }

    println!("{}", gitdir_str);

    repo::repo_dir(gitdir_str.as_str(), "branches", true).unwrap();
    repo::repo_dir(gitdir_str.as_str(), "objects", true).unwrap();
    repo::repo_dir(gitdir_str.as_str(), "refs/tags", true).unwrap();
    repo::repo_dir(gitdir_str.as_str(), "refs/heads", true).unwrap();

    let repo = repo::GetRepo::new(path, true);
}
