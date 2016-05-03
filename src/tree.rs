
use std::io;
use std::fs::{self, DirEntry};
use std::path::Path;
use util;

pub struct Node {
    pub name: String,
    pub is_dir: bool,
    pub is_file: bool,
    child: Vec<Node>,
    parent: &Option<Box<Node>>,
    pub path: String
}

pub struct Tree {
    base: String,
    tree: Option<Node>
}

fn callback(entry: &DirEntry, parentNode: Node) -> io::Result<()> {
    let is_file: bool;

    // unwrap 
    if try!(fs::metadata(entry.path())).is_file() {
        is_file = true;
    } else {
        is_file = false;
    }

    Node::new(String::from(entry.path().to_str().unwrap()), Some(Box::new(parentNode)), is_file).unwrap();

    Ok(())
}

impl Node {
    pub fn new(dir: String, parent: Option<Box<Node>>, is_file: bool) -> Result<Node, ()> {
        if is_file {
            let node = Node {
                name: util::path::basename(dir.clone()),
                is_file: is_file,
                is_dir: false,
                path: dir,
                parent: &parent,
                child: Vec::new()
            };

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
            path: dir,
            parent: &parent,
            child: Vec::new()
        };

        if parent.is_some() {
            parent.unwrap().push(parentNode);
        }

        // read dir and create the tree 
        let path = Path::new(&dir);
        visit_dirs(&path, &callback, parentNode);

        return Ok(parentNode);
    }

    pub fn push(&mut self, node: Node) {
        self.child.push(node);
    }

    pub fn children(&self) -> Vec<Node> {
        return self.child;
    }
}

impl Tree {
    pub fn search(&self, path: String) -> Option<Node> {
        self.fresh();

        None
    }

    pub fn fresh(&mut self) {
        self.tree = Node::new(self.base, None, false).ok();
    }
}


fn visit_dirs(dir: &Path, cb: &Fn(&DirEntry, Node) -> io::Result<()>, node: Node) -> io::Result<()> {
    if try!(fs::metadata(dir)).is_dir() {
        for entry in try!(fs::read_dir(dir)) {
            let entry = try!(entry);
            cb(&entry, node);
        }
    }
    Ok(())
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tree_fresh() {
        let tree = Tree {
            base: String::from("/Users/towry/Projects/mobile-flight/build"),
            tree: None
        };

        tree.fresh();
    }
}
