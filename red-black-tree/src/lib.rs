#![warn(clippy::all)]

use core::borrow::Borrow;
use std::cmp::Ord;
use std::mem::swap;

#[derive(Default)]
pub struct RedBlackTreeSet<T: Ord> {
    head: Option<Box<Node<T>>>,
}

#[derive(Copy, Clone, PartialEq, Debug)]
enum Color {
    Red,
    Black,
}

#[derive(Copy, Clone, PartialEq, Debug)]
enum Direction {
    Left,
    Right,
    Goal,
}

struct Node<T: Ord> {
    color: Color,
    data: T,
    left: Option<Box<Node<T>>>,
    right: Option<Box<Node<T>>>,
}

impl<T: Ord> RedBlackTreeSet<T> {
    pub fn new() -> Self {
        RedBlackTreeSet { head: None }
    }

    pub fn insert(&mut self, value: T) -> bool {
        if self.head.is_none() {
            self.head = new_some_box_node(Color::Black, value);
            return true;
        }

        let mut route: Vec<(Color, Direction)> = vec![];
        // 探索経路を記録しつつ葉に挿入
        {
            let mut some_box_node = &mut self.head;
            loop {
                let box_node = some_box_node.as_mut().unwrap();
                if box_node.data == value {
                    return false;
                } else if box_node.data > value {
                    route.push((box_node.color, Direction::Left));
                    if box_node.left.is_none() {
                        box_node.left = new_some_box_node(Color::Red, value);
                        break;
                    } else {
                        some_box_node = &mut box_node.left;
                    }
                } else {
                    route.push((box_node.color, Direction::Right));
                    if box_node.right.is_none() {
                        box_node.right = new_some_box_node(Color::Red, value);
                        break;
                    } else {
                        some_box_node = &mut box_node.right;
                    }
                }
            }
            route.push((Color::Red, Direction::Goal))
        }

        loop {
            let len = route.len();
            if len < 3 {
                return true;
            }
            println!("{:?}", route);
            match route[len - 2] {
                (Color::Red, Direction::Left) => {
                    if let (_, Direction::Left) = route[len - 3] {
                        self.rotate_r(&route[0..(len - 1)]);
                        // 回転操作をrouteに反映: CとAを除去
                        route.remove(len - 1);
                        route.remove(len - 3);
                    } else {
                        self.rotate_r_l(&route[0..(len - 1)]);
                        // 回転操作をrouteに反映: 2つ落として末尾を赤に変える
                        route.remove(len - 1);
                        route.remove(len - 2);
                        route[len - 3].0 = Color::Red;
                    }
                }
                (Color::Red, Direction::Right) => {
                    if let (_, Direction::Right) = route[len - 3] {
                        self.rotate_l(&route[0..(len - 1)]);
                        // 回転操作をrouteに反映: CとAを除去
                        route.remove(len - 1);
                        route.remove(len - 3);
                    } else {
                        self.rotate_l_r(&route[0..(len - 1)]);
                        // 回転操作をrouteに反映: 2つ落として末尾を赤に変える
                        route.remove(len - 1);
                        route.remove(len - 2);
                        route[len - 3].0 = Color::Red;
                    }
                }
                (_, _) => return true,
            }
        }
    }

    pub fn get<Q>(&self, value: &Q) -> Option<&T>
    where
        T: Borrow<Q>,
        Q: Ord,
    {
        let mut sbn = &self.head;
        loop {
            match sbn {
                None => return None,
                Some(ref bn) => {
                    if bn.data.borrow() == value {
                        return Some(&(bn.data));
                    } else if bn.data.borrow() > value {
                        sbn = &bn.right
                    } else {
                        sbn = &bn.left
                    }
                }
            }
        }
    }

    // 右回転
    // 1. Cを黒に変える
    // 2．NoneとB(A.left)をスワップ
    // 3. BとAをスワップ
    // 4. AとT3(B.right)をスワップ
    // 5. T3とNone(A.left)をスワップ
    // 6. ルートを黒に変える
    fn rotate_r(&mut self, route: &[(Color, Direction)]) {
        let len = route.len();
        let mut temp: Option<Box<Node<T>>> = None;
        // 1. Cを黒に変える
        {
            match get_some_box_node(&mut self.head, &route) {
                Some(ref mut bn) => bn.color = Color::Black,
                None => panic!(),
            }
        }
        // 2．NoneとB(A.left)をスワップ
        {
            match get_some_box_node(&mut self.head, &route[0..(len - 2)]) {
                Some(ref mut bn) => swap(&mut temp, &mut bn.left),
                None => panic!(),
            }
        }
        // 3. BとAをスワップ
        {
            let mut sbn = get_some_box_node(&mut self.head, &route[0..(len - 2)]);
            swap(&mut temp, &mut sbn);
        }
        // 4. AとT3(B.right)をスワップ
        {
            match get_some_box_node(&mut self.head, &route[0..(len - 2)]) {
                Some(ref mut bn) => swap(&mut temp, &mut bn.right),
                None => panic!(),
            }
        }
        // 5. T3とNone(A.left)をスワップ
        {
            match get_some_box_node(&mut self.head, &route[0..(len - 2)]) {
                Some(ref mut bn) => match &mut bn.right {
                    Some(ref mut bn) => swap(&mut temp, &mut bn.left),
                    None => panic!(),
                },
                None => panic!(),
            }
        }
        // 6. 根を黒に変える
        {
            match &mut self.head {
                Some(ref mut bn) => bn.color = Color::Black,
                None => panic!(),
            }
        }
    }

