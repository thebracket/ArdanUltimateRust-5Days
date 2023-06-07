# Foreign Function Interface - Interacting with Other Languages

Rust behaves very well when talking to other languages---both as a library for other languages to consume, and as a consumer of other languages' libraries.

We'll refer to "C Libraries"---but we really mean *any* language that compiles to a C-friendly library format. C, C++, Go, Fortran, Haskell, and many others can all be consumed by Rust.

## Consuming C Libraries

> The code for this is in `04_mem/c_rust` (C Rust)

Let's start with a tiny C library:

```c
// A simple function that doubles a number
int double_it(int x) {
    return x * 2;
}
```

We'd like to compile this and include it in a Rust program. We can automate compilation by including the ability to compile C (and C++) libraries as part of our build process with the `cc` crate. Rather than adding it with `cargo add`, we want to add it as a *build dependency*. It won't be included in the final program, it's just used during compilation. Open `Cargo.toml`:

```toml
[package]
name = "c_rust"
version = "0.1.0"
edition = "2021"

[dependencies]

[build-dependencies]
cc = "1"
```

Now we can create a `build.rs` file in the root of our project (*not* the `src` directory). This file will be run as part of the build process, and can be used to compile C libraries. We'll use the `cc` crate to do this:

```rust
fn main() {
    cc::Build::new()
        .file("src/crust.c")
        .compile("crust");
}
```

`build.rs` is automatically compiled and executed when your Rust program builds. You can use it to automate any build-time tasks you want. The `cc` calls will build the listed files and include the linked result in your final program as a static library.

Lastly, let's create some Rust to call the C:

```rust
// Do it by hand
extern "C" {
    fn double_it(x: i32) -> i32;
}

mod rust {
    pub fn double_it(x: i32) -> i32 {
        x * 2
    }
}
```

We've used an `extern "C"` to specify linkage to an external C library. We've also created a Rust version of the same function, so we can compare the two.

Now let's use some unit tests to prove that it works:

```rust
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_double_it() {
        assert_eq!(unsafe { double_it(2) }, 4);
    }

    #[test]
    fn test_c_rust() {
        assert_eq!(unsafe { double_it(2) }, rust::double_it(2));
    }
}
```

And it works when we run `cargo test`.

## Header files and BindGen

> You need LLVM installed (clang 5 or greater) to use this. On Windows, `winget install LLVM.LLVM` will work. Also set an environment variable `LIBCLANG_PATH` to the location of the Clang install. On Windows, `$Env:LIBCLANG_PATH="C:\Program Files\LLVM\bin"`

Larger C examples will include header files. Let's add `crust.h`:

```c
int double_it(int x);
```

And add C to require it:

```c
#include "crust.h"

// A simple function that doubles a number
int double_it(int x) {
    return x * 2;
}
```

