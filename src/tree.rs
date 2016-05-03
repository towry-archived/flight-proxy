
use std::io;
use std::fs::{self, DirEntry};
use std::path::Path;
use util;

pub struct Node {
    pub name: String,
    pub is_dir: bool,
    pub is_file: bool,
    child: Vec<Node>,
    parent: Option<Node>,
    pub path: String
}

impl Node {
    pub fn new(dir: String, parent: Option<Node>, is_file: bool) -> Result<Node, _> {
        if is_file {
            let node = Node {
                name: util::path::basename(dir),
                is_file: is_file,
                is_dir: false,
                path: dir
            }

            if parent.is_some() {
                parent.unwrap().push(node);
            }

            return Ok(node);
        }

        // if is dir 
        let parentNode = Node {
            name: util::path::basename(dir),
            is_dir: true,
            is_file: false,
            path: dir
        }

        if parent.is_some() {
            parent.unwrap().push(parentNode);
        }

        // read dir and create the tree 
        let path = Path::new(dir);
        visit_dirs(&path, move |entry| {
            let is_file: bool;

            if try!(fs::metadata(entry.path()).is_file()) {
                is_file = true;
            } else {
                is_file = false;
            }

            Node::new(String::from(entry.path().to_str().unwrap()), Some(parentNode), is_file).unwrap();
        });

        return Ok(parentNode);
    }

    pub fn push(&mut self, Node) {
        self.child.push(Node);
    }

    pub fn children(&self) -> Vec<Node> {
        return self.child;
    }
}


fn visit_dirs(dir: &Path, cb: &Fn(&DirEntry)) -> io::Result<()> {
    if try!(fs::metadata(dir)).is_dir() {
        for entry in try!(fs::read_dir(dir)) {
            let entry = try!(entry);
            cb(&entry);
        }
    }
    Ok(())
}

