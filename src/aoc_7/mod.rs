use lazy_static::lazy_static;
use regex::Regex;
use std::{
    cell::RefCell,
    fmt::Display,
    rc::{Rc, Weak},
};
use TreeNode::{File, Folder};

use crate::get_input;

type TreeRef = Rc<RefCell<Tree>>;
type WeakTreeRef = Weak<RefCell<Tree>>;

#[derive(Debug)]
struct Tree {
    size: u64,
    name: String,
    node_type: TreeNode,
    parent: Option<WeakTreeRef>,
}

#[derive(Debug)]
enum TreeNode {
    Folder { children: Vec<TreeRef> },
    File,
}

impl Tree {
    fn parent(node: TreeRef) -> TreeRef {
        match &node.as_ref().borrow().parent {
            Some(p) => p.upgrade().unwrap(),
            None => node.clone(),
        }
    }

    fn is_dir(&self) -> bool {
        if let Folder { children: _ } = self.node_type {
            true
        } else {
            false
        }
    }

    fn add_child(&mut self, tree: TreeRef) {
        match &mut self.node_type {
            Folder { children } => children.push(tree),
            _ => unreachable!(),
        }
    }

    fn find_child_folder(&self, folder_name: &str) -> Option<TreeRef> {
        match &self.node_type {
            Folder { children } => Tree::find_in_children(children, folder_name),
            _ => unreachable!(),
        }
    }

    fn find_in_children(children: &Vec<TreeRef>, folder_name: &str) -> Option<TreeRef> {
        children
            .iter() // iterate over children, easy
            .filter(|c| {
                let c = c.borrow();
                c.is_dir() && c.name == folder_name
            })
            .map(|c| c.clone()) //we send back a reference to the child but want also to keep the reference inside the tree > clone the rc
            .next() // should be only one result, so iter.next should give it
    }

    fn compute_size(tree: TreeRef) -> u64 {
        let mut update = false;
        let compute = match &tree.borrow().node_type {
            Folder { children } => {
                update = true;
                children
                    .iter()
                    .fold(0, |acc, c| acc + Tree::compute_size(c.clone()))
            }
            File => tree.borrow().size,
        };

        if update {
            tree.borrow_mut().size = compute;
        }

        compute
    }

    fn print_with_indent(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        indent: usize,
    ) -> std::fmt::Result {
        let Tree {
            name,
            size,
            node_type: _,
            parent: _,
        } = self;
        match &self.node_type {
            Folder { children } => {
                write!(f, "{:indent$}- {} (dir, size={})\n", "", name, size)?;
                for child in children {
                    let child = child.borrow();
                    child.print_with_indent(f, indent + 2)?;
                }
                Ok(())
            }
            File => {
                write!(f, "{:indent$}- {} (file, size={})\n", "", name, size)
            }
        }
    }

    fn visit<F: FnMut(TreeRef)>(tree: TreeRef, f: &mut F) {
        f(tree.clone());
        match &tree.borrow().node_type {
            Folder { children } => {
                for c in children {
                    Self::visit(c.clone(), f);
                }
            }
            _ => (),
        }
    }

    fn find_total_aoc_7_1(tree: TreeRef) -> u64 {
        let mut total_folder = 0;

        let mut visitor = |t: TreeRef| {
            let t = t.borrow();

            if t.is_dir() && t.size <= 100000 {
                println!("found folder {}", t.name);
                total_folder += t.size;
            }
        };

        Tree::visit(tree, &mut visitor);
        total_folder
    }

    fn find_min_dir_aoc_7_2(tree: TreeRef) -> u64 {
        let available = available_space(tree.clone());

        let target = 30000000 - available;

        assert!(target > 0);

        let mut min_dir_size = u64::MAX;
        let mut visitor = |t: TreeRef| {
            let t = t.borrow();
            if t.is_dir() {
                if t.size >= target && t.size < min_dir_size {
                    min_dir_size = t.size;
                }
            }
        };

        Tree::visit(tree, &mut visitor);

        min_dir_size
    }
}

impl Display for Tree {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.print_with_indent(f, 0)
    }
}

fn parse<T: Iterator<Item = String>>(input: T) -> TreeRef {
    lazy_static! {
        static ref CD_REX: Regex = Regex::new(r"\$ cd (?P<folder>[a-zA-Z\.]+)").unwrap();
    }

    let root = Tree {
        size: 0,
        name: "/".to_string(),
        parent: None,
        node_type: TreeNode::Folder { children: vec![] },
    };

    let mut current_node = Rc::new(RefCell::new(root));

    let root_ref = current_node.clone(); //keep a reference to the root so it's never dereferenced

    let mut peekable = input.peekable();

    while let Some(line) = peekable.next() {
        match line.as_str() {
            "$ cd /" => {
                current_node = root_ref.clone();
            }
            "$ cd .." => current_node = Tree::parent(current_node),
            "$ ls" => {
                while let Some(next_line) = peekable.peek() {
                    //look ahead to find if next line is a command or the result of a ls
                    if next_line.starts_with("$") {
                        break;
                    }
                    let next_line = peekable.next().unwrap(); //advance the iterator

                    let new_node = parse_folder_item(&next_line, current_node.clone());
                    let mut current_node = current_node.borrow_mut();
                    current_node.add_child(Rc::new(RefCell::new(new_node)));
                }
            }
            s if CD_REX.is_match(s) => {
                let folder_name = &CD_REX.captures(s).unwrap()["folder"];
                let child = current_node
                    .borrow()
                    .find_child_folder(folder_name)
                    .expect(&format!(
                        "Cannot find {} in {}",
                        folder_name,
                        current_node.borrow()
                    ));
                current_node = child;
            }
            _ => unreachable!(),
        }
    }

    root_ref
}