    // 左回転
    // 1. Cを黒に変える
    // 2．NoneとB(A.right)をスワップ
    // 3. BとAをスワップ
    // 4. AとT2(B.left)をスワップ
    // 5. T2とNone(A.right)をスワップ
    // 6. 根を黒に変える
    fn rotate_l(&mut self, route: &[(Color, Direction)]) {
        let len = route.len();
        let mut temp: Option<Box<Node<T>>> = None;
        // 1. Cを黒に変える
        {
            match get_some_box_node(&mut self.head, &route) {
                Some(ref mut bn) => bn.color = Color::Black,
                None => panic!(),
            }
        }
        // 2．NoneとB(A.right)をスワップ
        {
            match get_some_box_node(&mut self.head, &route[0..(len - 2)]) {
                Some(ref mut bn) => swap(&mut temp, &mut bn.right),
                None => panic!(),
            }
        }
        // 3. BとAをスワップ
        {
            let mut sbn = get_some_box_node(&mut self.head, &route[0..(len - 2)]);
            swap(&mut temp, &mut sbn);
        }
        // 4. AとT2(B.left)をスワップ
        {
            match get_some_box_node(&mut self.head, &route[0..(len - 2)]) {
                Some(ref mut bn) => swap(&mut temp, &mut bn.left),
                None => panic!(),
            }
        }
        // 5. T2とNone(A.right)をスワップ
        {
            match get_some_box_node(&mut self.head, &route[0..(len - 2)]) {
                Some(ref mut bn) => match &mut bn.left {
                    Some(ref mut bn) => swap(&mut temp, &mut bn.right),
                    None => panic!(),
                },
                None => panic!(),
            }
        }
        // 6. 根を黒に変える
        {
            match &mut self.head {
                Some(ref mut bn) => bn.color = Color::Black,
                None => panic!(),
            }
        }
    }

    // 右左回転
    // 1. Bを黒に変える
    // 2. NoneとC(B.left)をスワップ
    // 3. T3(C.right)とNone(B.left)をスワップ
    // 4. None(C.right)とB(A.right)をスワップ
    // 5. CとAをスワップ
    // 6. None(A.right)とT2(C.left)をスワップ
    // 7. AとNone(C.left)をスワップ
    // 8. 根を黒に変える
    fn rotate_r_l(&mut self, route: &[(Color, Direction)]) {
        let len = route.len();
        let mut temp: Option<Box<Node<T>>> = None;
        // 1. Bを黒に変える
        {
            match get_some_box_node(&mut self.head, &route[0..(len - 1)]) {
                Some(ref mut bn) => bn.color = Color::Black,
                None => panic!(),
            }
        }
        // 2. NoneとC(B.left)をスワップ
        {
            match get_some_box_node(&mut self.head, &route[0..(len - 1)]) {
                Some(ref mut bn) => swap(&mut temp, &mut bn.left),
                None => panic!(),
            }
        }
        // 3. T3(C.right)とNone(B.left)をスワップ
        {
            match &mut temp {
                Some(inbox_temp) => match get_some_box_node(&mut self.head, &route[0..(len - 1)]) {
                    Some(ref mut bn) => swap(&mut inbox_temp.right, &mut bn.left),
                    None => panic!(),
                },
                None => panic!(),
            }
        }
        // 4. None(C.right)とB(A.right)をスワップ
        {
            match &mut temp {
                Some(inbox_temp) => match get_some_box_node(&mut self.head, &route[0..(len - 2)]) {
                    Some(ref mut bn) => swap(&mut inbox_temp.right, &mut bn.right),
                    None => panic!(),
                },
                None => panic!(),
            }
        }
        // 5. CとAをスワップ
        {
            let mut sbn = get_some_box_node(&mut self.head, &route[0..(len - 2)]);
            swap(&mut temp, &mut sbn);
        }
        // 6. None(A.right)とT2(C.left)をスワップ
        {
            match &mut temp {
                Some(inbox_temp) => match get_some_box_node(&mut self.head, &route[0..(len - 2)]) {
                    Some(ref mut bn) => swap(&mut inbox_temp.right, &mut bn.left),
                    None => panic!(),
                },
                None => panic!(),
            }
        }
        // 7. AとNone(C.left)をスワップ
        {
            match get_some_box_node(&mut self.head, &route[0..(len - 2)]) {
                Some(ref mut bn) => swap(&mut temp, &mut bn.left),
                None => panic!(),
            }
        }
        // 8. 根を黒に変える
        {
            match &mut self.head {
                Some(ref mut bn) => bn.color = Color::Black,
                None => panic!(),
            }
        }
    }

