use std::{
    cell::RefCell,
    collections::HashMap,
    fs,
    io::{BufRead, BufReader},
    ops::Deref,
    rc::{Rc, Weak},
};

#[derive(Debug)]
struct Dir {
    parent: Option<Weak<RefCell<Dir>>>,
    children: HashMap<String, Node>,
}

impl Dir {
    fn new(parent: Option<Weak<RefCell<Dir>>>) -> Self {
        Self {
            parent,
            children: HashMap::new(),
        }
    }
}

#[derive(Debug)]
struct File {
    size: usize,
}

impl File {
    fn new(size: usize) -> Self {
        Self { size }
    }
}

#[derive(Debug)]
enum Node {
    Dir(Rc<RefCell<Dir>>),
    File(Rc<RefCell<File>>),
}

const THRESHOLD: usize = 100000;
const DISK_SIZE: usize = 70000000;
const UPDATE_SIZE: usize = 30000000;

fn main() {
    let f = fs::File::open("input/day7.txt").unwrap();
    let read = BufReader::new(f);

    let root = Rc::new(RefCell::new(Dir::new(None)));
    let mut cur = Rc::downgrade(&root);

    let mut lines = read.lines().peekable();
    loop {
        let Some(Ok(line)) = lines.next() else {
            break;
        };

        let mut toks = line.split(' ');
        assert_eq!(toks.next(), Some("$"));
        match toks.next() {
            Some("cd") => {
                match toks.next() {
                    Some("/") => {
                        // back to root
                        cur = Rc::downgrade(&root);
                    }
                    Some("..") => {
                        // up one level
                        cur = cur
                            .upgrade()
                            .unwrap()
                            .borrow()
                            .parent
                            .as_ref()
                            .cloned()
                            .unwrap_or_else(|| Rc::downgrade(&root));
                    }
                    Some(dir) => {
                        // down one level
                        let _cur = cur.upgrade().unwrap();
                        let _cur = _cur.borrow_mut();
                        let node = _cur.children.get(dir).expect("cd: inexistent");
                        match node {
                            Node::Dir(d) => cur = Rc::downgrade(d),
                            _ => panic!("Not a dir"),
                        }
                    }
                    None => panic!("cd: missing argument"),
                }
            }
            Some("ls") => loop {
                let Some(Ok(line)) = lines.next_if(|l| {
                    !l.as_ref().unwrap().starts_with("$ ")
                }) else {
                    break;
                };
                let mut toks = line.split(' ');
                match toks.next() {
                    Some("dir") => {
                        let Some(dir) = toks.next() else {
                            panic!("dir entry not well formatted");
                        };
                        let node = Node::Dir(Rc::new(RefCell::new(Dir::new(Some(cur.clone())))));
                        cur.upgrade()
                            .unwrap()
                            .borrow_mut()
                            .children
                            .insert(dir.to_string(), node);
                    }
                    Some(size) => {
                        let size: usize = size.parse().expect("file size not an integer");
                        let Some(file) = toks.next() else {
                            panic!("file entry: no name");
                        };
                        let node = Node::File(Rc::new(RefCell::new(File::new(size))));
                        cur.upgrade()
                            .unwrap()
                            .borrow_mut()
                            .children
                            .insert(file.to_string(), node);
                    }
                    None => panic!("unknown entry type"),
                }
            },
            Some(_) => panic!("unknown command"),
            None => panic!("no command given"),
        }
    }

    let mut dirsizes: Vec<usize> = vec![];
    let root_size = dir_size(root.borrow(), &mut dirsizes);

    let free_space = DISK_SIZE - root_size;
    let min_to_free = UPDATE_SIZE - free_space;

    let tot_size_below_thresh: usize = dirsizes.iter()
        .filter(|&&s| s <= THRESHOLD)
        .sum();

    let min_size_to_del = dirsizes.iter()
        .filter(|&&s| s >= min_to_free)
        .min()
        .unwrap_or(&0);

    println!("{}", tot_size_below_thresh);
    println!("{}", min_size_to_del);
}

fn dir_size<D: Deref<Target = Dir>>(dir: D, dirsizes: &mut Vec<usize>) -> usize {
    let mut dirsize: usize = 0;
    for (_, node) in dir.children.iter() {
        match node {
            Node::File(f) => dirsize += f.borrow().size,
            Node::Dir(d) => dirsize += dir_size(d.borrow(), dirsizes),
        }
    }
    dirsizes.push(dirsize);
    dirsize
}
