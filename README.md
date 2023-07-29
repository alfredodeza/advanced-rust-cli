# Rust CLI Example

A small Rust CLI example you can use to build on. With an emphasis on Linux and creating automation tools that solve a problem for you. This is the basis for DevOps principles that you can apply in day-to-day work.

## Setting up your environment
Rust development requires certain tools to be installed on your system. The easiest way to do this is to use the [rustup](https://rustup.rs/) tool. This will install the Rust compiler and Cargo, the Rust package manager. Although you can install it in Linux using the package manager, I recommend using `rustup`. Use the following command or go through the [rustup.rs](https://rustup.rs/) website to install it.

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

This repository and video course focuses on the development side of command-line tools in Rust. It uses [Visual Studio Code](https://code.visualstudio.com/?WT.mc_id=academic-0000-alfredodeza) as the editor of choice. You can use any editor you like, but the instructions in this repository will be for VS Code.

These are all the tools and editor extensions I recommend you install to get started:

- [Visual Studio Code](https://code.visualstudio.com/?WT.mc_id=academic-0000-alfredodeza)
- [Rust and Cargo tools](https://rustup.rs/)
- [Rust Analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer&WT.mc_id=academic-0000-alfredodeza)
- [GitHub Copilot](https://marketplace.visualstudio.com/items?itemName=GitHub.copilot&WT.mc_id=academic-0000-alfredodeza)

As part of your development workflow, I highly suggest you use the following programs in the terminal regularly:

- `cargo fmt` - Formats your code to the Rust standard
- `cargo clippy` - Lints your code and helps you find errors and potential issues
- `cargo check` - Checks your code for errors and allows you to fix them before compiling (which means its faster!)


## Resources
Explore additional content that you can use to learn more about the topics covered in this course.

- [Releasing](./resources/releasing.md)

**Coursera Courses**

- [Linux and Bash for Data Engineering](https://www.coursera.org/learn/linux-and-bash-for-data-engineering-duke)
- [Open Source Platforms for MLOps](https://www.coursera.org/learn/open-source-platforms-duke)
- [Python Essentials for MLOps](https://www.coursera.org/learn/python-essentials-mlops-duke)
- [Web Applications and Command-Line tools for Data Engineering](https://www.coursera.org/learn/web-app-command-line-tools-for-data-engineering-duke)
- [Python and Pandas for Data Engineering](https://www.coursera.org/learn/python-and-pandas-for-data-engineering-duke)
- [Scripting with Python and SQL for Data Engineering](https://www.coursera.org/learn/scripting-with-python-sql-for-data-engineering-duke)

**O'Reilly Courses and Books**

- [Python for DevOps](https://www.oreilly.com/library/view/python-for-devops/9781492057680/) (Book)
- [Practical MLOps](https://www.oreilly.com/library/view/practical-mlops/9781098103002/) (Book)
- [Linux For Beginners](https://learning.oreilly.com/videos/-/27922450VIDEOPAIML/) (Video)
- [GitHub Codespaces Course](https://learning.oreilly.com/videos/-/27724023VIDEOPAIML/) (Video)
- [Python Command-line Tools course](https://learning.oreilly.com/videos/python-command-line/50131VIDEOPAIML/) (Video)
