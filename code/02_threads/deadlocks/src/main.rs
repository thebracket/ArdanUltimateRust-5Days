use std::sync::Mutex;

static MY_SHARED : Mutex<u32> = Mutex::new(0);

fn main() {
    /*
    // Deadlock
    let lock = MY_SHARED.lock().unwrap();
    let lock = MY_SHARED.lock().unwrap();
    */

    // Try Lock
    /*
    if let Ok(_lock) = MY_SHARED.try_lock() {
        println!("I got the lock!");

        // Try again, but this time, the lock is already taken
        if let Ok(_lock) = MY_SHARED.try_lock() {
            println!("I got the lock!");
        } else {
            println!("I couldn't get the lock!");
        }

    } else {
        println!("I couldn't get the lock!");
    }*/

    /*
    // Explicitly dropping a lock
    let lock = MY_SHARED.lock().unwrap();
    std::mem::drop(lock);
    let lock = MY_SHARED.lock().unwrap();
    */

    // Using a scope to drop a lock
    {
        let _lock = MY_SHARED.lock().unwrap();
        println!("I got the lock!");
    }
    let _lock = MY_SHARED.lock().unwrap();
    println!("I got the lock again!");
}
