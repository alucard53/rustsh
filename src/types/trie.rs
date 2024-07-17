use super::{command::Command, inbuilt};
use std::collections::HashMap;

struct Node {
    command: Option<Box<dyn Command>>,
    children: [Option<Box<Node>>; 26],
}

pub struct Trie {
    root: Box<Node>,
}

const NONE: Option<Box<Node>> = None;

impl Trie {
    pub fn new(with_type: bool) -> Trie {
        let mut t = Self {
            root: Box::new(Node {
                command: Some(Box::new(inbuilt::Skip {})),
                children: [NONE; 26],
            }),
        };
        let mut comm_map: HashMap<&str, Box<dyn Command>> = HashMap::new();
        comm_map.insert("pwd", Box::new(inbuilt::Pwd {}));
        comm_map.insert("echo", Box::new(inbuilt::Echo {}));
        comm_map.insert("cd", Box::new(inbuilt::CD::new()));

        if with_type {
            comm_map.insert("type", Box::new(inbuilt::Type::new(Trie::new(false))));
        }

        for (c_name, c_struct) in comm_map {
            t.feed(c_name, c_struct);
        }
        t
    }

    pub fn feed(&mut self, c_name: &str, c_struct: Box<dyn Command>) {
        let chars = c_name.chars().collect::<Vec<_>>();

        fn insert(idx: usize, chars: Vec<char>, root: &mut Box<Node>, c_struct: Box<dyn Command>) {
            let i = chars[idx] as usize - 97;

            if i == chars.len() {
                root.command = Some(c_struct);
                return;
            }

            if let Some(mut child) = root.children[i].take() {
                insert(idx + 1, chars, &mut child, c_struct);
                root.children[i] = Some(child);
            } else {
                let mut new = Box::new(Node {
                    command: None,
                    children: [NONE; 26],
                });
                insert(idx + 1, chars, &mut new, c_struct);
                root.children[i] = Some(new);
            }
        }

        insert(0, chars, &mut self.root, c_struct);
    }

    pub fn find(&self, c_name: &str) -> Option<&Box<dyn Command>> {
        let chars = c_name.chars().collect::<Vec<_>>();

        fn fi<'a>(
            idx: usize,
            chars: Vec<char>,
            root: &'a Box<Node>,
        ) -> Option<&'a Box<dyn Command>> {
            let i = chars[idx] as usize - 97;

            if i == chars.len() && root.command.is_some() {
                return root.command.as_ref();
            }

            if let Some(child) = root.children[i].as_ref() {
                return fi(idx + 1, chars, child);
            }

            None
        }

        fi(0, chars, &self.root)
    }
}
