use std::cell::RefCell;
use std::cmp::max;
use std::cmp::Ordering;
use std::fmt::Display;
use std::rc::{Rc, Weak};

pub struct Stack<T> {
    maxsize: usize,
    items: Vec<T>,
}

impl<T> Stack<T> {
    pub fn with_capacity(maxsize: usize) -> Self {
        Self {
            maxsize,
            items: Vec::with_capacity(maxsize),
        }
    }

    pub fn pop(&mut self) -> Option<T> {
        self.items.pop()
    }

    pub fn push(&mut self, item: T) -> bool {
        if self.items.len() == self.maxsize {
            return false;
        }
        self.items.push(item);
        return true;
    }

    pub fn size(&self) -> usize {
        self.items.len()
    }
}

type Link<T> = Option<Rc<RefCell<QueueNode<T>>>>;

struct QueueNode<T> {
    item: T,
    next: Link<T>,
}
pub struct Queue<T> {
    first: Link<T>,
    last: Link<T>,
    length: usize,
}

impl<T> Queue<T> {
    pub fn new() -> Queue<T> {
        Queue {
            first: None,
            last: None,
            length: 0,
        }
    }
    pub fn is_empty(&self) -> bool {
        self.length == 0
    }
    pub fn size(&self) -> usize {
        self.length
    }
    pub fn enqueue(&mut self, item: T) {
        let temp = self.last.take();
        self.last = Some(Rc::new(RefCell::new(QueueNode { item, next: None })));
        if self.is_empty() {
            self.first = self.last.clone();
        } else {
            let temp = temp.unwrap();
            temp.borrow_mut().next = self.last.clone();
        }
        self.length += 1;
    }

    pub fn dequeue(&mut self) -> Result<T, String> {
        let first = (self.first.take().ok_or("Queue is empty".to_owned()))?;

        if Rc::strong_count(&first) == 2 {
            self.last = None;
        }
        let first_node = Rc::try_unwrap(first)
            .ok()
            .expect("This should be the only owner of the node")
            .into_inner();
        self.first = first_node.next;
        self.length -= 1;
        Ok(first_node.item)
    }

    pub fn dequeue_back(&mut self) -> Result<T, String> {
        let last = (self.last.take().ok_or("Queue is empty".to_owned()))?;
        self.length -= 1;
        {
            let mut search = self
                .first
                .clone()
                .expect("How come the queue is not empty, but there's no first element?");
            let potential_next = search.borrow().next.clone();
            if let Some(mut next) = potential_next {
                let mut potential_other = next.borrow().next.clone();
                while let Some(another) = potential_other {
                    search = next;
                    next = another;
                    potential_other = next.borrow().next.clone();
                }
                let last = search;
                last.borrow_mut().next = None;
                self.last = Some(last);
            } else {
                self.first = None;
            }
        }

        Ok(Rc::try_unwrap(last)
            .ok()
            .expect("This should be the only owner")
            .into_inner()
            .item)
    }
}

struct Node<T> {
    pub data: T,
    pub prev: Option<Weak<RefCell<Node<T>>>>,
    pub next: Option<Rc<RefCell<Node<T>>>>,
}

impl<T> Node<T> {
    pub fn new(data: T) -> Self {
        Self {
            data,
            prev: None,
            next: None,
        }
    }

    pub fn append(node: &mut Rc<RefCell<Node<T>>>, data: T) -> Option<Rc<RefCell<Node<T>>>> {
        let is_last = node.borrow().next.is_none();
        if is_last {
            let mut new_node = Node::new(data);
            new_node.prev = Some(Rc::downgrade(&node));
            let rc = Rc::new(RefCell::new(new_node));
            node.borrow_mut().next = Some(rc.clone());
            Some(rc)
        } else {
            if let Some(ref mut next) = node.borrow_mut().next {
                Self::append(next, data)
            } else {
                None
            }
        }
    }
}

pub struct List<T> {
    first: Option<Rc<RefCell<Node<T>>>>,
    last: Option<Rc<RefCell<Node<T>>>>,
}

impl<T> List<T> {
    pub fn new() -> Self {
        Self {
            first: None,
            last: None,
        }
    }

    pub fn append(&mut self, data: T) {
        if let Some(ref mut next) = self.first {
            self.last = Node::append(next, data);
        } else {
            let f = Rc::new(RefCell::new(Node::new(data)));
            self.first = Some(f.clone());
            self.last = Some(f);
        }
    }
}

