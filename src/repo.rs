use configparser::ini::Ini;
use std::error::Error;
use std::fs;
use std::path::{Path, PathBuf};

pub struct GetRepo {
    worktree: String,
    gitdir: String,
    conf: Ini,
}

impl GetRepo {
    pub fn new(path: &str, force: bool) -> Self {
        let worktree = path.to_string();
        let gitdir = format!("{}/.git", path);

        if !(force || Path::new(&gitdir).is_dir()) {
            panic!("Not a Get Repository {}", path);
            // return Err(format!("Not a Get Repository {}", path).into());
        }

        let mut conf = Ini::new();
        let cf = repo_file(gitdir.as_str(), "config", false).unwrap();

        match cf {
            Some(value) => {
                if Path::new(value.as_str()).exists() {
                    conf.read(value.to_string());
                }
            }
            None => {
                if !force {
                    panic!("Configuration file missing");
                }
            }
        }

        if !force {
            let vers = conf
                .get("core", "repositoryformatversion")
                .unwrap()
                .to_string()
                .parse::<i32>()
                .unwrap();
            if vers != 0 {
                panic!("Unsupported repositoryformatversion {}", vers);
            }
        }

        GetRepo {
            worktree,
            gitdir,
            conf,
        }
    }
}

fn repo_file(gitdir: &str, path: &str, mkdir: bool) -> Result<Option<String>, Box<dyn Error>> {
    let directory = repo_dir(gitdir, path, mkdir);

    match directory {
        Ok(value) => Ok(repo_path(gitdir, path)),
        Err(err) => Err(err),
    }
}

fn repo_path(gitdir: &str, path: &str) -> Option<String> {
    match Path::new(gitdir).join(path).into_os_string().into_string() {
        Ok(value) => Some(value),
        Err(_) => None,
    }
}

pub fn repo_dir(gitdir: &str, path: &str, mkdir: bool) -> Result<Option<String>, Box<dyn Error>> {
    let path_str = repo_path(gitdir, path).unwrap();
    let new_path = Path::new(path_str.as_str());
    if new_path.exists() {
        if new_path.is_dir() {
            return Ok(repo_path(gitdir, path));
        } else {
            return Err(format!("Not a directory {}", path).into());
        }
    }

    if mkdir {
        fs::create_dir_all(path_str);
        Ok(Some(path.to_string()))
    } else {
        Ok(None)
    }
}
