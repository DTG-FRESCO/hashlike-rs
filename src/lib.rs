use std::{collections::HashMap, hash::Hash};

pub trait HashLike<K, V>
where
    K: Hash + Eq,
{
    fn insert(&mut self, k: K, v: V) -> Option<V>;
    fn remove(&mut self, k: &K) -> Option<V>;
    fn contains_key(&self, k: &K) -> bool;
    fn get(&self, k: &K) -> Option<&V>;
    fn get_mut(&mut self, k: &K) -> Option<&mut V>;
}

impl<K, V> HashLike<K, V> for HashMap<K, V>
where
    K: Hash + Eq,
{
    fn insert(&mut self, k: K, v: V) -> Option<V> {
        self.insert(k, v)
    }

    fn remove(&mut self, k: &K) -> Option<V> {
        self.remove(k)
    }

    fn contains_key(&self, k: &K) -> bool {
        self.contains_key(k)
    }

    fn get(&self, k: &K) -> Option<&V> {
        self.get(k)
    }

    fn get_mut(&mut self, k: &K) -> Option<&mut V> {
        self.get_mut(k)
    }
}
