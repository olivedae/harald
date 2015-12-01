use std::cmp::Ordering;

pub struct Node<K, V> {
    key:   K,
    val:   V,
    left:  Option<Box<Node<K, V>>>,
    right: Option<Box<Node<K, V>>>,
    sz:    u8,
}

impl<K: Ord, V> Node<K, V> {
    pub fn new(key: K, value: V, _size: u8) -> Node<K, V> {
        Node {
            key:   key,
            val:   value,
            left:  None,
            right: None,
            sz:    1,   /* rustc is returning a mismatched type error if _size is used */
        }
    }
}

pub struct BSTMap<K, V> {
    root: Option<Box<Node<K, V>>>,
}

/*
 * BSTMap exposes a public interface for generic
 * features while under the hood implements
 * it through recursion. Currently these
 * methods can be identified with a prefix underscore.
 *
 * Further along I will like to see
 * this implmentation switched with function overloading
 * through traits.
*/

impl<K: Ord, V> BSTMap<K, V> {
    pub fn new() -> BSTMap<K, V> {
        BSTMap {
            root: None,
        }
    }

    pub fn get(&self, key: &K) -> Option<V> {
        self._get(Some(self.root), *key)
    }

    fn _get(&self, node: Option<Node<K, V>>, key: K) -> Option<V> {
        match node.unwrap() {
            Some(node) => {
                match *key.cmp(node.key) {
                    Ordering::Less    => self._get(Some(node.left), key),
                    Ordering::Greater => self._get(Some(node.right), key),
                    Ordering::Equal   => Some(node.val),
                }
            },
            None => None
        }
    }

    pub fn put(&self, key: K, value: V) {
        self.root = self._put(self.root, key, value);
    }

    fn _put(&self, node: Option<Node<K, V>>, key: K, val: V) -> Option<Node<K, V>> {
        match node.unwrap() {
            Some(node) => {
                match key.cmp(node.key) {
                    Ordering::Less    => node.left = self._put(node.left, key, val),
                    Ordering::Greater => node.right = self._put(node.left, key, val),
                    Ordering::Equal   => node.val = val
                }
            },
            None => return Node::new(key, val, 1)
        }
        node.sz = self._size(node.right) + self._size(node.left) + 1;
        return node
    }

    pub fn size(&self) -> u8 {
        self._size(&self.root)
    }

    fn _size(&self, node: Option<Node<K, V>>) -> u8 {
        match node.unwrap() {
            Some(node) => node.sz,
            None => 0
        }
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    fn settup_bst(length: usize) -> (BSTMap<usize, usize>, Vec<usize>) {
        let mut sorts: Vec<usize> = Vec::with_capity(length);
        let mut map: BSTMap<usize, usize> = BSTMap::new();

        for k in 0..length {
            let v = k+5;
            map.put(k, v);
            sorts.push(k);
        }
        (map, sorts.sort())
    }

    #[test]
    fn test_instance() {
        let map: BSTMap<i32, i32> = BSTMap::new();
        assert!(map.root.unwrap() == None);
    }

    #[test]
    fn test_is_sorted() {
        let (map, sorted) = settup_bst(10);
        assert_eq!(map.tree(), sorted);
    }

    #[test]
    fn test_get() {
        let (map, sorted) = settup_bst(10);
        let key = 5;
        let correct_val = key + 5;  /* settup function sets value to key + 5 */
        assert_eq!(map.get(key), correct_val);
    }

    #[test]
    fn test_size() {
        let (map, sorted) = settup_bst(10);
        assert_eq!(map.sz(), 10);
    }

    #[test]
    fn test_delete() {
        let (map, sorted) = settup_bst(10);
        map.delete(5);
        assert_eq!(map.get(5), None);
    }
}
