# Setup Your Rust Development Environment

> Hopefully, you've already done some of this.

## Rust Setup with RustUp

If you haven't already, you need to have Rust installed on your computer.
Visit (RustUp)[https://rustup.rs/] and install from there. Instructions
will vary by platform.

![](/images/RustUp.png)

### Things to Know About RustUp

There are some caveats about RustUp that you need to know:

* RustUp installs Rust *for your user account*. It doesn't perform a shared installation.
* You *can* get a working environment through package managers such as `snap`, `apt`, `homebrew` and similar. It's better to use the native Rust setup, you get much faster access to updates.
* On Windows, if you don't already have it installed you will be prompted to also install the Visual C++ build tools. 
* On a Mac, you will be prompted to install the OS X development tools if you haven't already.
* On Linux, make sure you have LLVM installed.

### Verify that you have a working RustUp

> This will occur in a live demo.

Enter the command `rustup --version`. You should see something like this:

```
rustup 1.25.1 (bb60b1e89 2022-07-12)
info: This is the version for the rustup toolchain manager, not the rustc compiler.
info: The currently active `rustc` version is `rustc 1.65.0 (897e37553 2022-11-02)`
```

> *The version numbers will change.*

**If you plan to code along, please take a second to make sure that Rust is installed and working.**

### Update to the Latest Stable Rust

> This will occur in a live demo.

**Let's take a second to make sure you have an up-to-date Rust install. We're going to use some of the more recent additions to the language, so you don't want to find yourself wondering why something didn't work. In particular, the borrow checker rules have eased up and printing with placeholders has become a lot easier.**

From time to time (I do monthly), it's a good idea to update your Rust setup. Bug fixes and
performance improvements are applied periodically. Rust won't break *stable* unless a
major security vulnerability emerges.

**To update RustUp itself**:

`rustup self update`

You'll see something like this:

```
info: checking for self-updates
info: downloading self-update
  rustup updated - 1.25.2 (from 1.25.1)
```

**To update Rust to the latest**:

> This will occur in a live demo.

Enter:

```
rustup update
```

Rust will update all the toolchains you have installed. You'll see something like this:

```
info: syncing channel updates for 'stable-x86_64-pc-windows-msvc'
info: latest update on 2023-01-26, rust version 1.67.0 (fc594f156 2023-01-24)
info: downloading component 'llvm-tools'
 44.2 MiB /  44.2 MiB (100 %)   7.3 MiB/s in  6s ETA:  0s
info: downloading component 'rust-std' for 'wasm32-unknown-unknown'
 18.9 MiB /  18.9 MiB (100 %)   8.0 MiB/s in  2s ETA:  0s
info: downloading component 'rust-src'
info: downloading component 'cargo'
info: downloading component 'clippy'
info: downloading component 'rust-docs'
 19.3 MiB /  19.3 MiB (100 %)   7.4 MiB/s in  2s ETA:  0s
info: downloading component 'rust-std'
 26.8 MiB /  26.8 MiB (100 %)   6.4 MiB/s in  3s ETA:  0s
info: downloading component 'rustc'
 62.7 MiB /  62.7 MiB (100 %)   7.4 MiB/s in  8s ETA:  0s
info: downloading component 'rustfmt'
info: removing previous version of component 'llvm-tools'
info: removing previous version of component 'rust-std' for 'wasm32-unknown-unknown'
info: removing previous version of component 'rust-src'
info: removing previous version of component 'cargo'
info: removing previous version of component 'clippy'
info: removing previous version of component 'rust-docs'
info: removing previous version of component 'rust-std'
info: removing previous version of component 'rustc'
info: removing previous version of component 'rustfmt'
info: installing component 'llvm-tools'
 44.2 MiB /  44.2 MiB (100 %)  18.6 MiB/s in  4s ETA:  0s
info: installing component 'rust-std' for 'wasm32-unknown-unknown'
 18.9 MiB /  18.9 MiB (100 %)  18.4 MiB/s in  1s ETA:  0s
info: installing component 'rust-src'
  2.4 MiB /   2.4 MiB (100 %)   1.9 MiB/s in  1s ETA:  0s
info: installing component 'cargo'
info: installing component 'clippy'
info: installing component 'rust-docs'
 19.3 MiB /  19.3 MiB (100 %)  60.0 KiB/s in  5m 13s ETA:  0s
info: installing component 'rust-std'
 26.8 MiB /  26.8 MiB (100 %)  17.0 MiB/s in  1s ETA:  0s
info: installing component 'rustc'
 62.7 MiB /  62.7 MiB (100 %)  20.3 MiB/s in  3s ETA:  0s
info: installing component 'rustfmt'
 13 IO-ops /  13 IO-ops (100 %)   0 IOPS in  5s ETA: Unknown
info: checking for self-updates

  stable-x86_64-pc-windows-msvc updated - rustc 1.67.0 (fc594f156 2023-01-24) (from rustc 1.65.0 (897e37553 2022-11-02))

info: cleaning up downloads & tmp directories
```

You'll often see "nothing to update", and you probably won't have as many platforms installed as I do!

> Once you have a stable project, you may want to coordinate updates with your team to ensure that you are all on the same version.

## Adding Toolchains

You may want to target platforms other than the one you are using for development. Rust includes cross-compilation.

For example, you can support WASM by running:

```
rustup target add wasm32-unknown-unknown
```

Once that's present, you can build for WASM with `cargo build --target wasm32-unknown-unknown`.

## Install Clippy!

It should be installed by default, but some older---updated---setups don't always have it. Run the following:

```
rustup component add clippy
```

Hopefully, you'll see:

```
info: component 'clippy' for target 'x86_64-pc-windows-msvc' is up to date
```

>> Everyone doing ok? If you're stuck, let me know before we move on and we'll see if there's an immediate option to help. Otherwise, I'll try and help you at the first break.

>> **Really stuck?** You can use most of this online at [Replit](https://replit.com/~). It won't be as fast as working locally.