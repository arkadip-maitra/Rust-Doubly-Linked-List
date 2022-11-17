use std::cell::RefCell;
use std::rc::Rc;

type Point<T> = Option<Rc<RefCell<Node<T>>>>;

struct Node<T> {
    val: T,
    next: Point<T>,
    prev: Point<T>,
}

impl<T> Node<T> {
    fn new(val: T) -> Self {
        Self {
            val,
            next: None,
            prev: None,
        }
    }
}

pub struct DLL<T> {
    head: Point<T>,
    tail: Point<T>,
    size: usize,
}

impl<T> DLL<T> {
    pub fn new() -> Self {
        Self {
            head: None,
            tail: None,
            size: 0,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn len(&self) -> usize {
        self.size
    }

    pub fn append(&mut self, val: T) {
        println!("Append");
        let node = Rc::new(RefCell::new(Node::new(val)));
        if let Some(prev_tail) = self.tail.take() {
            prev_tail.borrow_mut().next = Some(Rc::clone(&node));
            node.borrow_mut().prev = Some(prev_tail);
            self.tail = Some(node);
            self.size += 1;
        } else {
            self.head = Some(Rc::clone(&node));
            self.tail = Some(node);
            self.size = 1;
        }
    }

    pub fn insert(&mut self, val: T) {
        println!("Insert");
        let node = Rc::new(RefCell::new(Node::new(val)));
        if let Some(prev_head) = self.head.take() {
            prev_head.borrow_mut().prev = Some(Rc::clone(&node));
            node.borrow_mut().next = Some(prev_head);
            self.head = Some(node);
            self.size += 1;
        } else {
            self.head = Some(Rc::clone(&node));
            self.tail = Some(node);
            self.size = 1;
        }
    }
    
    pub fn delete_head(&mut self) -> Option<T> {
        println!("Delete_Head");
        self.head.take().map(|prev_head| {
            self.size -= 1;
            match prev_head.borrow_mut().next.take() {
                Some(node) => {
                    node.borrow_mut().prev = None;
                    self.head = Some(node);
                }
                None => {
                    self.tail.take();
                }
            }
            Rc::try_unwrap(prev_head).ok().unwrap().into_inner().val
        })
    }
    
    pub fn delete_tail(&mut self) -> Option<T> {
        println!("Delete_Tail");
        self.tail.take().map(|prev_tail| {
            self.size -= 1;
            match prev_tail.borrow_mut().prev.take() {
                Some(node) => {
                    node.borrow_mut().next = None;
                    self.tail = Some(node);
                }
                None => {
                    self.head.take();
                }
            }
            Rc::try_unwrap(prev_tail).ok().unwrap().into_inner().val
        })
    }
    
    pub fn drop(&mut self) {
        while let Some(node) = self.head.take() {
            println!("Dropping");
            let _ = node.borrow_mut().prev.take();
            self.head = node.borrow_mut().next.take();
        }
        self.tail.take();
        self.size = 0;
    }
    
}

fn main() {
    let mut l = DLL::new();
    for i in 0..5 {
        l.append(i);
        l.insert(i);
    }
    assert_eq!(l.delete_head(), Some(4));
    assert_eq!(l.len(), 9);
    assert_eq!(l.delete_tail(), Some(4));
    assert_eq!(l.len(), 8);
    l.drop();
    assert_eq!(l.is_empty(), true);
}

