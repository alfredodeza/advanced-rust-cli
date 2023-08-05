# Packaging your Rust CLI tool

There are a few things that are essential when packaging your Rust tool for distribution. This guide will walk you through the process of creating a distributable binary as well as creating a container image for your tool.

## Creating a distributable binary on Linux

The first step is to create a distributable binary. This is a binary that can be run on any Linux system without any dependencies. This is done by compiling your Rust code with the `musl` target. This target is a Linux target that is statically linked. This means that all the dependencies are included in the binary itself.

To compile your Rust code with the `musl` target, you need to install the `musl` target for Rust. You can do this by running the following command in a Debian based Linux distribution:

```bash
sudo apt-get install -y musl-tools
```

Once you have the `musl` target installed, you can compile your Rust code with the `musl` target by running the following command:

```bash
cargo build --release --target x86_64-unknown-linux-musl
```

This will create a binary in the `target/x86_64-unknown-linux-musl/release` directory. This binary can be run on any Linux system without any dependencies.

## Creating a distributable binary on OSX

You can create a binary for OSX by running the following command:

```bash
cargo build --release
```

This will create a binary in the `target/release` directory. This binary can be run on any OSX system without any dependencies.

## Creating a container image

To create a container image you must create a `Dockerfile`. Make sure that the right version of Rust is used as a base image. You can find the right version of Rust on the [Rust website](https://www.rust-lang.org/tools/install). You can also use the `rustup` tool to install the right version of Rust.

Once you ensure the right version of Rust, make sure that your container can run from the terminal using the `CMD` instruction. This will allow you to run your container as if it was a normal binary.
