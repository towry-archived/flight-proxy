
use std::io;
use std::fs::{self, DirEntry};
use std::path::Path;
use util;

pub struct Node<'a> {
    pub name: String,
    pub is_dir: bool,
    pub is_file: bool,
    child: Vec<&'a Box<Node<'a>>>,
    parent: Option<&'a Node<'a>>,
    pub path: String
}

pub struct Tree<'a> {
    base: String,
    tree: Option<Box<Node<'a>>>
}

fn callback<'a>(entry: &DirEntry, parent_node: &'a mut Node<'a>) -> io::Result<()> {
    let is_file: bool;

    // unwrap 
    if try!(fs::metadata(entry.path())).is_file() {
        is_file = true;
    } else {
        is_file = false;
    }

    Node::new(String::from(entry.path().to_str().unwrap()), Some(parent_node), is_file).unwrap();

    Ok(())
}

impl<'a> Node<'a> {
    pub fn new(dir: String, parent: Option<&'a Node<'a>>, is_file: bool) -> Result<Box<Node<'a>>, ()> {
        let uw_parent = parent.unwrap();

        if is_file {
            let mut node = Node {
                name: util::path::basename(dir.clone()),
                is_file: is_file,
                is_dir: false,
                path: dir,
                parent: Some(uw_parent),
                child: Vec::new()
            };

            let node_ref = Box::new(node);
            uw_parent.push(&node_ref);

            // this is not right
            return Ok(node_ref);
        }

        // if is dir 
        let mut parent_node = Node {
            name: util::path::basename(dir),
            is_dir: true,
            is_file: false,
            path: dir,
            parent: Some(uw_parent.clone()),
            child: Vec::new()
        };

        let parent_node_ref = Box::new(parent_node);

        if parent.is_some() {
            uw_parent.push(&parent_node_ref);
        }

        // read dir and create the tree 
        let path = Path::new(&dir);
        visit_dirs(&path, &callback, &mut *parent_node_ref);

        return Ok(parent_node_ref);
    }

    pub fn push(&mut self, node: &Box<Node>) {
        self.child.push(node);
    }

    pub fn children(&self) -> Vec<Node> {
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


fn visit_dirs (dir: &Path, cb: &Fn(&DirEntry, &mut Node) -> io::Result<()>, node: &mut Node) -> io::Result<()> {
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
