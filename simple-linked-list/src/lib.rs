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
        let mut some_box_node = &self.head;
        for i in 0.. {
            match some_box_node {
                None => return i,
                Some(box_node) => some_box_node = &((*box_node).next),
            }
        }
        panic!("never here")
    }

    // push_back
    pub fn push(&mut self, element: T) {
        let mut some_box_node: &mut Option<Box<Node<T>>> = &mut self.head;
        loop {
            match some_box_node {
                None => {
                    let mut new_some_box_node = Some(Box::new(Node {
                        data: element,
                        next: None,
                    }));
                    std::mem::swap(&mut new_some_box_node, &mut some_box_node);
                    return;
                }
                Some(ref mut box_node) => some_box_node = &mut (*box_node).next,
            }
        }
    }

    pub fn push_front(&mut self, element: T) {
        let mut some_box_node = Some(Box::new(Node {
            data: element,
            next: None,
        }));
        if let None = &self.head {
            self.head = some_box_node;
            return;
        }
        std::mem::swap(&mut some_box_node, &mut self.head);
        match &mut self.head {
            Some(ref mut box_node) => (*box_node).next = some_box_node,
            None => panic!("never occure"),
        }
    }

    // pop back
    pub fn pop(&mut self) -> Option<T> {
        let mut some_box_node: &mut Option<Box<Node<T>>> = &mut self.head;
        if let None = some_box_node {
            return None;
        }
        loop {
            if let Some(box_node) = some_box_node {
                if let None = (*box_node).next {
                    let result_node = std::mem::replace(some_box_node, None);
                    return Some((*(result_node.unwrap())).data);
                }
            }
            // 上のif let式中でmatchでNoneとSomeで分岐させると、some_box_nodeへの代入でなぜかmut借用エラーになる。よってここでif let式からやり直す
            if let Some(box_node) = some_box_node {
                some_box_node = &mut (*box_node).next
            }
        }
    }

    pub fn pop_front(&mut self) -> Option<T> {
        let mut next_some_box_node;
        match &mut self.head {
            None => return None,
            Some(ref mut box_node) => {
                next_some_box_node = std::mem::replace(&mut (*box_node).next, None);
            }
        }
        let result = std::mem::replace(&mut self.head, None);
        self.head = next_some_box_node;
        Some((*(result.unwrap())).data)
    }

    pub fn peek(&self) -> Option<&T> {
        if let None = &self.head {
            return None;
        }
        let mut some_box_node: &Option<Box<Node<T>>> = &self.head;
        loop {
            if let Some(box_node) = some_box_node {
                let node: &Node<T> = &(*box_node);
                match node.next {
                    None => return Some(&node.data),
                    Some(_) => some_box_node = &node.next,
                }
            };
        }
    }
}

impl<T: Clone> SimpleLinkedList<T> {
    pub fn rev(&self) -> SimpleLinkedList<T> {
        let mut list = SimpleLinkedList::new();
        if let None = &self.head {
            return list;
        }
        let mut some_box_node: &Option<Box<Node<T>>> = &self.head;
        loop {
            if let Some(box_node) = some_box_node {
                let node = &(*box_node);
                list.push_front(node.data.clone());
                match &node.next {
                    None => return list,
                    Some(_) => some_box_node = &node.next,
                }
            }
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
                None => return v,
                Some(box_n) => n = *box_n,
            }
        }
    }
}
