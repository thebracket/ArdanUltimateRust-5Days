# Traits

You've used traits a lot---they are an important part of Rust. But we haven't really talked about them.

## Implementing Traits

Whenever you've used `#[derive(Debug, Clone, Serialize)]` and similar---you are using procedural macros to implement traits. We're not going to dig into procedural macros---they are worthy of their own class---but we will look at what they are doing.

`Debug` is a *trait*. The derive macro is implementing the trait for you (including identifying all of the fields to output). You can implement it yourself:

```rust
use std::fmt;

struct Point {
    x: i32,
    y: i32,
}

impl fmt::Debug for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Point")
         .field("x", &self.x)
         .field("y", &self.y)
         .finish()
    }
}
```

Traits are an *interface*. Each trait defines functions that must be implemented to apply the trait to a type. Once you implement the trait, you can use the trait's functions on the type---and you can also use the trait as a type.

## Making a Trait

> The code for this is in `code/04_mem/make_trait`.

Let's create a very simple trait:

```rust
trait Animal {
    fn speak(&self);
}
```

This trait has one function: `speak`. It takes a reference to `self` (the type implementing the trait) and returns nothing.

> Note: trait parameters are also part of the interface, so if a trait entry needs `&self`---all implementations of it will need `&self`.

Now we can make a cat:

```rust
struct Cat;

impl Animal for Cat {
    fn speak(&self) {
        println!("Meow");
    }
}
```

Now you can run `speak()` on any `Cat`:

```rust
fn main() {
    let cat = Cat;
    cat.speak();
}
```

You could go on and implement as many speaking animals as you like.

## Traits as Function Parameters

You can also create functions that require that a parameter implement a trait:

```rust
fn speak_twice(animal: &impl Animal) {
    animal.speak();
    animal.speak();
}
```

You can call it with `speak_twice(&cat)`---and it runs the trait's function twice.

## Traits as Return Types

You can also return a trait from a function:

```rust
fn get_animal() -> impl Animal {
    Cat
}
```

The fun part here is that you no-longer know the concrete type of the returned type---you know for sure that it implements `Animal`. So you can call `speak` on it, but if `Cat` implements other traits or functions, you can't call those functions.

## Traits that Require Other Traits

You could require that all `Animal` types require `Debug` be also implemented:

```rust
trait Animal: Debug {
    fn speak(&self);
}
```

Now `Cat` won't compile until you derive (or implement) `Debug).

You can keep piling on the requirements:

```rust
trait DebuggableClonableAnimal: Animal+Debug+Clone {}
```

Let's make a Dog that complies with these rules:

```rust
#[derive(Debug, Clone)]
struct Dog;

impl Animal for Dog {
    fn speak(&self) {
        println!("Woof");
    }
}

impl DebuggableClonableAnimal for Dog {}
```

Now you can make a dog and call `speak` on it. You can also use `DebuggableCloneableAnimal` as a parameter or return type, and be sure that all of the trait functions are available.

## Dynamic Dispatch

All of the examples above can be resolved at *compile time*. The compiler knows the concrete type of the trait, and can generate the code for it. But what if you want to store a bunch of different types in a collection, and call a trait function on all of them?

You might want to try this:

```rust
let animals: Vec<impl Animal> = vec![Cat, Dog];
```

And it won't work. The *reason* it won't work is that `Vec` stores identical entries for each record. That means it needs to know the size of the entry. Since cats and dogs might be of different sizes, `Vec` can't store them.

You can get around this with *dynamic dispatch*. You've seen this once before, with `type GenericResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;`. The `dyn` keyword means that the type is *dynamic*---it can be different sizes.

Now think back to boxes. Boxes are a *smart-pointer*. That means they occupy the size of a *pointer* in memory, and that pointer tells you where the data actually is in the heap. So you *can* make a vector of dynamic, boxed traits:

```rust
let animals: Vec<Box<dyn Animal>> = vec![Box::new(Cat), Box::new(Dog)];
```

Each vector entry is a pointer (with a type hint) to a trait. The trait itself is stored in the heap. Accessing each entry requires a pointer dereference and a virtual function call. (A `vtable` will be implemented, but often optimized away---LLVM is very good at avoiding making vtables when it can).

In the threads class, someone asked if you could "send interfaces to channels". And yes, you can---you have to use dynamic dispatch to do it. This is valid:

```rust
let (tx, rx) = std::sync::mpsc::channel::<Box<dyn Animal>>();
```

> This works with other pointer types like `Rc`, and `Arc`, too. You can have a reference-counted, dynamic dispatch pointer to a trait.

Using dynamic dispatch won't perform as well as static dispatch, because of pointer chasing (which reduces the likelihood of a memory cache hit).

## The `Any` Type

If you really, *really* need to find out the concrete type of a dynamically dispatched trait, you can use the `std::any::Any` trait. It's not the most efficient design, but it's there if you *really* need it.

The easiest way to "downcast" is to require `Any` in your type and an `as_any` function:

```rust
struct Tortoise;

impl Animal for Tortoise {
    fn speak(&self) {
        println!("What noise does a tortoise make anyway?");
    }
}

impl DowncastableAnimal for Tortoise {
    fn as_any(&self) -> &dyn Any {
        self
    }
}
```

Then you can "downcast" to the concrete type:

```rust
let more_animals : Vec<Box<dyn DowncastableAnimal>> = vec![Box::new(Tortoise)];
for animal in more_animals.iter() {
    if let Some(cat) = animal.as_any().downcast_ref::<Tortoise>() {
        println!("We have access to the tortoise");
    }
    animal.speak();
}
```

If you can avoid this pattern, you should. It's not very Rusty---it's pretending to be an object-oriented language. But it's there if you need it.

## Implementing Operators

> "Operator overloading" got a bad name from C++. You *can* abuse it, and decide that operators do bizarre things. Please don't. If you allow two types to be added together, please use an operation that makes sense to the code reader!

> See the `04_mem/operator_overload` project.

You can implement operators for your types. Let's make a `Point` type that can be added together:

```rust
use std::ops::Add;

struct Point {
    x: f32,
    y: f32,
}

impl Add for Point {
    type Output = Point;

    fn add(self, rhs: Self) -> Self::Output {
        Point {
            x: self.x + rhs.x, 
            y: self.y + rhs.y
        }
    }
}

fn main() {
    let a = Point { x: 1.0, y: 2.0 };
    let b = Point { x: 3.0, y: 4.0 };
    let c = a + b;
    println!("c.x = {}, c.y = {}", c.x, c.y);
}
```

There's a full range of operators you can overload. You can also overload the `+=`, `/`, `*` operators, and so on. This is very powerful for letting you express functions (rather than remembering to add `x` and `y` each time)---but it can be abused horribly if you decide that `+` should mean "subtract" or something. Don't do that. Please.