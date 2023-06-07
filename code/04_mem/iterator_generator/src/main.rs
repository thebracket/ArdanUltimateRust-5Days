struct Counter<const MAX: u32> {
    count: u32,
}

impl <const MAX:u32> Counter<MAX> {
    fn new() -> Self {
        Self { count: 0 }
    }
}

impl <const MAX:u32> Iterator for Counter<MAX> {
    type Item = u32;
    fn next(&mut self) -> Option<Self::Item> {
        if self.count < MAX {
            let result = Some(self.count);
            self.count += 1;
            result
        } else {
            None
        }
    }
}

impl <const MAX:u32> ExactSizeIterator for Counter<MAX> {
    fn len(&self) -> usize {
        MAX as usize
    }
}

fn main() {
    let numbers: Vec<u32> = Counter::<10>::new().collect();
    println!("{numbers:?}");
}
