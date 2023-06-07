use std::collections::HashMap;

struct HashMapBucket<K,V>
{
    map: HashMap<K, Vec<V>>
}

impl <K,V> HashMapBucket<K,V> 
where K: Eq + std::hash::Hash
{
    fn new() -> Self {
        HashMapBucket {
            map: HashMap::new()
        }
    }

    fn insert(&mut self, key: K, value: V) {
        let values = self.map.entry(key).or_insert(Vec::new());
        values.push(value);
    }
}

impl <K,V> HashMapBucket<K, V> {
    fn iter(&self) -> HashMapBucketIter<K, V> {
        let mut key_iter = self.map.iter();
        let current_map_entry = key_iter.next();
        HashMapBucketIter {
            key_iter,
            current_map_entry,
            current_vec_index: 0,
        }
    }
}

struct HashMapBucketIter<'a, K, V> {
    key_iter: std::collections::hash_map::Iter<'a, K, Vec<V>>,
    current_map_entry: Option<(&'a K, &'a Vec<V>)>,
    current_vec_index: usize,
}

impl <'a, K, V> Iterator for HashMapBucketIter<'a, K, V> {
    type Item = (&'a K, &'a V);

    fn next(&mut self) -> Option<Self::Item> {
        // If there is a current map entry, and a current vec index
        if let Some((key, values)) = self.current_map_entry {
            // If the index is less than the length of the vector
            if self.current_vec_index < values.len() {
                // Get the value at the current index
                let value = &values[self.current_vec_index];
                // Increment the index
                self.current_vec_index += 1;
                // Return the key and value
                return Some((key, value));
            } else {
                // We're past the end of the vector, next key
                self.current_map_entry = self.key_iter.next();
                self.current_vec_index = 0;

                if let Some((key, values)) = self.current_map_entry {
                    // If the index is less than the length of the vector
                    if self.current_vec_index < values.len() {
                        // Get the value at the current index
                        let value = &values[self.current_vec_index];
                        // Increment the index
                        self.current_vec_index += 1;
                        // Return the key and value
                        return Some((key, value));
                    }
                }
            }
        }

        None
    }
}

fn main() {
    let mut my_buckets = HashMapBucket::new();
    my_buckets.insert("hello", 1);
    my_buckets.insert("hello", 2);
    my_buckets.insert("goodbye", 3);
    println!("{:#?}", my_buckets.map);
    let contents: Vec<_> = my_buckets.iter().collect();
    println!("{contents:?}");
}
