use std::{borrow::Borrow};
use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;
use std::ops::{Index, IndexMut};

struct Entity<Key, Value> {
    key: Key,
    value: Value,
}

pub struct HashTable<Key, Value> {
    buckets: Vec<Vec<Entity<Key, Value>>>,
    elemnts_count: usize,
    capacity: usize,
}

impl<Key: Hash + PartialEq, Value> HashTable<Key, Value> {
    pub fn new() -> Self {
        const INITIAL_CAPACITY: usize = 16;

        let mut buckets = Vec::with_capacity(INITIAL_CAPACITY);

        for _ in 0..INITIAL_CAPACITY {
            buckets.push(Vec::new());
        }

        Self {
            buckets: buckets,
            elemnts_count: 0,
            capacity: INITIAL_CAPACITY,
        }
    }

    pub fn is_empty(&self) -> bool {
        return self.elemnts_count == 0;
    }

    pub fn clear(&mut self) {
        for bucket in &mut self.buckets {
            bucket.clear();
        }

        self.elemnts_count = 0;
    }

    pub fn get<Q>(&self, key: &Q) -> Option<&Value>
    where
        Key: std::borrow::Borrow<Q>,
        Q: PartialEq + Hash,
    {
        let idx = self.hash_key(key) % self.capacity;

        for entity in &self.buckets[idx] {
            if entity.key.borrow() == key {
                return Some(&entity.value);
            }
        }

        return None
    }

    pub fn get_mut<Q>(&mut self, key: &Q) -> Option<&mut Value>
    where
        Key: std::borrow::Borrow<Q>,
        Q: PartialEq + Hash,
    {
        let idx = self.hash_key(key) % self.capacity;

        for entity in &mut self.buckets[idx] {
            if entity.key.borrow() == key {
                return Some(&mut entity.value);
            }
        }

        return None
    }

    pub fn insert(&mut self, key: Key, value: Value)
    {
        if 3 * self.elemnts_count > 4 * self.capacity {
            self.expand()
        }

        let idx = self.hash_key(&key) % self.capacity;

        for entity in &mut self.buckets[idx] {
            if entity.key == key {
                entity.value = value;
                return;
            }
        }

        self.buckets[idx].push(Entity { key:key, value: value });
        self.elemnts_count += 1;
    }

    pub fn remove<Q>(&mut self, key: &Q) -> Option<Value>
    where
        Key: Borrow<Q>,
        Q: PartialEq + Hash,
    {
        let mut remove_index: usize = 0;
        let idx = self.hash_key(key) % self.capacity;
        let bucket_len = self.buckets[idx].len();

        for entity in &self.buckets[idx] {
            if entity.key.borrow() == key {
                break;
            }

            remove_index += 1;
        }

        if remove_index < bucket_len {
            self.elemnts_count -= 1;
            return Some(self.buckets[idx].remove(remove_index).value);
        } else {
            return None;
        }
    }

    pub fn contains_key<Q>(&self, key: &Q) -> bool
    where
        Key: Borrow<Q>,
        Q: PartialEq + Hash,
    {
        let idx = self.hash_key(key) % self.capacity;

        for entity in &self.buckets[idx] {
            if entity.key.borrow() == key {
                return true;
            }
        }

        return false;
    }

    pub fn inter(&self) -> impl Iterator<Item = (&Key, &Value)> {
        return self.buckets
        .iter()
        .flat_map(|bucket| bucket.iter()
            .map(|entity| (&entity.key, &entity.value)));
    }

    fn expand(&mut self) {
        self.capacity *= 2;
        let old_buckets = std::mem::take(&mut self.buckets);
        let mut new_buckets:Vec<Vec<Entity<Key, Value>>> = Vec::with_capacity(self.capacity);
        for _ in 0..self.capacity {
            new_buckets.push(Vec::new());
        }

        for bucket in old_buckets {
            for entity in bucket {
                let idx = self.hash_key(&entity.key) % self.capacity;
                new_buckets[idx].push(entity);
            }
        }
        
        self.buckets = new_buckets;
    }

    fn hash_key<Q>(&self, key: &Q) -> usize
    where
        Key: Borrow<Q>,
        Q: Hash,
    {
        let mut state = DefaultHasher::new();
        key.hash(&mut state);
        return state.finish() as usize;
    }
}

impl<Key, Value, Q> Index<&Q> for HashTable<Key, Value>
where
    Key: Borrow<Q> + PartialEq + Hash,
    Q: PartialEq + Hash,
{
    type Output = Value;

    fn index(&self, index: &Q) -> &Self::Output {
        return self.get(index).expect("no entry found for key");
    }
}

impl<Key, Value, Q> IndexMut<&Q> for HashTable<Key, Value>
where
    Key: Borrow<Q> + PartialEq + Hash,
    Q: PartialEq + Hash,
{
    fn index_mut(&mut self, index: &Q) -> &mut Self::Output {
        return self.get_mut(index).expect("no entry found for key");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn insert_and_get() {
        let mut hashtable:HashTable<String, i32> = HashTable::new();

        hashtable.insert("key".to_string(), 1);
        assert_eq!(hashtable.get(&"key".to_string()), Some(&1));
    }

    #[test]
    fn insert_and_get_mut() {
        let mut hashtable:HashTable<String, i32> = HashTable::new();

        hashtable.insert("key".to_string(), 1);
        if let Some(x) = hashtable.get_mut(&"key".to_string()) {
            *x = 30;
        }

        assert_eq!(hashtable[&"key".to_string()], 30)
    }

    #[test]
    fn contains_key() {
        let mut hashtable:HashTable<i32, i32> = HashTable::new();

        hashtable.insert(1, 1);

        assert_eq!(hashtable.contains_key(&1), true);
    }

    #[test]
    fn remove() {
        let mut hashtable:HashTable<String, i32> = HashTable::new();

        hashtable.insert("key".to_string(), 1);
        hashtable.remove(&"key".to_string());

        assert_eq!(hashtable.is_empty(), true);
        assert_eq!(hashtable.contains_key(&"key".to_string()), false);
    }
}