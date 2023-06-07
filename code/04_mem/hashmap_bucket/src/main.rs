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

fn main() {
    let mut my_buckets = HashMapBucket::new();
    my_buckets.insert("hello", 1);
    my_buckets.insert("hello", 2);
    my_buckets.insert("goodbye", 3);
    println!("{:#?}", my_buckets.map);
}
