use std::mem;
use std::cmp::Ordering;

pub type Link<K, V> = Option<Box<Node<K, V>>>;

#[derive(Debug)]
pub struct Node<K, V> {
    pub key: K,
    pub val: V,
    n: usize,
    color: Colors,
    left: Link<K, V>,
    right: Link<K, V>,
}

#[derive(Debug)]
enum Colors {
    RED,
    BLACK,
}

enum FlipType {
    UP,
    DOWN,
}

trait LinkMethods<K, V> {
    fn new(key: K, val: V) -> Link<K, V>;
    fn put(&mut self, key: K, val: V);
    fn get(&self, key: K) -> Option<&V>;
    fn delete(&mut self, key: K);
    fn delete_min(&mut self);
    fn delete_max(&mut self);
    fn size(&self) -> usize;
    fn update_size(&mut self);
    fn is_red(&self) -> bool;
    fn left(&self) -> &Link<K, V>;
    fn left_mut(&mut self) -> &mut Link<K, V>;
    fn right(&self) -> &Link<K, V>;
    fn right_mut(&mut self) -> &mut Link<K, V>;
    fn min(&self) -> &Link<K, V>;
    fn min_mut(&mut self) -> &mut Link<K, V>;
    fn max(&self) -> &Link<K, V>;
    fn rotate_left(&mut self);
    fn rotate_right(&mut self);
    fn flip_colors(&mut self, flip_type: FlipType);
    fn balance(&mut self);
    fn compare_key(key: &K, link: &Link<K, V>) -> Option<Ordering>;
    fn move_red_left(&mut self);
    fn move_red_right(&mut self);
    fn select(&self, k: usize) -> &Link<K, V>;
    fn rank(&self, key: K) -> usize;
    fn floor(&self, key: K) -> &Link<K, V>;
    fn ceiling(&self, key: K) -> &Link<K, V>;
    fn pre_order(&self) -> Vec<&Node<K, V>>;
    fn in_order(&self) -> Vec<&Node<K, V>>;
    fn post_order(&self) -> Vec<&Node<K, V>>;
    fn level_order(&self) -> Vec<&Node<K, V>>;
}

impl<K: PartialOrd, V> LinkMethods<K, V> for Link<K, V> {
    fn new(key: K, val: V) -> Self {
        let boxed_node = Box::new(Node {
            key,
            val,
            n: 1,
            color: Colors::RED,
            left: None,
            right: None,
        });

        Some(boxed_node)
    }

    fn put(&mut self, key: K, val: V) {
        match Self::compare_key(&key, &self) {
            Some(Ordering::Less) => self.left_mut().put(key, val),
            Some(Ordering::Greater) => self.right_mut().put(key, val),
            Some(Ordering::Equal) => {
                self.as_mut().map(|node| node.val = val);
            },
            None => *self = Self::new(key, val),
        };

        self.balance();
    }

    fn get(&self, key: K) -> Option<&V> {
        match Self::compare_key(&key, &self) {
            Some(Ordering::Less) => self.left().get(key),
            Some(Ordering::Greater) => self.right().get(key),
            Some(Ordering::Equal) => Some(&self.as_ref().unwrap().val),
            None => None,
        }
    }

    fn delete(&mut self, key: K) {
        match Self::compare_key(&key, &self) {
            Some(Ordering::Less) => {
                // 确保左侧节点为红色
                if ! self.left().is_red() && ! self.left().left().is_red() {
                    self.move_red_left();
                }

                self.left_mut().delete(key);
            },
            Some(Ordering::Greater) | Some(Ordering::Equal) => {
                // 因为要经过右分支，所以如果 h.left 为红色，就进行右旋
                if self.left().is_red() {
                    self.rotate_right();
                }

                if let Some(Ordering::Equal) = Self::compare_key(&key, &self) {
                    if self.right().is_none() {
                        *self = None;
                        return
                    }
                }

                // 确保右侧节点为红色
                if ! self.right().is_red() && ! self.right().left().is_red() {
                    self.move_red_right();
                }

                // 经过旋转之后，当前节点匹配成功的话，右侧节点必定不为空
                if let Some(Ordering::Equal) = Self::compare_key(&key, &self) {
                    if let Some(mut boxed_node) = self.take() {
                        {
                            let node = &mut *boxed_node;
                            let next = node.right.min_mut();
                            mem::swap(&mut node.key, &mut next.as_mut().unwrap().key);
                            mem::swap(&mut node.val, &mut next.as_mut().unwrap().val);
                        }

                        boxed_node.right.delete_min();

                        *self = Some(boxed_node);
                    }
                }
                else {
                    self.right_mut().delete(key);
                }
            },
            None => {},
        }

        self.balance();
    }

