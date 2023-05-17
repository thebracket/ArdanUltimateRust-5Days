fn greet(s: &String) {
    println!("Hello, {s}");
}

fn greet_mut(s: &mut String) {
    *s = format!("Hello {s}");
    println!("{s}");
}

fn main() {
    {
        // Immutable reference
        let mut name = "Hello".to_string();
        greet(&name);
        name += " World";
    }

    {
        // Mutable reference
        let mut name = "Hello".to_string();
        greet_mut(&mut name);
        name += " World";
        println!("{name}");
    }
}
