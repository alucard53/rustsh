use super::{
    command::Command,
    error::{ArgParseError, CommandError},
    external::External,
    trie::Trie,
};
use std::env;

trait Internal {
    type T;
    fn arg_parse(args: &Vec<&str>) -> Result<Self::T, ArgParseError>;
}

pub struct Type {
    trie: Trie,
}

impl Type {
    pub fn new(trie: Trie) -> Self {
        return Self { trie };
    }
}

impl Internal for Type {
    type T = String;

    fn arg_parse(args: &Vec<&str>) -> Result<Self::T, ArgParseError> {
        if args.len() > 2 {
            return Err(ArgParseError::TooManyArguments);
        }
        if args.len() < 2 {
            return Err(ArgParseError::MissingArgs("Command"));
        }

        Ok(String::from(args[1]))
    }
}

impl Command for Type {
    fn run(&self, args: &Vec<&str>, paths: &Vec<&str>) {
        match Self::arg_parse(args) {
            Ok(c_name) => {
                if c_name == "type" || self.trie.find(&c_name).is_some() {
                    println!("{c_name} is a shell builtin")
                } else if let Some(path) = External::find(&c_name, paths) {
                    println!("{c_name} is {path}")
                } else {
                    println!("{c_name}: not found");
                }
            }
            Err(e) => println!("{e}"),
        }
    }
}

pub struct CD {
    home: String,
}

impl CD {
    pub fn new() -> CD {
        return CD {
            home: env::var("HOME").unwrap(),
        };
    }
}

impl Internal for CD {
    type T = String;

    fn arg_parse(args: &Vec<&str>) -> Result<Self::T, ArgParseError> {
        if args.len() > 2 {
            return Err(ArgParseError::TooManyArguments);
        }

        if args.len() == 1 {
            return Ok("~".into());
        }

        Ok(args[1].into())
    }
}

impl Command for CD {
    fn run(&self, args: &Vec<&str>, _: &Vec<&str>) {
        match Self::arg_parse(args) {
            Ok(mut dir) => {
                if dir == "~" {
                    dir = self.home.clone();
                }
                if let Err(e) = env::set_current_dir(&dir) {
                    println!("{e}");
                }
            }
            Err(e) => println!("{e}"),
        }
    }
}

pub struct Skip {}

impl Command for Skip {
    fn run(&self, _: &Vec<&str>, _: &Vec<&str>) {}
}

// TODO quote thingy
pub struct Echo {}

impl Internal for Echo {
    type T = String;
    fn arg_parse(args: &Vec<&str>) -> Result<Self::T, ArgParseError> {
        if args.len() == 1 {
            return Ok("".into());
        }
        Ok(args[1..].join(""))
    }
}

impl Command for Echo {
    fn run(&self, args: &Vec<&str>, _: &Vec<&str>) {
        let out = Self::arg_parse(args)
            .expect("arg_parse for Echo doesn't return error, htf did this happen");
        println!("{out}");
    }
}

pub struct Pwd {}

impl Internal for Pwd {
    type T = ();
    fn arg_parse(args: &Vec<&str>) -> Result<Self::T, super::error::ArgParseError> {
        if args.len() == 1 && args[0] == "pwd" {
            return Ok(());
        }
        Err(super::error::ArgParseError::TooManyArguments)
    }
}

impl Command for Pwd {
    fn run(&self, args: &Vec<&str>, _: &Vec<&str>) {
        match Self::arg_parse(args) {
            Ok(_) => println!("{}", env::current_dir().unwrap().to_str().unwrap()),
            Err(e) => println!("{}: {}", args[0], (CommandError::InvalidArgsError(e))),
        }
    }
}
