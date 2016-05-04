
use std::io;
use std::fs::{self, DirEntry};
use std::path::Path;
use util;

pub struct Node<'s> {
    pub name: String,
    pub is_dir: bool,
    pub is_file: bool,
    child: Vec<Node<'s>>,
    parent: Option<&'s mut Node<'s>>,
    pub path: String
}

pub struct Tree<'a> {
    base: String,
    tree: Option<Node<'a>>
}

fn callback<'s> (entry: &DirEntry, parentNode: &'s mut Node<'s>) -> io::Result<()> {
    let is_file: bool;

    // unwrap 
    if try!(fs::metadata(entry.path())).is_file() {
        is_file = true;
    } else {
        is_file = false;
    }

    Node::new(String::from(entry.path().to_str().unwrap()), Some(parentNode), is_file).unwrap();

    Ok(())
}

impl<'s> Node<'s> {
    pub fn new(dir: String, parent: Option<&'s mut Node<'s>>, is_file: bool) -> Result<Node<'s>, ()> {
        if is_file {
            let uw_parent = parent.unwrap();

            // node.name = util::path::basename(dir.clone());
            // node.is_file = is_file;
            // node.is_dir = false;
            // node.path = dir;
            // node.parent = Some(uw_parent);

            let mut node = Node {
                name: util::path::basename(dir.clone()),
                is_file: is_file,
                is_dir: false,
                path: dir,
                parent: Some(uw_parent),
                child: Vec::new()
            };

            uw_parent.push(node);

            // this is not right
            return Ok(node);
        }

        // if is dir 
        let mut parentNode = Node {
            name: util::path::basename(dir),
            is_dir: true,
            is_file: false,
            path: dir,
            parent: Some(parent.unwrap()),
            child: Vec::new()
        };

        if parent.is_some() {
            parent.unwrap().push(parentNode);
        }

        // read dir and create the tree 
        let path = Path::new(&dir);
        visit_dirs(&path, &callback, &mut parentNode);

        return Ok(parentNode);
    }

    pub fn push(&mut self, node: Node<'s>) {
        self.child.push(node);
    }

    pub fn children(&self) -> Vec<Node<'s>> {
        return self.child;
    }
}

impl<'a> Tree<'a> {
    pub fn search(&self, path: String) -> Option<Node<'a>> {
        self.fresh();

        None
    }

    pub fn fresh(&mut self) {
        self.tree = Node::new(self.base, None, false).ok();
    }
}


fn visit_dirs<'a> (dir: &Path, cb: &Fn(&DirEntry, &'a mut Node<'a> ) -> io::Result<()>, node: &'a mut Node<'a>) -> io::Result<()> {
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
