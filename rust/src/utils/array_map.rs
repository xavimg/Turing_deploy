use std::{ops::{Index, IndexMut}};

/// Map with keys that don't require the ```Hash``` trait
#[derive(Debug)]
pub struct ArrayMap<K: PartialEq, V>(Vec<(K,V)>);

impl<K: PartialEq, V> ArrayMap<K,V> {
    pub fn new () -> Self {
        Self(Vec::new())
    }
    
    pub fn with_capacity (capacity: usize) -> Self {
        Self(Vec::with_capacity(capacity))
    }

    pub fn len (&self) -> usize {
        self.0.len()
    }

    pub fn capacity (&self) -> usize {
        self.0.capacity()
    }
    
    // CONTAINERS
    pub fn contains_key (&self, target: &K) -> bool {
        self.0.iter().any(|(key, _)| key == target)
    }

    pub fn contains_value (&self, target: &V) -> bool where V: PartialEq {
        self.0.iter().any(|(_, value)| value == target)
    }

    fn key_index (&self, target: &K) -> Option<usize> {
        self.0.iter()
            .zip(0..self.len())
            .map(|((x, _), z)| (x, z))
            .find(|(key, _)| *key == target)
            .map(|(_, idx)| idx)
    }

    // GETTER
    pub fn safe_get (&self, idx: K) -> Option<&V> {
        let test = self.0.iter().find(|(key, _)| key == &idx);
        test.map(|(_, value)| value)
    }

    pub fn safe_get_mut (&mut self, idx: K) -> Option<&mut V> {
        let test = self.0.iter_mut().find(|(key, _)| key == &idx);
        test.map(|(_, value)| value)
    }

    // SETER
    pub fn insert (&mut self, key: K, value: V) -> Option<V> {
        match self.key_index(&key) {
            None => { self.0.push((key, value)); None },
            Some(i) => Some(std::mem::replace(&mut self.0[i].1, value))
        }
    }

    pub fn try_insert (&mut self, key: K, value: V) -> Result<(),&V> {
        match self.key_index(&key) {
            None => { self.0.push((key, value)); Ok(()) },
            Some(i) => Err(&self.0[i].1)
        }
    }

    // UPDATER
    pub fn set (&mut self, key: &K, value: V) -> Option<V> {
        self.key_index(&key).map(|i| std::mem::replace(&mut self.0[i].1, value))
    }

    pub fn update<F: Fn(V) -> V> (&mut self, key: &K, f: F) -> bool {
        match self.key_index(key) {
            Some(i) => {
                let mem = &mut self.0[i].1;
                unsafe {
                    let old = std::ptr::read(mem);
                    std::ptr::write(mem, f(old));
                }

                true
            },

            None => false
        }
    }

    pub fn update_else_insert<F: Fn(V) -> V> (&mut self, key: K, update: F, lse: V) {
        match self.key_index(&key) {
            Some(i) => {
                let mem = &mut self.0[i].1;
                unsafe {
                    let old = std::ptr::read(mem);
                    std::ptr::write(mem, update(old));
                }
            },

            None => self.0.push((key, lse))
        };
    }

    // CONCAT
    pub fn concat (self, other: Self) -> Self {
        Self(self.0.into_iter().chain(other.0).collect())
    }

    // ITERS
    pub fn iter (&self) -> impl Iterator<Item = &(K, V)> {
        self.0.iter()
    }

    pub fn iter_mut (&mut self) -> impl Iterator<Item = &mut (K, V)> {
        self.0.iter_mut()
    }

    pub fn key_iter (&self) -> impl Iterator<Item = &K> {
        self.0.iter().map(|(k, _)| k)
    }

    pub fn key_iter_mut (&mut self) -> impl Iterator<Item = &mut K> {
        self.0.iter_mut().map(|(k, _)| k)
    }

    pub fn value_iter (&self) -> impl Iterator<Item = &V> {
        self.0.iter().map(|(_, v)| v)
    }

    pub fn value_iter_mut (&mut self) -> impl Iterator<Item = &mut V> {
        self.0.iter_mut().map(|(_, v)| v)
    }
}

impl<K: PartialEq, V> Index<K> for ArrayMap<K,V> {
    type Output = V;

    fn index(&self, index: K) -> &Self::Output {
        match self.safe_get(index) {
            None => panic!("Key not found"),
            Some(x) => x
        }
    }
}

impl<K: PartialEq, V> IndexMut<K> for ArrayMap<K,V> {
    fn index_mut (&mut self, index: K) -> &mut Self::Output {
        match self.safe_get_mut(index) {
            None => panic!("Key not found"),
            Some(x) => x
        }
    }
}

impl<K: PartialEq, V> IntoIterator for ArrayMap<K,V> {
    type Item = (K,V);
    type IntoIter = <Vec::<(K,V)> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}