    fn delete_min(&mut self) {
        if self.left().is_none() {
            *self = None;
            return
        }

        if ! self.left().is_red() && ! self.left().left().is_red() {
            self.move_red_left();
        }

        self.left_mut().delete_min();

        self.balance();
    }

    fn delete_max(&mut self) {
        if self.left().is_red() {
            self.rotate_right();
        }

        if self.right().is_none() {
            *self = None;
            return
        }

        if ! self.right().is_red() && ! self.right().left().is_red() {
            self.move_red_right();
        }

        self.right_mut().delete_max();

        self.balance();
    }

    fn size(&self) -> usize {
        match *self {
            Some(ref boxed_node) => boxed_node.n,
            None => 0,
        }
    }

    fn update_size(&mut self) {
        self.as_mut().map(|node| {
            node.n = node.left.size() + node.right.size() + 1;
        });
    }

    fn is_red(&self) -> bool {
        match *self {
            Some(ref boxed_node) => {
                match boxed_node.color {
                    Colors::RED => true,
                    Colors::BLACK => false,
                }
            },
            None => false,
        }
    }

    fn left(&self) -> &Self {
        &self.as_ref().unwrap().left
    }

    fn left_mut(&mut self) -> &mut Self {
        &mut self.as_mut().unwrap().left
    }

    fn right(&self) -> &Self {
        &self.as_ref().unwrap().right
    }

    fn right_mut(&mut self) -> &mut Self {
        &mut self.as_mut().unwrap().right
    }

    fn min(&self) -> &Self {
        match {self} {
            &Some(ref node) if node.left.is_some() => {
                node.left.min()
            },
            node @ &Some(_) | node @ &None => node,
        }
    }

    fn min_mut(&mut self) -> &mut Self {
        match {self} {
            &mut Some(ref mut node) if node.left.is_some() => {
                node.left.min_mut()
            },
            node @ &mut Some(_) | node @ &mut None => node,
        }
    }

    fn max(&self) -> &Self {
        match {self} {
            &Some(ref node) if node.right.is_some() => {
                node.right.max()
            },
            node @ &Some(_) | node @ &None => node,
        }
    }

    fn rotate_left(&mut self) {
        let mut h = self.take();
        let mut x = h.right_mut().take();

        x.as_mut().map(|node| {
            node.color = match &h.as_ref().unwrap().color {
                &Colors::RED => Colors::RED,
                &Colors::BLACK => Colors::BLACK,
            };
            node.n = h.as_ref().unwrap().n;
        });

        h.as_mut().map(|node| {
            node.color = Colors::RED;
            node.right = x.left_mut().take();
        });

        h.update_size();

        x.as_mut().map(|node| node.left = h);

        *self = x;
    }

    fn rotate_right(&mut self) {
        let mut h = self.take();
        let mut x = h.left_mut().take();

        x.as_mut().map(|node| {
            node.color = match &h.as_ref().unwrap().color {
                &Colors::RED => Colors::RED,
                &Colors::BLACK => Colors::BLACK,
            };
            node.n = h.as_ref().unwrap().n;
        });

        h.as_mut().map(|node| {
            node.color = Colors::RED;
            node.left = x.right_mut().take();
        });

        h.update_size();

        x.as_mut().map(|node| node.right = h);

        *self = x;
    }

