use std::mem;

pub type Link<K, V> = Option<Box<Node<K, V>>>;

#[derive(Debug)]
pub enum Colors {
    RED,
    BLACK,
}

pub trait ST<K, V> {
    fn new(key: K, val: V) -> Link<K, V>;
    fn size(&self) -> usize;
    fn is_red(&self) -> bool;
    fn change_color(&mut self, color: Colors);
    fn change_red(&mut self);
    fn change_black(&mut self);
    fn rotate_left(&mut self);
    fn rotate_right(&mut self);
    fn flip_colors(&mut self);
    fn put(&mut self, key: K, val: V);
}

#[derive(Debug)]
pub struct Node<K, V> {
    key: K,
    val: V,
    n: usize,
    color: Colors,
    left: Link<K, V>,
    right: Link<K, V>,
}

impl<K: PartialOrd, V> ST<K, V> for Link<K, V> {
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

    fn size(&self) -> usize {
        match *self {
            Some(ref boxed_node) => boxed_node.n,
            None => 0,
        }
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

    fn change_color(&mut self, color: Colors) {
        match *self {
            Some(ref mut boxed_node) => {
                boxed_node.color = color;
            },
            None => {},
        }
    }

    fn change_red(&mut self) {
        self.change_color(Colors::RED);
    }

    fn change_black(&mut self) {
        self.change_color(Colors::BLACK);
    }

    fn rotate_left(&mut self) {
        if let Some(mut boxed_node) = self.take() {
            let mut x = boxed_node.right.take().unwrap();
            boxed_node.right = x.left.take();

            mem::swap(&mut x.color, &mut boxed_node.color);
            boxed_node.color = Colors::RED;
            x.n = boxed_node.n;
            boxed_node.n = 1 + boxed_node.left.size() + boxed_node.right.size();

            x.left = Some(boxed_node);
            *self = Some(x);
        }
    }

    fn rotate_right(&mut self) {
        if let Some(mut boxed_node) = self.take() {
            let mut x = boxed_node.left.take().unwrap();
            boxed_node.left = x.right.take();

            mem::swap(&mut x.color, &mut boxed_node.color);
            boxed_node.color = Colors::RED;
            x.n = boxed_node.n;
            boxed_node.n = 1 + boxed_node.left.size() + boxed_node.right.size();

            x.right = Some(boxed_node);
            *self = Some(x);
        }
    }

    fn flip_colors(&mut self) {
        self.change_red();
        self.as_mut().unwrap().left.change_black();
        self.as_mut().unwrap().right.change_black();
    }

    fn put(&mut self, key: K, val: V) {
        match *self {
            Some(ref mut boxed_node) if key == boxed_node.key => {
                boxed_node.val = val;
            },
            Some(ref mut boxed_node) => {
                if key < boxed_node.key {
                    boxed_node.left.put(key, val);
                }
                else {
                    boxed_node.right.put(key, val);
                }

                boxed_node.n = boxed_node.left.size() + boxed_node.right.size() + 1;
            },
            None => {
                *self = Link::new(key, val);
            }
        }

        if ! self.as_ref().unwrap().left.is_red() && self.as_ref().unwrap().right.is_red() {
            self.rotate_left();
        }

        if self.as_ref().unwrap().left.is_red() && self.as_ref().unwrap().left.as_ref().unwrap().left.is_red() {
            self.rotate_right();
        }

        if self.as_ref().unwrap().left.is_red() && self.as_ref().unwrap().right.is_red() {
            self.flip_colors();
        }
    }
}


#[test]
fn test() {
    let mut tree_node = Link::new("A", 1);
    assert_eq!(tree_node.is_red(), true);

    tree_node.change_black();
    assert_eq!(tree_node.is_red(), false);

    tree_node.change_red();
    assert_eq!(tree_node.is_red(), true);
}