# Giving the Collector a Diet

500kb isn't bad, and plenty small enough to run on Raspberry Pi Zero or similar devices. Let's start with the ways Rust can help us make it smaller.

Annoyingly, compiler settings in workspaces are *for the whole workspace*. Normally, at this point I'd break the collector off into its own workspace. For now, let's play with the top-level `Cargo.toml` and see what we can do.

## Optimize for Size

In `Cargo.toml`, you can specify optimization levels by profile. Add this to the `Cargo.toml` file:

```toml
[profile.release]
opt-level = "s"
```

Run `cargo build --release`. It'll take a moment, it has to recompile every dependency and also optimize the dependency for size.

On Windows, the resulting binary is now: 510,976 bytes (499 kb). A small improvement.

There's also an optimization level named "z". Let's see if it does any better?

```toml
[profile.release]
opt-level = "z"
```

It weighs in at 509,440 bytes (497.5 kb). A very tiny improvement.

## Strip the binary

In `Cargo.toml`, let's also strip the binary of symbols.

```toml
[profile.release]
opt-level = "z"
strip = true # Automatically strip symbols
```

Compiling again, this reduces the binary to 508,928 (497 kb).

## Enable LTO

In `Cargo.toml`, let's enable link-time optimization. This optimizes across crate boundaries, at the expense of a SLOW compile.

```toml
[profile.release]
opt-level = "z"
strip = true # Automatically strip symbols
lto = true
```

We're down to 438,272 bytes (428 kb). Getting better!

## Reduce Codegen Units

By default, Rust parallelizes builds across all of your CPUs - which *can* prevent some optimizations. Let's make our compilation even slower in the name of a small binary:

```toml
[profile.release]
opt-level = "z"
strip = true # Automatically strip symbols
lto = true
codegen-units = 1
```

You may have to run `cargo clean` before building this.

Our binary is now 425,472 bytes (415 kb). Another small improvement.

## Abort on Panic

A surprising amount of a Rust binary is the "panic handler". Similar to an exception handler in C++, it adds some hefty code to unwind the stack and provide detailed traces on crashes. We can turn this behavior off:

```toml
[profile.release]
opt-level = "z"
strip = true # Automatically strip symbols
lto = true
codegen-units = 1
panic = "abort"
```

This reduces my binary to 336,896 bytes (329 kb). That's a big improvement! The downside is that if your program panics, it won't be able to tell you all the details about how it died.

## Heavy Measures: Optimize the Standard Library for Size

If you don't have `nightly` installed, you will need it:

```bash
rustup toolchain install nightly
rustup component add rust-src --toolchain nightly
```

Then find out your current build target:

```bash
rustc -vV
```

And use that target to issue a build that includes the standard library:

```bash
cargo +nightly build -Z build-std=std,panic_abort -Z build-std-features=panic_immediate_abort --target x86_64-apple-darwin --release
```

The binary goes into `target/(platform)/release`. There's a pretty substantial size improvement: 177,152 bytes (173 kb)

That's about as small as it gets without using a different standard library. Let's see what we can do about the dependencies.

## Using Cargo Bloat

Install a tool, `cargo install cargo-bloat`. And run `cargo bloat`:

This will give you a report on which parts of your program are using data in the executable:

```
    Analyzing target\debug\collector_v3.exe

 File  .text      Size       Crate Name
 2.0%   2.6%   32.6KiB rand_chacha rand_chacha::guts::refill_wide::impl_sse2
 2.0%   2.6%   32.5KiB rand_chacha rand_chacha::guts::refill_wide::impl_ssse3
 1.9%   2.4%   31.0KiB rand_chacha rand_chacha::guts::refill_wide::impl_avx
 1.8%   2.4%   29.9KiB rand_chacha rand_chacha::guts::refill_wide::impl_sse41
 1.5%   1.9%   24.6KiB rand_chacha rand_chacha::guts::refill_wide::impl_avx2
 0.3%   0.4%    4.5KiB         std core::num::flt2dec::strategy::dragon::format_exact
 0.3%   0.3%    4.3KiB rand_chacha <rand_chacha::chacha::Array64<T> as core::default::Default>::default
 0.2%   0.3%    3.8KiB         std core::num::flt2dec::strategy::dragon::format_exact
 0.2%   0.3%    3.5KiB  rayon_core rayon_core::registry::Registry::new
 0.2%   0.3%    3.5KiB  rayon_core rayon_core::registry::Registry::new
 0.2%   0.3%    3.5KiB     sysinfo sysinfo::windows::process::Process::new_full
 0.2%   0.3%    3.3KiB     sysinfo <sysinfo::windows::network::Networks as sysinfo::traits::NetworksExt>::refresh_networks_list
 0.2%   0.3%    3.2KiB         std rustc_demangle::demangle
 0.2%   0.2%    3.1KiB     sysinfo sysinfo::windows::disk::get_disks::{{closure}}
 0.2%   0.2%    3.0KiB         std <rustc_demangle::legacy::Demangle as core::fmt::Display>::fmt
 0.2%   0.2%    2.8KiB  rayon_core rayon_core::log::SimulatorState::dump<std::io::buffered::bufwriter::BufWriter<std::fs::File> >       
 0.2%   0.2%    2.8KiB  rayon_core rayon_core::log::SimulatorState::dump<std::io::buffered::bufwriter::BufWriter<std::io::stdio::Stde...
 0.2%   0.2%    2.7KiB   crc32fast crc32fast::baseline::update_fast_16
 0.2%   0.2%    2.7KiB     sysinfo sysinfo::windows::users::get_users::{{closure}}
 0.2%   0.2%    2.7KiB   crc32fast crc32fast::specialized::pclmulqdq::calculate
60.9%  79.7% 1010.3KiB             And 6386 smaller methods. Use -n N to show more.
76.3% 100.0%    1.2MiB             .text section size, the file size is 1.6MiB
```

