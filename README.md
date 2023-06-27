# Permer

Permer is a CLI tool written in Rust that provides information about files present in your file system.

## Installation

- You can download the source code and build it from there

```bash
cargo build --release
```

Running this command will create a binary under the `target/release` directory that you can execute:

```bash
./target/release/Permer <File>
```

- Or you can check the release section and download the pre-compiled binary from there§.
  > Check for the latest release to assure that all the functionalities are available.

## TODO

the following section consists of the features and options that I am willing to implement in the future.

- [x] Provide a Dockerfile that makes building and running this tool as a container possible.
- [ ] Distribute this tool as an AUR package available to download.
- [ ] Added color-support and better formatting the output.