    fn flip_colors(&mut self, flip_type: FlipType) {
        self.as_mut().map(|node| {
            match flip_type {
                FlipType::UP => {
                    node.color = Colors::RED;
                    node.left.as_mut().map(|left| left.color = Colors::BLACK);
                    node.right.as_mut().map(|right| right.color = Colors::BLACK);
                },
                FlipType::DOWN => {
                    node.color = Colors::BLACK;
                    node.left.as_mut().map(|left| left.color = Colors::RED);
                    node.right.as_mut().map(|right| right.color = Colors::RED);
                }
            }
        });
    }

    fn balance(&mut self) {
        // 左偏红黑树，不存在右侧红节点

        // h.right 为红色，执行左旋
        if ! self.left().is_red() && self.right().is_red() {
            self.rotate_left();
        }

        // h.left 和 h.left.left 为红色，执行右旋
        if self.left().is_red() && self.left().left().is_red() {
            self.rotate_right();
        }

        // h.left 和 h.right 为红色，分解 4 节点
        if self.left().is_red() && self.right().is_red() {
            self.flip_colors(FlipType::UP);
        }

        self.update_size();
    }

    fn compare_key(key: &K, link: &Self) -> Option<Ordering> {
        match *link {
            Some(ref boxed_node) => {
                if key < &boxed_node.key {
                    Some(Ordering::Less)
                }
                else if key > &boxed_node.key {
                    Some(Ordering::Greater)
                }
                else {
                    Some(Ordering::Equal)
                }
            },
            None => None,
        }
    }

    fn move_red_left(&mut self) {
        // 假设当前节点 h 为红色，h.right 和 h.right.left 为黑色
        // 将 h.left 或者 h.left.left 变红

        // Easy case: h.right.left is BLACK
        // combine siblings
        self.flip_colors(FlipType::DOWN);

        // Harder case: h.right.left is RED
        // borrow from siblings
        if self.right().left().is_red() {
            self.right_mut().rotate_right();
            self.rotate_left();
            self.flip_colors(FlipType::UP);
        }
    }

    fn move_red_right(&mut self) {
        // 假设当前节点 h 为红色，h.left 和 h.left.left 为黑色
        // 将 h.right 或者 h.right.right 变红

        // Easy case: h.left.left is BLACK
        // combine siblings
        self.flip_colors(FlipType::DOWN);

        // Harder case: h.left.left is RED
        // borrow from siblings
        if self.left().left().is_red() {
            self.rotate_right();
            self.flip_colors(FlipType::UP);
        }
    }

    fn select(&self, k: usize) -> &Self {
        match {self} {
            &Some(ref boxed_node) if boxed_node.left.size() != k => {
                let t = boxed_node.left.size();

                if k < t {
                    boxed_node.left.select(k)
                }
                else {
                    boxed_node.right.select(k - t - 1)
                }
            },
            link @ &Some(_) | link @ &None => link,
        }
    }

    fn rank(&self, key: K) -> usize {
        match Self::compare_key(&key, &self) {
            Some(Ordering::Less) => self.left().rank(key),
            Some(Ordering::Greater) => self.left().size() + self.right().rank(key) + 1,
            Some(Ordering::Equal) => self.left().size(),
            None => 0,
        }
    }

    fn floor(&self, key: K) -> &Self {
        match Self::compare_key(&key, &self) {
            Some(Ordering::Less) => self.left().floor(key),
            Some(Ordering::Greater) => {
                let node = self.right().floor(key);

                if node.is_none() {
                    &self
                }
                else {
                    node
                }
            },
            Some(Ordering::Equal) | None => &None,
        }
    }

    fn ceiling(&self, key: K) -> &Self {
        match Self::compare_key(&key, &self) {
            Some(Ordering::Less) => {
                let node = self.left().ceiling(key);

                if node.is_none() {
                    &self
                } else {
                    node
                }
            },
            Some(Ordering::Greater) => self.right().ceiling(key),
            Some(Ordering::Equal) | None => &None,
        }
    }

