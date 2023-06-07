# The Drop Trait and RAII (Resource Acquisition is Initialization)

When C++ came along, it introduced two really useful concepts for an Object-Oriented Programming (OOP) language. Rust isn't OOP, but it inherits a lot of similarities. The two concepts are:

* Constructors
* Destructors

A `constructor` is used to initialize an object or data structure. You've used them:

```rust
struct MyStruct {n: i32}

impl MyStruct {
    fn new() -> MyStruct {
        MyStruct {n: 42}
    }
}
```

> Not adding `self` to the parameter list makes it an "associated function" - the struct acts as a namespace and you can call the function as `MyStruct::new`.

In C++, it's not uncommon to have to write 5 different constructors! (Default, copy, copy assign, move and move assign). Rust won't do that to you!

The corollary to a constructor is a `destructor`. This is used to clean up an object or data structure. Lots of OOP languages have destructors of varying utility; when you start using J2EE object pools, when the destructor will be called becomes a fascinating case of frustration! In C++, you can be *sure* that the destructor will fire:

* When an object leaves scope (via the stack)
* When an object is deleted (via the heap)

## Droppable Structures

Rust doesn't call them destructors, but provides a similar concept: the `Drop` trait.

> The code for this is in the `code/04_mem/drop_trait` folder.

Let's make a new type:

```rust
struct Droppable(i32);

impl Droppable {
    fn new(n: i32) -> Self {
        println!("Constructing {n}");
        Self(n)
    }
}

impl Drop for Droppable {
    fn drop(&mut self) {
        println!("Dropping {}", self.0);
    }
}
```

We've made a constructor and a destructor (a `Drop` implementation). Let's use it:

```rust
fn main() {
    let _x = Droppable::new(1);
    let _y = Droppable::new(2);
    let _z = Droppable::new(3);
    println!("Exiting main");
}
```

Running the program shows you that everything is constructed and dropped in the right order:

```text
Constructing 1
Constructing 2
Constructing 3
Exiting main
Dropping 3
Dropping 2
Dropping 1
```

You can use a scope and create a `droppable` in there---the `Drop` happens at the end of the scope.

```rust
{
    let _scoped = Droppable::new(4);
    println!("Ending scope");
}
println!("Ended scope");
```

The scope ends the existence of droppable 4, and the destructor fires:

```text
Constructing 4
Ending scope
Dropping 4
Ended scope
```

Likewise, you can *move* it into a function and the destructor will fire when the function ends:

```rust
fn move_me(x: Droppable) {
    println!("Moved {}", x.0);
}
```

And then call it:

```rust
let a = Droppable::new(5);
move_me(a);
println!("Function returned");
```

And no reconstructing occurs, and the `drop` fires when the function ends:

```text
Constructing 5
Moved 5
Dropping 5
Function returned
```

## Transitive Dropping

So what happens if you have a structure that contains droppable items?

Let's find out:

```rust
struct HasDrop {
    id: i32,
    d: Droppable,
}

impl HasDrop {
    fn new(id: i32) -> Self {
        Self {
            id,
            d: Droppable::new(id + 100),
        }
    }
}
```

and in `main()`:

```rust
// Transitive drops
let _b = HasDrop::new(600);
```

When you run the program:

```rust
Constructing 700
Exiting main
Dropping 700
```

Rust has done the right thing, and dropped the *contained* resources for you.

## RAII - Resource Acquisition is Initialization

This pattern can be combined with *resources*. Memory, files, etc. Wrapping the resource in a type, and implementing `Drop` to close it. C++ invented this paradigm, it led to an immediate improvement over C:

* No more `goto` to cleanup resources.
* No more forgetting to cleanup resources.

This is why you haven't had to deal with memory or resource management: the RAII pattern is built into Rust, and every `File`, `Mutex`, `Box`, `Drop`, `String` (etc.) are implementing `Drop` in some way to ensure that you don't leak any memory or resources.

> This example code is in `code/04_mem/smart_ptr`.

So let's take the memory allocation example and turn it into a "smart pointer"---a pointer that will clean up after itself.

```rust
use std::alloc::{Layout, alloc, dealloc};

struct SmartPointer<T> {
    ptr: *mut u8,
    data: *mut T,
    layout: Layout
}

impl <T> SmartPointer<T> {
    fn new() -> SmartPointer<T> {
        println!("Allocating memory for SmartPointer");

        unsafe {
            let layout = Layout::new::<T>();
            let ptr = alloc(layout);

            SmartPointer {
                ptr,
                data: ptr as *mut T,
                layout
            }
        }
    }

    fn set(&mut self, val: T) {
        unsafe {
            *self.data = val;
        }
    }

    fn get(&self) -> &T {
        unsafe {
            self.data.as_ref().unwrap()
        }
    }
}

impl <T> Drop for SmartPointer<T> {
    fn drop(&mut self) {
        println!("Deallocating memory from SmartPointer");
        unsafe {
            dealloc(self.ptr, self.layout);
        }
    }
}

fn main() {
    let mut my_num = SmartPointer::<i32>::new();
    my_num.set(12);
    println!("my_num = {}", my_num.get());
}
```

Run the program, and you'll see that the memory is properly deallocated---and the interface for the user is quite straightforward.

## And now the for good news!

You just implemented part of `Box`, a built-in Rust type. Box is a smart pointer, and has lots of niceties pre-implemented. It manages all of the allocation and de-allocation for you---as well as proving a much easier interface. (`Box` actually uses a more complicated set of allocation calls)

So you can replace the entirety of the `SmartPointer` with:

```rust
let my_num = Box::new(12u32);
println!("my_num = {}", *my_num);
```

All the functionality---and it's built in. (C++ has `unique_ptr` to do the same thing)

### You now understand `Box`, `Vec`, `String`...

So `Box` is a smart pointer that ensures that your data is freed when you are done with it.

`Vec` stores its type, length and capacity. The actual big blob of heap memory that stores the data? It's inside a `Box`. `Vec` just takes advantage of the `Drop` trait to have `Box` do the cleanup.

A `String` is a `Vec` of bytes, with the bytes mapped to unicode (UTF-8) characters (`char` in Rust - anywhere from 1 to 8 bytes per char). And since `Vec` is using a `Box`---it's smart pointers using `Drop` all the way down.

So that's why you haven't had to manually clean up any vectors, strings, boxes, etc. You've been using smart pointers all along.

### You Understand Closing Channels, Network Connections, Files, etc. Too!

The RAII/`Drop` pattern is used for `File`, `mpsc` channels, `TCPStream` and other types that "wrap" a resource. When the object goes out of scope, `Drop` fires and the resource is closed for you. That's why you've been able to open channels and never close them, accept TCP connections without ever cleaning up afterwards, etc.

With RAII, you get a "single ownership" model. The object that owns the resource is the only one that can clean it up. This is a *huge* improvement over C, where you can have multiple pointers to the same resource, and you have to be careful to clean up after yourself. It does limit you to moving the resource around---but that's why Rust is move-by-default. In some cases, it simply doesn't make sense to have multiple owners of a resource.

In other cases, you can use [Reference Counting](./ReferenceCounting.md)...