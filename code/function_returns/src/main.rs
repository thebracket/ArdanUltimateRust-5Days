fn double(n: i32) -> i32 {
    n * 2
}

#[allow(dead_code)] // <- Allow unused function
fn double_or_nothing(n: i32) -> i32 {
    if n > 0 {
        return n * 2;
    }
    0
}

fn main() {
    {
        // Call double
        let n = double(5);
        println!("{n}");
    }

    /*{
        // Scope return
        println!("{}", double_or_nothing(5));
        println!("{}", double_or_nothing(-5));
        println!("{}", {
            let n = 12;
            return n; // <- Does not compile
        });
    }*/
}
