## Releasing
Releasing a package (either a library or a CLI) is usually done with [crates.io](https://crates.io) as the destination. Rust has all the tooling in place via `cargo` to help out with this task.

The tooling for publishing has several advantages from other programming languages, but mainly, the simplicity and focus helps accomplish publishing without much problems.

For example, this is a _"dry run"_ (does not actually publish anything) way to try out with `cargo`:

```
$ cargo publish --dry-run
    Updating crates.io index
warning: manifest has no description, license, license-file, documentation, homepage or repository.
See https://doc.rust-lang.org/cargo/reference/manifest.html#package-metadata for more info.
error: 5 files in the working directory contain changes that were not yet committed into git:

Cargo.toml
src/lib.rs
src/main.rs
.vscode/settings.json
src/util.rs

to proceed despite this and include the uncommitted changes, pass the `--allow-dirty` flag
```

It didn't let me publish without some useful files like the license and the documentation. The amount of extra information, alternatives, and suggestions makes this process straightforward.
