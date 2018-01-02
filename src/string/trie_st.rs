// 字典树
use std::fmt;
const R: usize = 256;

type Link<T> = Option<Box<Node<T>>>;

#[derive(Debug)]
struct Node<T> {
    val: Option<T>,
    next: Vec<Link<T>>
}

trait LinkMethods<T> {
    fn new() -> Link<T>;
    fn get(&self, key: &str, d: usize) -> &Link<T>;
    fn put(&mut self, key: &str, val: T, d: usize) -> Link<T>;
}

impl<T: fmt::Debug> LinkMethods<T> for Link<T> {
    fn new() -> Self {
        let mut this = Box::new(Node {
            val: None,
            next: Vec::with_capacity(R),
        });

        for _ in 0..R {
            this.next.push(None);
        }

        Some(this)
    }

    fn get(&self, key: &str, d: usize) -> &Self {
        match *self {
            None => &self,
            Some(_) if d == key.chars().count() => &self,
            Some(ref boxed_node) => {
                let c = key.chars().nth(d).unwrap() as usize;
                boxed_node.next[c].get(key, d + 1)
            }
        }
    }

    fn put(&mut self, key: &str, val: T, d: usize) -> Link<T> {
        let mut x = match self.take() {
            Some(mut boxed_node) => boxed_node,
            None => Self::new().unwrap(),
        };

        if d == key.chars().count() {
            x.val = Some(val);
        }
        else {
            let c = key.chars().nth(d).unwrap() as usize;
            x.next[c] = x.next[c].put(key, val, d + 1);
        }

        Some(x)
    }
}

#[derive(Debug)]
pub struct TrieST<T> {
    root: Link<T>,
}

impl<T: fmt::Debug> TrieST<T> {

    pub fn new() -> Self {
        TrieST { root: None }
    }

    pub fn get(&self, key: &str) -> &Option<T> {
        match *self.root.get(key, 0) {
            Some(ref boxed_node) => &boxed_node.val,
            None => &None,
        }
    }

    pub fn put(&mut self, key: &str, val: T) {
        self.root = self.root.put(key, val, 0);
    }
}


#[test]
fn test() {
    let mut trie_st = TrieST::new();

    trie_st.put("abc", 1);
    trie_st.put("cbd", 2);
    trie_st.put("bde", 3);
    trie_st.put("def", 4);

    assert_eq!(trie_st.get("def"), &Some(4));
}