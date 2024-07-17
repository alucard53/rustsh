mod types;

use std::{
    env,
    io::{self, Write},
};

use types::{command::Command, external::External, trie::Trie};

fn main() {
    let nl = if env::consts::OS == "windows" { 2 } else { 1 };

    let path_var = env::var("PATH").unwrap();
    let paths = path_var.split(":").collect::<Vec<_>>();

    print!("$ ");
    io::stdout().flush().unwrap();

    let stdin = io::stdin();
    let mut input = String::new();

    let trie = Trie::new(true);

    let e = External {};

    while let Ok(_) = stdin.read_line(&mut input) {
        let args = (&input[..input.len() - nl]).split(" ").collect::<Vec<_>>();

        if args[0] == "exit" {
            break;
        }

        if args[0] != "" {
            if let Some(command) = trie.find(args[0]) {
                command.run(&args, &paths);
            } else {
                e.run(&args, &paths);
            }
        }

        input.clear();
        print!("$ ");
        io::stdout().flush().unwrap();
    }
}
