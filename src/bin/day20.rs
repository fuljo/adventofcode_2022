use std::{
    cell::RefCell,
    fmt::Display,
    fs::File,
    io::{BufRead, BufReader},
    rc::Rc,
};

type NodePtr<T> = Rc<RefCell<Node<T>>>;

struct Node<T: Sized> {
    value: T,
    prev: Option<NodePtr<T>>,
    next: Option<NodePtr<T>>,
}

impl<T> Node<T> {
    fn new(value: T) -> NodePtr<T> {
        Rc::new(RefCell::new(Self {
            value,
            prev: None,
            next: None,
        }))
    }
}

struct CircleList<T: Sized> {
    nodes: Vec<NodePtr<T>>,
    head: Option<NodePtr<T>>,
}

impl<T> CircleList<T> {
    fn len(&self) -> usize {
        self.nodes.len()
    }
}

impl<T: Sized + PartialEq + Copy + Default> FromIterator<T> for CircleList<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let zero = T::default();
        let mut vec: Vec<NodePtr<T>> = Vec::new();
        let head: NodePtr<T>;
        let mut tail: NodePtr<T>;
        let mut zero_node: Option<NodePtr<T>> = None;

        let mut iter = iter.into_iter();

        // head
        if let Some(value) = iter.next() {
            let node = Node::new(value);
            tail = node.clone();
            head = node.clone();
            if value == zero {
                zero_node = Some(node.clone());
            }
            vec.push(node);
        } else {
            return Self {
                nodes: vec,
                head: None,
            };
        }

        // rest
        for value in iter {
            let node = Node::new(value);
            (*node).borrow_mut().prev = Some(tail.clone());
            (*tail).borrow_mut().next = Some(node.clone());
            tail = node.clone();
            if value == zero {
                zero_node = Some(node.clone());
            }
            vec.push(node);
        }

        // close the list
        (*head).borrow_mut().prev = Some(tail.clone());
        (*tail).borrow_mut().next = Some(head);

        // the list will start from zero
        Self {
            nodes: vec,
            head: zero_node,
        }
    }
}

impl CircleList<isize> {
    fn mix(&self) {
        for node in self.nodes.iter() {
            self.move_node(node);
        }
    }

    fn move_node(&self, node_ptr: &NodePtr<isize>) {
        let mut node = (*node_ptr).borrow_mut();
        let amt = node.value % (self.len() - 1) as isize;
        if amt == 0 {
            return;
        }

        // unlink the node
        let prev_ptr = node.prev.take().unwrap();
        let next_ptr = node.next.take().unwrap();
        (*prev_ptr).borrow_mut().next.replace(next_ptr.clone());
        (*next_ptr).borrow_mut().prev.replace(prev_ptr.clone());

        // move
        if amt > 0 {
            // move forward
            let mut next_ptr = next_ptr;
            for _ in 0..amt.abs() {
                let tmp = (*next_ptr).borrow().next.as_ref().unwrap().clone();
                next_ptr = tmp;
            }
            // insert
            let prev_ptr = (*next_ptr)
                .borrow_mut()
                .prev
                .replace(node_ptr.clone())
                .unwrap();
            let next_ptr = (*prev_ptr)
                .borrow_mut()
                .next
                .replace(node_ptr.clone())
                .unwrap();
            node.next = Some(next_ptr);
            node.prev = Some(prev_ptr);
        } else {
            // move backward
            let mut prev_ptr = prev_ptr;
            for _ in 0..amt.abs() {
                let tmp = (*prev_ptr).borrow().prev.as_ref().unwrap().clone();
                prev_ptr = tmp;
            }
            // insert
            let next_ptr = (*prev_ptr)
                .borrow_mut()
                .next
                .replace(node_ptr.clone())
                .unwrap();
            let prev_ptr = (*next_ptr)
                .borrow_mut()
                .prev
                .replace(node_ptr.clone())
                .unwrap();
            node.next = Some(next_ptr);
            node.prev = Some(prev_ptr);
        }
    }
}

impl<T: Copy> CircleList<T> {
    fn iter(&self) -> CircleListIter<T> {
        CircleListIter {
            cur: self.head.as_ref().unwrap().clone(),
        }
    }
}

impl<T: Display + Copy> Display for CircleList<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut i = self.len() - 1;
        for value in self.iter() {
            if i > 0 {
                write!(f, "{}, ", value)?;
            } else {
                write!(f, "{}", value)?;
                break;
            }
            i -= 1;
        }
        Ok(())
    }
}

struct CircleListIter<T: Sized + Copy> {
    cur: NodePtr<T>,
}

impl<T: Copy> Iterator for CircleListIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        let value = (*self.cur).borrow().value;
        let tmp = (*self.cur).borrow().next.as_ref().unwrap().clone();
        self.cur = tmp;
        Some(value)
    }
}

const DECRYPTION_KEY: isize = 811589153;

fn main() {
    // part 1
    let f = File::open("input/day20.txt").unwrap();
    let read = BufReader::new(f);
    let lines = read.lines();

    let elems = lines.map(|l| {
        let n: isize = l.unwrap().parse().unwrap();
        n
    });

    // parse
    let list: CircleList<isize> = elems.collect();

    // mix once
    list.mix();

    let mut it = list.iter().skip(1);
    let mut acc: isize = 0;
    for _ in 0..3 {
        let value = it.nth((1000 - 1) % list.len()).unwrap();
        // println!("{}", value);
        acc += value;
    }
    println!("{}", acc);

    // part 2
    let f = File::open("input/day20.txt").unwrap();
    let read = BufReader::new(f);
    let lines = read.lines();

    let elems = lines.map(|l| {
        let n: isize = l.unwrap().parse().unwrap();
        n * DECRYPTION_KEY
    });

    // parse
    let list: CircleList<isize> = elems.collect();

    // mix ten times
    for _ in 0..10 {
        list.mix();
    }

    let mut it = list.iter().skip(1);
    let mut acc: isize = 0;
    for _ in 0..3 {
        let value = it.nth((1000 - 1) % list.len()).unwrap();
        // println!("{}", value);
        acc += value;
    }
    println!("{}", acc);
}
