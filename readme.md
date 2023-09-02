# Cargo-Pack

Cargo-Pack is a executable packer for Rust crates, designed to streamline the packaging process for any Rust crate that has a binary target. With Cargo-Pack, you can easily bundle your Rust applications, making it a valuable tool for securing and distributing your software.

## Features

Cargo-Pack offers the following features to enhance your packaging workflow:

- [x] **Binary Encryption/Obfuscation**: Cargo-Pack utilizes the robust `ChaCha20` encryption algorithm to protect your binary.

- [x] **Compression**: Your application's binary is compressed using the efficient `Brotli` stream compression algorithm, reducing its size and optimizing deployment.

- [x] **Windows PE Support**: Cargo-Pack fully supports Windows PE format, ensuring compatibility with Windows environments.

## Planned Improvements

We are actively working on extending Cargo-Pack's capabilities to provide even more value. Our upcoming improvements include:

- [ ] **ELF Linux Support**: We aim to add support for ELF Linux binaries, expanding the range of platforms where you can use Cargo-Pack.

- [ ] **Mac Binary Support**: Enhancements are in progress to support packaging Mac binaries, making Cargo-Pack more versatile for cross-platform applications.

## Installation

To use Cargo-Pack, you'll need to follow these installation steps:

1. Clone the Cargo-Pack repository to your local machine by running the following command:

   ```
   git clone https://github.com/michaelvanstraten/cargo-pack
   ```

2. After cloning, navigate to the repository directory.

3. Use `cargo install` to install Cargo-Pack:

   ```
   cargo install --path .
   ```

Make sure that the Cargo-Pack repository remains in the same location for successful installation.

## Usage

Once you've installed Cargo-Pack, you can use it in your Rust projects effortlessly. Here's a sample command to get you started:

```bash
cargo pack --target x86_64-uwp-windows-gnu
```

You can also use Cargo-Pack through the regular Cargo build command interface, making it seamless to integrate into your existing workflow.

Cargo-Pack simplifies the process of packaging Rust applications, providing essential security and optimization features, with plans to support a broader range of platforms in the future.
