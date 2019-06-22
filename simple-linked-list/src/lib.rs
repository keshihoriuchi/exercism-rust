pub use std::boxed::Box;

pub struct SimpleLinkedList<T> {
    head: Option<Box<Node<T>>>,
}

struct Node<T> {
    data: T,
    next: Option<Box<Node<T>>>,
}

impl<T> SimpleLinkedList<T> {
    pub fn new() -> Self {
        SimpleLinkedList { head: None }
    }

    pub fn len(&self) -> usize {
        let mut i = 0;
        let mut n = &self.head;
        loop {
            match n {
                None => return i,
                Some(b) => {
                    i += 1;
                    n = &((*b).next)
                }
            }
        }
    }

    pub fn push(&mut self, element: T) {
        let some_box_node = Some(Box::new(Node {
            data: element,
            next: None,
        }));
        match &mut self.head {
            None => {
                self.head = some_box_node;
                return;
            }
            Some(ref mut box_node) => {
                let node: &mut Node<T> = &mut (*box_node);
                recur_func_for_push_back(node, some_box_node)
            }
        }
    }

    pub fn pop(&mut self) -> Option<T> {
        match &mut self.head {
            None => None,
            Some(ref mut box_node) => {
                let node: &mut Node<T> = &mut (*box_node);
                match node.next {
                    None => {
                        let mut result_node = None;
                        std::mem::swap(&mut result_node, &mut self.head);
                        Some((*(result_node.unwrap())).data)
                    }
                    Some(_) => recur_func_for_pop_back(node),
                }
            }
        }
    }

    pub fn peek(&self) -> Option<&T> {
        match &self.head {
            None => None,
            Some(box_node) => {
                let node: &Node<T> = &(*box_node);
                recur_func_for_peek(&node)
            }
        }
    }
}

fn recur_func_for_push_back<T>(node: &mut Node<T>, some_box_node: Option<Box<Node<T>>>) {
    match &mut node.next {
        None => node.next = some_box_node,
        Some(ref mut box_node) => recur_func_for_push_back(&mut (*box_node), some_box_node),
    }
}

fn recur_func_for_pop_back<T>(node: &mut Node<T>) -> Option<T> {
    match node.next {
        None => panic!("node.next never be `None`"),
        Some(ref mut next_box_node) => {
            let next_node: &mut Node<T> = &mut (*next_box_node);
            match next_node.next {
                None => {
                    let mut result_node = None;
                    std::mem::swap(&mut result_node, &mut node.next);
                    return Some((*(result_node.unwrap())).data);
                }
                Some(_) => return recur_func_for_pop_back(next_node),
            }
        }
    }
}

fn recur_func_for_peek<T>(node: &Node<T>) -> Option<&T> {
    match &node.next {
        None => Some(&(node.data)),
        Some(box_node) => recur_func_for_peek(&(*box_node)),
    }
}

impl<T: Clone> SimpleLinkedList<T> {
    pub fn rev(&self) -> SimpleLinkedList<T> {
        let list: SimpleLinkedList<T> = SimpleLinkedList::new();
        match &self.head {
            None => list,
            Some(box_node) => recur_func_for_rev(box_node, list),
        }
    }
}

fn recur_func_for_rev<T: Clone>(
    box_node: &Box<Node<T>>,
    mut list: SimpleLinkedList<T>,
) -> SimpleLinkedList<T> {
    let node = &(*box_node);
    match &node.next {
        None => {
            list.push(node.data.clone());
            list
        }
        Some(next_box_node) => {
            let mut newlist = recur_func_for_rev(&next_box_node, list);
            newlist.push(node.data.clone());
            newlist
        }
    }
}

impl<'a, T: Clone> From<&'a [T]> for SimpleLinkedList<T> {
    fn from(item: &[T]) -> Self {
        let mut list: SimpleLinkedList<T> = SimpleLinkedList::new();
        for e in item {
            list.push((*e).clone())
        }
        list
    }
}

impl<T> Into<Vec<T>> for SimpleLinkedList<T> {
    fn into(self) -> Vec<T> {
        let mut v = Vec::with_capacity(self.len());
        if let None = self.head {
            return v;
        }
        let mut n: Node<T> = *(self.head.unwrap());
        loop {
            v.push(n.data);
            match n.next {
                None => break,
                Some(box_n) => n = *box_n,
            }
        }
        v
    }
}
