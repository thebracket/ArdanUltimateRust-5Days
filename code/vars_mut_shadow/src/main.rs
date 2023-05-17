// Note that I've used scopes to separate the examples, rather than
// creating a large number of tiny projects.
//
// A scope "drops" every variable inside it when it ends. This allows
// easy separation of sections.
fn main() {
    {
        // Basic variable
        let n = 5;
        println!("{n}");
    }

    /*{
        // Cannot change an immutable variable
        let n = 5;
        n = n + 1; // <-- Will not compile
        println!("{n}");
    }*/

    {
        // Mutable variable
        let mut n = 5;
        n += 1;
        println!("{n}");
    }

    {
        // Shadowing
        let n = 5;
        let n = n + 1;
        // Where did n go?
        println!("{n}");
    }

    {
        // Scope Shadowing
        let n = 5;
        {
            let n = 6;
            println!("{n}");
        }
        println!("{n}");
    }

    {
        // Scope return
        let n = {
            let mut accumulator = 0;
            for i in 1..10 {
                accumulator += i;
            }
            accumulator // No semicolon - this is the return value
        };
        println!("{n}");
    }

    {
        // The Unit Type
        #[allow(clippy::let_unit_value)] // Ignore the linter warning, this is for demonstration
        let n = {
            println!("Hello World!");
        };
        println!("{n:?}"); // :? is a debug formatter
    }
}