fn parse_folder_item(line: &String, parent: TreeRef) -> Tree {
    lazy_static! {
        static ref FILE_REX: Regex =
            Regex::new(r"(?P<size>[0-9]+) (?P<name>[a-zA-Z0-9\.]+)").unwrap();
        static ref FOLDER_REX: Regex = Regex::new(r"dir (?P<name>[a-zA-Z0-9]+)").unwrap();
    }

    if let Some(captures) = FOLDER_REX.captures(line) {
        Tree {
            name: captures["name"].to_string(),
            size: 0,
            parent: Some(Rc::<RefCell<Tree>>::downgrade(&parent)),
            node_type: TreeNode::Folder { children: vec![] },
        }
    } else if let Some(captures) = FILE_REX.captures(line) {
        Tree {
            name: captures["name"].to_string(),
            size: captures["size"].parse().unwrap(),
            parent: Some(Rc::<RefCell<Tree>>::downgrade(&parent)),
            node_type: TreeNode::File,
        }
    } else {
        unreachable!()
    }
}

fn available_space(tree: TreeRef) -> u64 {
    let tree = tree.borrow();

    assert!(&tree.parent.is_none());

    70000000 - tree.size
}

pub fn aoc_7_1() {
    let input = get_input("resource/aoc_7/data.txt");

    let tree = parse(input.into_iter());

    Tree::compute_size(tree.clone());

    let total_folder = Tree::find_total_aoc_7_1(tree);

    println!("AOC 7-1 total folder under 100000 {}", &total_folder);
}

pub fn aoc_7_2() {
    let input = get_input("resource/aoc_7/data.txt");

    let tree = parse(input.into_iter());

    Tree::compute_size(tree.clone());

    let min_dir = Tree::find_min_dir_aoc_7_2(tree);

    println!("AOC 7-2 min dir size {}", &min_dir);
}

#[cfg(test)]
mod test {
    use itertools::Itertools;

    use super::*;

    const INPUT: [&'static str; 23] = [
        "$ cd /",
        "$ ls",
        "dir a",
        "14848514 b.txt",
        "8504156 c.dat",
        "dir d",
        "$ cd a",
        "$ ls",
        "dir e",
        "29116 f",
        "2557 g",
        "62596 h.lst",
        "$ cd e",
        "$ ls",
        "584 i",
        "$ cd ..",
        "$ cd ..",
        "$ cd d",
        "$ ls",
        "4060174 j",
        "8033020 d.log",
        "5626152 d.ext",
        "7214296 k",
    ];

    #[test]
    fn test_parse() {
        let iter = INPUT.iter().map(|s| s.to_string());

        let tree = parse(iter);

        let expected = vec![
            "- / (dir, size=0)",
            "  - a (dir, size=0)",
            "    - e (dir, size=0)",
            "      - i (file, size=584)",
            "    - f (file, size=29116)",
            "    - g (file, size=2557)",
            "    - h.lst (file, size=62596)",
            "  - b.txt (file, size=14848514)",
            "  - c.dat (file, size=8504156)",
            "  - d (dir, size=0)",
            "    - j (file, size=4060174)",
            "    - d.log (file, size=8033020)",
            "    - d.ext (file, size=5626152)",
            "    - k (file, size=7214296)\n",
        ]
        .iter()
        .join("\n");
        assert_eq!(tree.borrow().to_string(), expected);
    }

    #[test]
    fn test_compute() {
        let iter = INPUT.iter().map(|s| s.to_string());

        let tree = parse(iter);

        Tree::compute_size(tree.clone());

        let expected = vec![
            "- / (dir, size=48381165)",
            "  - a (dir, size=94853)",
            "    - e (dir, size=584)",
            "      - i (file, size=584)",
            "    - f (file, size=29116)",
            "    - g (file, size=2557)",
            "    - h.lst (file, size=62596)",
            "  - b.txt (file, size=14848514)",
            "  - c.dat (file, size=8504156)",
            "  - d (dir, size=24933642)",
            "    - j (file, size=4060174)",
            "    - d.log (file, size=8033020)",
            "    - d.ext (file, size=5626152)",
            "    - k (file, size=7214296)\n",
        ]
        .iter()
        .join("\n");
        assert_eq!(tree.borrow().to_string(), expected);
    }

    #[test]
    fn test_total_size() {
        let iter = INPUT.iter().map(|s| s.to_string());

        let tree = parse(iter);

        Tree::compute_size(tree.clone());

        let total = Tree::find_total_aoc_7_1(tree.clone());

        assert_eq!(95437, total);
    }

    #[test]
    fn test_min_dir_size() {
        let iter = INPUT.iter().map(|s| s.to_string());

        let tree = parse(iter);

        Tree::compute_size(tree.clone());

        let total = Tree::find_min_dir_aoc_7_2(tree.clone());

        assert_eq!(24933642, total);
    }

    #[test]
    fn test_aoc_7_1() {
        aoc_7_1();
    }

    #[test]
    fn test_aoc_7_2() {
        aoc_7_2();
    }
}
