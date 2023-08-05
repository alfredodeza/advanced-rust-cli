# Containerizing your Rust CLI tool

Another useful way to package your CLI tool is using a container. To accomplish this you'll need to use a `Dockerfile`. For command-line tools, it is useful to package the project so that it can be used with Docker while accepting arguments. This can be done by using the `ENTRYPOINT` command in the `Dockerfile` and passing the arguments to the container.

```dockerfile
ENTRYPOINT ["cargo", "run", "--"]
```

If the container has already built the release-ready tool, you can use the actual executable instead of `cargo run`. For example, if the executable is called `mytool`, the `ENTRYPOINT` command would look like this:

```dockerfile
# Set the entrypoint to the binary
ENTRYPOINT ["./target/release/mytool"]
```

Additionally, it is useful to have a default command that is run when the container is run without any arguments. This can be done by using the `CMD` command in the `Dockerfile`. For the example in this repository, the `Dockerfile` uses the `--help` flag as the default command:

```dockerfile
CMD ["--help"]
```

