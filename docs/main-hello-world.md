# Hello world example

## Generate a project

CrossBundle uses [`cargo-generate`](https://github.com/cargo-generate/cargo-generate) to generate a new project. This means that you need to install it before we proceed.

```sh
cargo install cargo-generate
```

Then you can create a new project:

```sh
crossbundle new project-name
# crossbundle new project-name --template bevy
# crossbundle new project-name --template quad
```

All supported templates you can watch [`here`](https://github.com/dodorare/crossbundle-templates) (each branch = template).

## Project overview

The project has been created. Now let's see what the project consists of.

```toml
# Cargo.toml

[package]
name = "project-name"
version = "0.1.0"
authors = ["Example <example@example.com>"]
edition = "2021"

[dependencies]
crossbundle = "*"

[package.metadata]
icon = "ic_launcher"
android_res = "res/android"
apple_res = "res/apple"
```

```rust
// main.rs

fn main() {
    println!("Hello, project-name!");
}
```

## Build an application

Let's build and run our first CrossBundle application.

```sh
# cd project-name
crossbundle run android
# or (if your project uses macroquad)
crossbundle run android --quad
# or
crossbundle run apple
```

If you want to build the application for android as AAB - add `--aab` flag.

When the application deploys on your device, you can attach a logger.

```sh
crossbundle log android
```

and you will see the message: `"Hello, project-name!"`