    // 左右回転
    // 1. Bを黒に変える
    // 2. NoneとC(B.right)をスワップ
    // 3. T2(C.left)とNone(B.right)をスワップ
    // 4. None(C.left)とB(A.left)をスワップ
    // 5. CとAをスワップ
    // 6. None(A.left)とT3(C.right)をスワップ
    // 7. AとNone(C.right)をスワップ
    // 8. 根を黒に変える
    fn rotate_l_r(&mut self, route: &[(Color, Direction)]) {
        let len = route.len();
        let mut temp: Option<Box<Node<T>>> = None;
        // 1. Bを黒に変える
        {
            match get_some_box_node(&mut self.head, &route[0..(len - 1)]) {
                Some(ref mut bn) => bn.color = Color::Black,
                None => panic!(),
            }
        }
        // 2. NoneとC(B.right)をスワップ
        {
            match get_some_box_node(&mut self.head, &route[0..(len - 1)]) {
                Some(ref mut bn) => swap(&mut temp, &mut bn.right),
                None => panic!(),
            }
        }
        // 3. T2(C.left)とNone(B.right)をスワップ
        {
            match &mut temp {
                Some(inbox_temp) => match get_some_box_node(&mut self.head, &route[0..(len - 1)]) {
                    Some(ref mut bn) => swap(&mut inbox_temp.left, &mut bn.right),
                    None => panic!(),
                },
                None => panic!(),
            }
        }
        // 4. None(C.left)とB(A.left)をスワップ
        {
            match &mut temp {
                Some(inbox_temp) => match get_some_box_node(&mut self.head, &route[0..(len - 2)]) {
                    Some(ref mut bn) => swap(&mut inbox_temp.left, &mut bn.left),
                    None => panic!(),
                },
                None => panic!(),
            }
        }
        // 5. CとAをスワップ
        {
            let mut sbn = get_some_box_node(&mut self.head, &route[0..(len - 2)]);
            swap(&mut temp, &mut sbn);
        }
        // 6. None(A.left)とT3(C.right)をスワップ
        {
            match &mut temp {
                Some(inbox_temp) => match get_some_box_node(&mut self.head, &route[0..(len - 2)]) {
                    Some(ref mut bn) => swap(&mut inbox_temp.left, &mut bn.right),
                    None => panic!(),
                },
                None => panic!(),
            }
        }
        // 7. AとNone(C.right)をスワップ
        {
            match get_some_box_node(&mut self.head, &route[0..(len - 2)]) {
                Some(ref mut bn) => swap(&mut temp, &mut bn.right),
                None => panic!(),
            }
        }
        // 8. 根を黒に変える
        {
            match &mut self.head {
                Some(ref mut bn) => bn.color = Color::Black,
                None => panic!(),
            }
        }
    }
}

fn get_some_box_node<'a, T: Ord>(
    head: &'a mut Option<Box<Node<T>>>,
    route: &[(Color, Direction)],
) -> &'a mut Option<Box<Node<T>>> {
    let mut mref_some_box_node = head;
    for r in route {
        if let Some(box_node) = mref_some_box_node {
            if r.1 == Direction::Left {
                mref_some_box_node = &mut box_node.left;
            } else {
                mref_some_box_node = &mut box_node.right;
            }
        }
    }
    mref_some_box_node
}

fn new_some_box_node<T: Ord>(color: Color, value: T) -> Option<Box<Node<T>>> {
    Some(Box::new(Node {
        color,
        data: value,
        left: None,
        right: None,
    }))
}

#[test]
fn test_insert_2_items() {
    let mut set: RedBlackTreeSet<u32> = RedBlackTreeSet::new();
    set.insert(3);
    set.insert(2);
    match &(set.head) {
        Some(box_node) => {
            assert_eq!(box_node.data, 3);
            assert_eq!(box_node.color, Color::Black);
            match &box_node.left {
                Some(box_node) => {
                    assert_eq!(box_node.data, 2);
                    assert_eq!(box_node.color, Color::Red);
                }
                None => panic!(),
            }
        }
        None => panic!(),
    }
}

