use super::command::Command;
use super::error::CommandError;
use std::fs;

pub struct External {}

impl External {
    pub fn find(name: &str, paths: &Vec<&str>) -> Option<String> {
        for &path in paths.iter() {
            if let Ok(dir) = fs::read_dir(path) {
                for f in dir {
                    let path_buf = f.unwrap().path();
                    let file_path = path_buf.to_str().unwrap().split('/').collect::<Vec<_>>();

                    if file_path[file_path.len() - 1] == name {
                        return Some(path_buf.to_str().unwrap().to_string());
                    }
                }
            }
        }

        None
    }
}

//TODO: err
impl Command for External {
    fn run(&self, args: &Vec<&str>, paths: &Vec<&str>) {
        if let Some(command) = Self::find(args[0], &paths) {
            let output = String::from_utf8(
                std::process::Command::new(command)
                    .args(args.iter().skip(1))
                    .output()
                    .unwrap()
                    .stdout,
            )
            .unwrap();
            print!("{}", output);
        } else {
            println!("{}: {}", args[0], CommandError::NotFoundError)
        }
    }
}