    // 前序遍历
    fn pre_order(&self) -> Vec<&Node<K, V>> {
        let mut stack : Vec<&Node<K, V>> = Vec::new();
        let mut res : Vec<&Node<K, V>> = Vec::new();

        if self.is_some() {
            stack.push(self.as_ref().unwrap());

            while ! stack.is_empty() {
                let node = stack.pop().unwrap();
                res.push(node);

                if node.right.is_some() {
                    stack.push(node.right.as_ref().unwrap());
                }

                if node.left.is_some() {
                    stack.push(node.left.as_ref().unwrap());
                }
            }
        }

        res
    }

    // 中序遍历
    fn in_order(&self) -> Vec<&Node<K, V>> {
        let mut stack : Vec<&Node<K, V>> = Vec::new();
        let mut res : Vec<&Node<K, V>> = Vec::new();
        let mut p = self;

        while p.is_some() || ! stack.is_empty() {
            while p.is_some() {
                let node = p.as_ref().unwrap();
                stack.push(node);
                p = &node.left;
            }

            let cur = stack.pop().unwrap();
            res.push(cur);
            p = &cur.right;
        }

        res
    }

    // 后序遍历
    fn post_order(&self) -> Vec<&Node<K, V>> {
        let mut stack : Vec<&Node<K, V>> = Vec::new();
        let mut res : Vec<&Node<K, V>> = Vec::new();
        let mut rev : Vec<&Node<K, V>> = Vec::new();

        if self.is_some() {
            stack.push(self.as_ref().unwrap());

            while ! stack.is_empty() {
                let node = stack.pop().unwrap();
                res.push(node);

                if node.left.is_some() {
                    stack.push(node.left.as_ref().unwrap());
                }

                if node.right.is_some() {
                    stack.push(node.right.as_ref().unwrap());
                }
            }

            for node in res.iter().rev() {
                rev.push(node);
            }
        }

        rev
    }

    // 层级遍历
    fn level_order(&self) -> Vec<&Node<K, V>> {
        use std::collections::VecDeque;

        let mut queue : VecDeque<&Node<K, V>> = VecDeque::new();
        let mut res: Vec<&Node<K, V>> = Vec::new();

        if self.is_some() {
            queue.push_back(self.as_ref().unwrap());

            while ! queue.is_empty() {
                let node = queue.pop_front().unwrap();
                res.push(node);

                if node.left.is_some() {
                    queue.push_back(node.left.as_ref().unwrap());
                }

                if node.right.is_some() {
                    queue.push_back(node.right.as_ref().unwrap());
                }
            }
        }

        res
    }
}


#[derive(Debug)]
pub struct RedBlackTree<K, V> {
    root: Link<K, V>,
}

impl<K: PartialOrd, V> RedBlackTree<K, V> {
    pub fn new() -> Self {
        RedBlackTree { root: None }
    }

    pub fn put(&mut self, key: K, val: V) {
        self.root.put(key, val);
    }

    pub fn get(&self, key: K) -> Option<&V> {
        self.root.get(key)
    }

    pub fn delete(&mut self, key: K) {
        if ! self.root.left().is_red() && ! self.root.right().is_red() {
            self.root.as_mut().map(|node| node.color = Colors::RED);
        }

        self.root.delete(key);

        if self.root.size() > 0 {
            self.root.as_mut().map(|node| node.color = Colors::BLACK);
        }
    }

    pub fn delete_min(&mut self) {
        if ! self.root.left().is_red() && ! self.root.right().is_red() {
            self.root.as_mut().map(|node| node.color = Colors::RED);
        }

        self.root.delete_min();

        if self.root.size() > 0 {
            self.root.as_mut().map(|node| node.color = Colors::BLACK);
        }
    }

    pub fn delete_max(&mut self) {
        if ! self.root.left().is_red() && ! self.root.right().is_red() {
            self.root.as_mut().map(|node| node.color = Colors::RED);
        }

        self.root.delete_max();

        if self.root.size() > 0 {
            self.root.as_mut().map(|node| node.color = Colors::BLACK);
        }
    }