#[test]
fn test_right_rotate() {
    let mut set: RedBlackTreeSet<u32> = RedBlackTreeSet::new();
    set.insert(3);
    set.insert(2);
    set.insert(1);
    match &(set.head) {
        Some(box_node) => {
            assert_eq!(box_node.data, 2);
            assert_eq!(box_node.color, Color::Black);
            match &box_node.left {
                Some(box_node) => {
                    assert_eq!(box_node.data, 1);
                    assert_eq!(box_node.color, Color::Black);
                }
                None => panic!(),
            }
            match &box_node.right {
                Some(box_node) => {
                    assert_eq!(box_node.data, 3);
                    assert_eq!(box_node.color, Color::Black);
                }
                None => panic!(),
            }
        }
        None => panic!(),
    }
}

#[test]
fn test_left_rotate() {
    let mut set: RedBlackTreeSet<u32> = RedBlackTreeSet::new();
    set.insert(1);
    set.insert(2);
    set.insert(3);
    match &(set.head) {
        Some(box_node) => {
            assert_eq!(box_node.data, 2);
            assert_eq!(box_node.color, Color::Black);
            match &box_node.left {
                Some(box_node) => {
                    assert_eq!(box_node.data, 1);
                    assert_eq!(box_node.color, Color::Black);
                }
                None => panic!(),
            }
            match &box_node.right {
                Some(box_node) => {
                    assert_eq!(box_node.data, 3);
                    assert_eq!(box_node.color, Color::Black);
                }
                None => panic!(),
            }
        }
        None => panic!(),
    }
}

#[test]
fn test_right_left_rotate() {
    let mut set: RedBlackTreeSet<u32> = RedBlackTreeSet::new();
    set.insert(1);
    set.insert(3);
    set.insert(2);
    match &(set.head) {
        Some(box_node) => {
            assert_eq!(box_node.data, 2);
            assert_eq!(box_node.color, Color::Black);
            match &box_node.left {
                Some(box_node) => {
                    assert_eq!(box_node.data, 1);
                    assert_eq!(box_node.color, Color::Black);
                }
                None => panic!(),
            }
            match &box_node.right {
                Some(box_node) => {
                    assert_eq!(box_node.data, 3);
                    assert_eq!(box_node.color, Color::Black);
                }
                None => panic!(),
            }
        }
        None => panic!(),
    }
}

#[test]
fn test_left_right_rotate() {
    let mut set: RedBlackTreeSet<u32> = RedBlackTreeSet::new();
    set.insert(3);
    set.insert(1);
    set.insert(2);
    match &(set.head) {
        Some(box_node) => {
            assert_eq!(box_node.data, 2);
            assert_eq!(box_node.color, Color::Black);
            match &box_node.left {
                Some(box_node) => {
                    assert_eq!(box_node.data, 1);
                    assert_eq!(box_node.color, Color::Black);
                }
                None => panic!(),
            }
            match &box_node.right {
                Some(box_node) => {
                    assert_eq!(box_node.data, 3);
                    assert_eq!(box_node.color, Color::Black);
                }
                None => panic!(),
            }
        }
        None => panic!(),
    }
}

#[test]
fn test_recursive_rotate() {
    let mut set: RedBlackTreeSet<u32> = RedBlackTreeSet::new();
    set.insert(1);
    set.insert(3);
    set.insert(2);
    set.insert(5);
    set.insert(4);
    set.insert(6);
    set.insert(7);
    match &(set.head) {
        Some(box_node) => {
            assert_eq!(box_node.data, 4);
            assert_eq!(box_node.color, Color::Black);
            match &box_node.left {
                Some(box_node) => {
                    assert_eq!(box_node.data, 2);
                    assert_eq!(box_node.color, Color::Black);
                    match &box_node.left {
                        Some(box_node) => {
                            assert_eq!(box_node.data, 1);
                            assert_eq!(box_node.color, Color::Black);
                        }
                        None => panic!(),
                    }
                    match &box_node.right {
                        Some(box_node) => {
                            assert_eq!(box_node.data, 3);
                            assert_eq!(box_node.color, Color::Black);
                        }
                        None => panic!(),
                    }
                }
                None => panic!(),
            }
            match &box_node.right {
                Some(box_node) => {
                    assert_eq!(box_node.data, 6);
                    assert_eq!(box_node.color, Color::Black);
                    match &box_node.left {
                        Some(box_node) => {
                            assert_eq!(box_node.data, 5);
                            assert_eq!(box_node.color, Color::Black);
                        }
                        None => panic!(),
                    }
                    match &box_node.right {
                        Some(box_node) => {
                            assert_eq!(box_node.data, 7);
                            assert_eq!(box_node.color, Color::Black);
                        }
                        None => panic!(),
                    }
                }
                None => panic!(),
            }
        }
        None => panic!(),
    }
}