We can add it to the `build.rs` file, but it will be ignored (it's just a forward declaration).

Writing the `extern "C"` for a large library could be time consuming. Let's use `bindgen` to do it for us.

Add another build-dependency:

```toml
[build-dependencies]
cc = "1"
bindgen = "0"
```

Now in `build.rs` we'll add some calls to use it:

```rust
let bindings = bindgen::Builder::default()
    .header("src/crust.h")
    .parse_callbacks(Box::new(bindgen::CargoCallbacks))
    .generate()
    .expect("Unable to generate bindings");

// Write the bindings to the $OUT_DIR/bindings.rs file.
let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
bindings
    .write_to_file(out_path.join("bindings.rs"))
    .expect("Couldn't write bindings!");
```

[See this page for details](https://rust-lang.github.io/rust-bindgen/tutorial-3.html)

This is pretty much standard boilerplate, but there are a lot of options available.

Now run `cargo build`. You'll see a new file in `target/debug/build/c_rust-*/out/bindings.rs`. This is the automatically generated bindings file. Let's use it:

```rust
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
```

Your compile time has suffered, but now the header is parsed and Rust bindings are generated automatically. The unit tests should still work.

## Calling Rust from Other Languages

> The code for this is in `04_mem/rust_c` (Rust C)

You can also setup Rust functions and structures for export via a C API. You lose some of the richness of the Rust language ---everything has to be C compatible---but you can still use Rust's safety and performance.

Start with some `Cargo.toml` entries:

```toml
[package]
name = "rust_c"
version = "0.1.0"
edition = "2021"

[lib]
crate-type = ["staticlib"]

[dependencies]
libc = "0.2"
```

Providing a `lib` and `crate-type` section lets you change compilation behavior. We're instructing Rust to build a C-compatible static library (it can also take a `dynlib` for dynamic linkage).

Next, we'll build a single Rust function to export:

```rust
use std::ffi::CStr;

/// # Safety
/// Use a valid C-String!
#[no_mangle]
pub unsafe extern "C" fn hello(name: *const libc::c_char) {
    let name_cstr = unsafe { CStr::from_ptr(name) };
    let name = name_cstr.to_str().unwrap();
    println!("Hello {name}");
}
```

Notice that we're using `c_char` as an array---just like the C ABI. `CStr` and `CString` provide Rust friendly layers between string types, allowing you to convert back and forth. C strings will never be as safe as Rust strings, but this is a good compromise.

We've turned off name mangling, making it easy for the linker to find the function.

The function is also "unsafe"---because it receives an unsafe C string type.

Build the project with `cargo build`, and you'll see that `target/debug/rust_c.lib` (on Windows, `.a` on Linux) has been created. This is the static library that we can link to from C.

Linkage via C requires a header file. In this case, it's pretty easy to just write one:

```c
void hello(char *name);
```

You can now use this in C or another language. In Go, it looks like this:

```go
package main

/*
#cgo LDFLAGS: ./rust_c.a -ldl
#include "./lib/rust_c.h"
*/
import "C"

import "fmt"
import "time"

func main() {
	start := time.Now()
    fmt.Println("Hello from GoLang!")
	duration := time.Since(start)
	fmt.Println(duration)
	start2 := time.Now()
	C.hello(C.CString("from Rust!"))
	duration2 := time.Since(start2)
	fmt.Println(duration2)
}
```

(There's a few microseconds delay in the Rust call, but it's pretty fast! Marshaling the C string in Go is the slowest part).

## Using CBindGen to Write the Header For You

Setup `cbindgen` as a build dependency:

```toml
[package]
name = "rust_c"
version = "0.1.0"
edition = "2021"

[lib]
crate-type = ["staticlib"]

[dependencies]
libc = "0.2"

[build-dependencies]
cbindgen = "0.24"
```

And once again, add a `build.rs` file:

```rust
use std::env;
use std::path::PathBuf;
use cbindgen::Config;


fn main() {
    let crate_dir = env::var("CARGO_MANIFEST_DIR").unwrap();

    let package_name = env::var("CARGO_PKG_NAME").unwrap();
    let output_file = target_dir()
        .join(format!("{}.hpp", package_name))
        .display()
        .to_string();

    let config = Config {
        //namespace: Some(String::from("ffi")),
        ..Default::default()
    };

    cbindgen::generate_with_config(&crate_dir, config)
      .unwrap()
      .write_to_file(&output_file);
}

/// Find the location of the `target/` directory. Note that this may be 
/// overridden by `cmake`, so we also need to check the `CARGO_TARGET_DIR` 
/// variable.
fn target_dir() -> PathBuf {
    if let Ok(target) = env::var("CARGO_TARGET_DIR") {
        PathBuf::from(target)
    } else {
        PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap()).join("target")
    }
}
```

This is [boilerplate from this guide](https://michael-f-bryan.github.io/rust-ffi-guide/cbindgen.html)

Now run `cargo build` and a `target` directory appears - with a header file.