
use std::io;
use std::fs::{self, DirEntry};
use std::path::Path;
use util;

pub struct Node<'a> {
    pub name: String,
    pub is_dir: bool,
    pub is_file: bool,
    child: Vec<Box<Node<'a>>>,
    parent: Option<Box<Node<'a>>>,
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

    let node = Node {
        name: String::new(),
        is_file: false,
        is_tree: true,
        child: Vec::new(),
        parent: None,
        path: String::new()
    };

    Node::new(String::from(entry.path().to_str().unwrap()), Some(parent_node), Box::new(node), is_file).unwrap();

    Ok(())
}

impl<'a> Node<'a> {
    pub fn new(dir: String, mut parent: Option<Box<Node<'a>>>, mut node: Box<Node<'a>>, is_file: bool) -> Result<(), ()> {
        let mut uw_parent = parent.unwrap();

        node.name = util::path::basename(dir.clone());
        node.is_file = is_file;
        node.is_dir = !is_file;
        node.path = dir;

        // ! 
        // how we use uw_parent here,
        // and use it in match ?
        node.parent = Some(uw_parent);

        match uw_parent {
            None => {},
            Some(_) => uw_parent.push(node),
        }

        // done here
        if is_file {
            return Ok(());
        }

        // if is dir 
        // read dir and create the tree 
        let path = Path::new(&dir);
        visit_dirs(&path, &callback, uw_parent);

        return Ok(());
    }

    pub fn push(&mut self, node: Box<Node<'a>>) {
        self.child.push(node);
    }

    pub fn children(&self) -> Vec<Box<Node<'a>>> {
        return self.child;
    }
}

impl<'a> Tree<'a> {
    pub fn search(&self, path: String) -> Option<Node<'a>> {
        self.fresh();

        None
    }

    pub fn fresh(&mut self) {
        let node = Node {
            name: String::new(),
            is_file: false,
            is_tree: true,
            child: Vec::new(),
            parent: None,
            path: String::new()
        };
        self.tree = Node::new(self.base, None, Box::new(node), false).ok();
    }
}


fn visit_dirs<'a> (dir: &Path, cb: &Fn(&DirEntry, &'a mut Node<'a>) -> io::Result<()>, node: &'a mut Node<'a>) -> io::Result<()> {
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