So this tells us that `rand_chacha` is using large chunks of the binary. That's not a dependency we listed, so where did it come from?

Running `cargo tree` gives some clues:

```
collector_v3 v0.1.0 (C:\Users\Herbert\Documents\Ardan\5x1 Day Ultimate Rust\code\05_server\collector_v3)
├── shared_v3 v0.1.0 (C:\Users\Herbert\Documents\Ardan\5x1 Day Ultimate Rust\code\05_server\shared_v3)
│   ├── bincode v1.3.3
│   │   └── serde v1.0.164
│   │       └── serde_derive v1.0.164 (proc-macro)
│   │           ├── proc-macro2 v1.0.58
│   │           │   └── unicode-ident v1.0.8
│   │           ├── quote v1.0.27
│   │           │   └── proc-macro2 v1.0.58 (*)
│   │           └── syn v2.0.16
│   │               ├── proc-macro2 v1.0.58 (*)
│   │               ├── quote v1.0.27 (*)
│   │               └── unicode-ident v1.0.8
│   ├── crc32fast v1.3.2
│   │   └── cfg-if v1.0.0
│   └── serde v1.0.164 (*)
├── sysinfo v0.29.2
│   ├── cfg-if v1.0.0
│   ├── libc v0.2.144
│   ├── ntapi v0.4.1
│   │   └── winapi v0.3.9
│   ├── once_cell v1.18.0
│   ├── rayon v1.7.0
│   │   ├── either v1.8.1
│   │   └── rayon-core v1.11.0
│   │       ├── crossbeam-channel v0.5.8
│   │       │   ├── cfg-if v1.0.0
│   │       │   └── crossbeam-utils v0.8.15
│   │       │       └── cfg-if v1.0.0
│   │       ├── crossbeam-deque v0.8.3
│   │       │   ├── cfg-if v1.0.0
│   │       │   ├── crossbeam-epoch v0.9.14
│   │       │   │   ├── cfg-if v1.0.0
│   │       │   │   ├── crossbeam-utils v0.8.15 (*)
│   │       │   │   ├── memoffset v0.8.0
│   │       │   │   │   [build-dependencies]
│   │       │   │   │   └── autocfg v1.1.0
│   │       │   │   └── scopeguard v1.1.0
│   │       │   │   [build-dependencies]
│   │       │   │   └── autocfg v1.1.0
│   │       │   └── crossbeam-utils v0.8.15 (*)
│   │       ├── crossbeam-utils v0.8.15 (*)
│   │       └── num_cpus v1.15.0
│   └── winapi v0.3.9
├── thiserror v1.0.40
│   └── thiserror-impl v1.0.40 (proc-macro)
│       ├── proc-macro2 v1.0.58 (*)
│       ├── quote v1.0.27 (*)
│       └── syn v2.0.16 (*)
└── uuid v1.3.3
    ├── getrandom v0.2.9
    │   └── cfg-if v1.0.0
    └── rand v0.8.5
        ├── rand_chacha v0.3.1
        │   ├── ppv-lite86 v0.2.17
        │   └── rand_core v0.6.4
        │       └── getrandom v0.2.9 (*)
        └── rand_core v0.6.4 (*)
```

`uuid` appears to be the source of the random numbers---which makes sense, since it's randomly generating a UUID.

Looking at `Cargo.toml` for the project, we are using several features from `uuid`:

```toml
uuid = { version = "1.3.3", features = ["v4", "fast-rng"] }
```

Unfortunately, removing the `fast-rng` and `v4` features mean that we can no longer generate random UUIDs. If its acceptable to require that one be generated externally---say as part of the setup process---then we can remove the `uuid` dependency entirely.

> We're testing this in the `collector_nouuid` project.

Removing `uuid` yields a small improvement: 162,304 bytes (158 kb).

### Rayon?

**Rayon** also shows up as using a fair amount of size. What's that doing in there? The `sysinfo` crate is using Rayon! We can't remove that dependency, but we can remove the `rayon` feature from `sysinfo`:

```toml
sysinfo = { version = "0.29.2", default-features=false, features = ["apple-app-store"] }
```

Let's give that a go. That got us down to 137,216 bytes! (134 kb).

We're not going to get much smaller than that.

Let's make sure that the collector still works. It does!