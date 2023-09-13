# Practice Lab: Build a Timestamp Command Line tool
In this practice lab, you will build a command line tool that will generate a timestamp based on the current time. The tool will accept a single argument, which is the number of seconds to add to the current time. If no argument is provided, the tool will default to adding 0 seconds to the current time.

**Learning objectives:**

- Use a command line framework like Clap to handle arguments and options
- Practice error-handling techniques to provide useful messages to the user
- Use logging to make it easier to debug when errors happen
- Package and develop your tool using Cargo

**Steps:**

1. Create a new repository in your acconut for your Rust project. Use the [Rust template repository](https://github.com/alfredodeza/rust-template) to quickly generate one for your own account. Use this link to [create it in one step](https://github.com/alfredodeza/rust-template/generate).
1. Use Clap with the `derive` feature. Refer to the [derive example](./examples/2-complex) as a guide.
1. Add a single argument to your tool. The argument should be a number of seconds to add to the current time. If no argument is provided, the tool should default to adding 0 seconds to the current time.

**Bonus challenge:** Add an option to your tool that will allow the user to specify the format of the timestamp. For example, the user could specify `--format=rfc3339` to get the timestamp in RFC 3339 format. If no format is specified, the tool should default to using the format `YYYY-MM-DD HH:MM:SS`.

**Concepts Covered:**

- [x] Building a command line tool using Rust and the Clap framework
- [x] Using the `derive` feature of Clap to generate a command line tool
- [x] Adding arguments to a command line tool
- [x] Error handling and logging
- [x] Packaging and developing a Rust tool using Cargo

By completing this lab, you will gain hands-on experience in Rust and CLI tools that can help you automate tasks and build useful tools for your own projects. This should allow you to be proficient in Data Engineering, DevOps, Systems Engineering, or Data Scientist roles that require you to build tools to automate tasks.

# Bonus Practice Lab: Package and distribute your tool

In this practice lab, you will take your existing command line tool that generates timestamps and focus on packaging and distributing it as a usable Rust CLI tool. You will use Cargo, Rust's package manager, to prepare your tool for distribution, making it easy for others to install and use.

**Learning Objectives:**

- Package your Rust CLI tool using Cargo for distribution.
- Make your tool available for others to install and use.
- Understand the process of creating a distributable Rust CLI tool.
- Explore the practical aspects of packaging and distributing software.

**Steps:**

Ensure your existing timestamp command line tool is working correctly and that it has been implemented following the previous lab instructions.

1. Create or generate a `Cargo.toml` file in the root of your project directory. This file will contain metadata about your project and its dependencies. Refer to the official Cargo documentation for guidance on creating this file.
1. Define your project's metadata in the Cargo.toml file. This metadata should include the project's name, version, author, and a brief description of the tool. For example:
    ```toml
    [package]
    name = "timestamp-tool"
    version = "0.1.0"
    authors = ["Your Name <your@email.com>"]
    description = "A Rust CLI tool to generate timestamps."
    ```
1. Add any necessary dependencies to your Cargo.toml file. For instance, if your tool uses external crates, specify them in the `[dependencies]` section. Ensure that you have correctly declared Clap as a dependency since you are using it for command-line argument parsing.
1. Build your Rust CLI tool using Cargo. Run the following command in your project's root directory:
    ```bash
    cargo build --release
    ```
1. This will create a compiled binary in the target/release/ directory. The --release flag optimizes the build for release.
1. Consider creating installation instructions or documentation for your tool, describing how users can install and use it.
1. If you want to distribute your tool widely, consider publishing it on platforms like crates.io or creating a standalone executable. Publishing on crates.io makes it easy for Rust developers to discover and use your tool, while creating a standalone executable allows users to run your tool without needing to install Rust or Cargo.


Concepts Covered:

- Packaging and distributing a Rust CLI tool using Cargo.
- Defining project metadata in a Cargo.toml file.
- Building a release-ready binary with Cargo.
- Creating distribution packages for your tool.
- Considerations for publishing and distributing Rust CLI tools.
  
By completing this lab, you will have taken your Rust CLI tool to the next level by packaging and distributing it, making it accessible to a wider audience. This experience will help you understand the practical aspects of sharing software and prepare you for real-world scenarios where you need to package and distribute your Rust tools.
