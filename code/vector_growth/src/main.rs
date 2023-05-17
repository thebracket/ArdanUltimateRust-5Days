fn main() {
    let mut my_vector = Vec::new();
    for _ in 0..20 {
        my_vector.push(0);
        println!("Size: {}, Capacity: {}", my_vector.len(), my_vector.capacity());
    }
}