impl<T: Display> Display for List<T> {
    fn fmt(&self, w: &mut std::fmt::Formatter) -> std::result::Result<(), std::fmt::Error> {
        write!(w, "[")?;
        let mut node = self.first.clone();
        while let Some(n) = node {
            write!(w, "{}", n.borrow().data)?;
            node = n.borrow().next.clone();
            if node.is_some() {
                write!(w, ", ")?;
            }
        }
        write!(w, "]")
    }
}

struct TreeNode<K, V> {
    key: K,
    data: V,
    left: Option<Box<TreeNode<K, V>>>,
    right: Option<Box<TreeNode<K, V>>>,
}

impl <K: Copy + Ord + Display, V> TreeNode<K, V> {
    pub fn new(key: K, item: V) -> Box<TreeNode<K, V>> {
        Box::new(TreeNode{key: key, data: item, left: None, right: None})
    }

    pub fn contains(&self, key: K) -> bool {
        match self.key.cmp(&key) {
            Equal => true,
            Less => match self.left {
                None => false,
                Some(ref node) => node.contains(key)
            },
            Greater => match self.right {
                None => false,
                Some(ref node) => node.contains(key)
            }
        }
    }

    pub fn get(&self, key: K) -> Option<&V> {
        match self.key.cmp(&key) {
            Equal => Some(&self.data),
            Less => match self.right {
                None => None,
                Some(ref node) => node.get(key)
            },
            Greater => match self.left {
                None => None,
                Some(ref node) => node.get(key)
            }
        }
    }

    pub fn insert(&mut self, key: K, item: V) {
        match self.key.cmp(&key) {
            Equal => {}, // do nothing; item is already inside the tree
            Less => match self.right {
                None => self.right = Some(TreeNode::new(key, item)),
                Some(ref mut node) => node.insert(key, item)
            },
            Greater => match self.left {
                None => self.left = Some(TreeNode::new(key, item)),
                Some(ref mut node) => node.insert(key, item)
            }
        }
    }

    pub fn print(&self) {
        match self.left {
            None => (),
            Some(ref node) => node.print()
        }
        println!("{}", self.key);
        match self.right {
            None => (),
            Some(ref node) => node.print()
        }
    }

    pub fn remove(&mut self, key: K) {
        match self.left {
            None => {},
            Some(ref mut node) => {
                match node.key.cmp(&key) {
                    Equal => match (&node.left, &node.right) {
                        (&None, &None) => {
                            self.left = None;
                            return
                        },
                        (&Some(_), &None) => {
                            self.left = self.left.take().unwrap().left;
                            return
                        },
                        (&None, &Some(_)) => {
                            self.left = self.left.take().unwrap().right;
                            return
                        },
                        (&Some(_), &Some(_)) => {
                            let min = self.delete_min().unwrap(); // should never be None
                            let mut new_left = self.left.take().unwrap();
                            new_left.key = min.key;
                            new_left.data = min.data;
                            self.left = Some(new_left);
                            return
                        }
                    },
                    Greater => node.remove(key),
                    Less => {}
                }
            }
        }

        match self.right {
            None => {},
            Some(ref mut node) => {
                if node.key == key {
                    match (&node.left, &node.right) {
                        (&None, &None) => {
                            self.right = None;
                            return},
                        (&Some(_), &None) => {
                            self.right = self.right.take().unwrap().left;
                            return
                        },
                        (&None, &Some(_)) => {
                            self.right = self.right.take().unwrap().right;
                            return
                        },
                        (&Some(_), &Some(_)) => {
                            let min = self.delete_min().unwrap(); // should never be None
                            let mut new_right = self.right.take().unwrap();
                            new_right.key = min.key;
                            new_right.data = min.data;
                            self.right = Some(new_right);
                            return
                        }
                    }
                } else if key > self.key {
                    node.remove(key)
                }
            }
        }
    }

    fn delete_min(&mut self) -> Option<Box<TreeNode<K, V>>> {
        let mut no_left = false;
        match self.right {
            None => {},
            Some(ref mut node) => {
                if node.left.is_none() {
                    no_left = true;
                } else {

                }
            }
        }
        match no_left {
            true => self.right.take(),
            false => match self.right {
                None => None,
                Some(ref mut node) => node.min()
            }
        }
    }

    fn min(&mut self) -> Option<Box<TreeNode<K, V>>> {
        let mut found = false;
        match self.left {
            None => return None,
            Some(ref mut node) => {
                if node.left.is_none() {
                    found = true;
                } else {

                }
            }
        }
        if found {
            self.left.take()
        } else {
            match self.left {
                None => None,
                Some(ref mut node) => node.min()
            }
        }
    }

}

