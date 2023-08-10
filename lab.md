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