    pub fn size(&self) -> usize {
        self.root.size()
    }

    pub fn min(&self) -> &Link<K, V> {
        self.root.min()
    }

    pub fn max(&self) -> &Link<K, V> {
        self.root.max()
    }

    pub fn select(&self, k: usize) -> &Link<K, V> {
        self.root.select(k)
    }

    pub fn rank(&self, key: K) -> usize {
        self.root.rank(key)
    }

    pub fn floor(&self, key: K) -> &Link<K, V> {
        self.root.floor(key)
    }

    pub fn ceiling(&self, key: K) -> &Link<K, V> {
        self.root.ceiling(key)
    }

    pub fn pre_order(&self) -> Vec<&Node<K, V>> {
        self.root.pre_order()
    }

    pub fn in_order(&self) -> Vec<&Node<K, V>> {
        self.root.in_order()
    }

    pub fn post_order(&self) -> Vec<&Node<K, V>> {
        self.root.post_order()
    }

    pub fn level_order(&self) -> Vec<&Node<K, V>> {
        self.root.level_order()
    }
}


#[test]
fn test() {
    let mut tree = RedBlackTree::<&str, isize>::new();
    // A C E H M R S X
    tree.put("S", 1);
    tree.put("E", 2);
    tree.put("X", 3);
    tree.put("A", 4);
    tree.put("R", 5);
    tree.put("C", 6);
    tree.put("H", 7);
    tree.put("M", 8);

    // 不存在树中的key, 获取前继元素和后继元素
    assert_eq!(tree.floor("J").as_ref().unwrap().key, "H");
    assert_eq!(tree.ceiling("J").as_ref().unwrap().key, "M");

    // 存在树中的key, 获取前继元素和后继元素
    assert_eq!(tree.floor("R").as_ref().unwrap().key, "M");
    assert_eq!(tree.ceiling("R").as_ref().unwrap().key, "S");

    // 最小值和最大值
    assert_eq!(tree.min().as_ref().unwrap().key, "A");
    assert_eq!(tree.max().as_ref().unwrap().key, "X");

    // 选择第k个元素
    assert_eq!(tree.select(0).as_ref().unwrap().key, "A");
    assert_eq!(tree.select(1).as_ref().unwrap().key, "C");
    assert_eq!(tree.select(2).as_ref().unwrap().key, "E");
    assert_eq!(tree.select(3).as_ref().unwrap().key, "H");
    assert_eq!(tree.select(4).as_ref().unwrap().key, "M");
    assert_eq!(tree.select(5).as_ref().unwrap().key, "R");
    assert_eq!(tree.select(6).as_ref().unwrap().key, "S");
    assert_eq!(tree.select(7).as_ref().unwrap().key, "X");
    assert!(tree.select(8).is_none());

    // 查看元素的排名
    assert_eq!(tree.rank("A"), 0);
    assert_eq!(tree.rank("C"), 1);
    assert_eq!(tree.rank("E"), 2);
    assert_eq!(tree.rank("H"), 3);
    assert_eq!(tree.rank("M"), 4);
    assert_eq!(tree.rank("R"), 5);
    assert_eq!(tree.rank("S"), 6);
    assert_eq!(tree.rank("X"), 7);

    // 查看元素个数
    assert_eq!(tree.size(), 8);

    // 获取值
    assert_eq!(tree.get("S"), Some(&1));

    // 删除最小元素
    tree.delete_min();
    assert_eq!(tree.size(), 7);
    assert!(tree.get("A").is_none());
    assert_eq!(tree.select(0).as_ref().unwrap().key, "C");

    // 删除最大元素
    tree.delete_max();
    assert_eq!(tree.size(), 6);
    assert!(tree.get("X").is_none());
    assert_eq!(tree.select(5).as_ref().unwrap().key, "S");

    // 根据key删除元素
    tree.delete("S");
    assert_eq!(tree.size(), 5);
    assert!(tree.get("S").is_none());

    tree.pre_order();
}