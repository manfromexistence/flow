# Contributing to dx

Thank you for your interest in contributing to dx! We welcome contributions in the form of bug reports, feature requests, documentation improvements, and code changes.

This guide will help you understand how to contribute to the project.

## Table of Contents

1. [Getting Started](#getting-started)
2. [Project Structure](#project-structure)
3. [Development Setup](#development-setup)
4. [How to Contribute](#how-to-contribute)
5. [Pull Requests](#pull-requests)

## Getting Started

### Prerequisites

Before you begin, ensure you have met the following requirements:

- Rust installed on your machine. You can download it from [rustup.rs](https://rustup.rs).
- Familiarity with Git and GitHub.

### Fork the Repository

1. Fork the [dx repository](https://github.com/sxdx/dx) to your GitHub account.
2. Clone your fork to your local machine:

   ```sh
   git clone https://github.com/<your-username>/dx.git
   ```

3. Set up the upstream remote:
   ```sh
   git remote add upstream https://github.com/sxdx/dx.git
   ```

## Project Structure

A brief overview of the project's structure:

```sh
.
├── assets/             # Assets like images and fonts
├── nix/                # Nix-related configurations
├── scripts/            # Helper scripts used by CI/CD
├── snap/               # Snapcraft configuration
├── dx-adapter/       # dx image adapter
├── dx-binding/       # dx Lua bindings
├── dx-boot/          # dx bootstrapper
├── dx-cli/           # dx command-line interface
├── dx-codegen/       # dx code generator
├── dx-config/        # dx configuration file parser
├── dx-core/          # dx core logic
├── dx-dds/           # dx data distribution service
├── dx-ffi/           # dx foreign function interface
├── dx-fm/            # dx file manager
├── dx-fs/            # dx file system
├── dx-macro/         # dx macros
├── dx-plugin/        # dx plugin system
├── dx-proxy/         # dx event proxy
├── dx-scheduler/     # dx task scheduler
├── dx-shared/        # dx shared library
├── dx-term/          # dx terminal extensions
├── dx-widgets/       # dx user interface widgets
├── .github/            # GitHub-specific files and workflows
├── Cargo.toml          # Rust workflow configuration
└── README.md           # Project overview
```

## Development Setup

1. Ensure the latest stable Rust is installed:

   ```sh
   rustc --version
   cargo --version
   ```

2. Build the project:

   ```sh
   cargo build
   ```

3. Run the tests:

   ```sh
   cargo test --workspace --verbose
   ```

4. Format the code (requires `rustfmt` nightly):

   ```sh
   rustup component add rustfmt --toolchain nightly
   rustfmt +nightly **/*.rs
   ```

## How to Contribute

### Reporting Bugs

If you encounter a bug and have found a way to reliably reproduce it on the latest `main` branch, please file a [bug report](https://github.com/sxdx/dx/issues/new?template=bug.yml) with a [minimal reproducer](https://stackoverflow.com/help/minimal-reproducible-example).

### Suggesting Features

If you want to request a feature, please file a [feature request](https://github.com/sxdx/dx/issues/new?template=feature.yml). Please make sure to search for existing issues and discussions before submitting.

### Improving Documentation

dx's documentation placed at [dx-rs/dx-rs.github.io](https://github.com/dx-rs/dx-rs.github.io), contributions related to documentation need to be made there.

### Improving Icons

dx's icon originates from [`nvim-web-devicons`](https://github.com/nvim-tree/nvim-web-devicons), and it is periodically grabbed and updated with the latest changes from upstream via [`generate.lua`](https://github.com/sxdx/dx/blob/main/scripts/icons/generate.lua).

Contributions related to the icon should be made upstream to facilitate easier automation of this process.

### Submitting Code Changes

1. Create a new branch for your changes:

   ```sh
   git checkout -b your-branch-name
   ```

2. Make your changes. Ensure that your code follows the project's [coding style](https://github.com/sxdx/dx/blob/main/rustfmt.toml) and passes all tests.
3. Commit your changes with a descriptive commit message:

   ```sh
   git commit -m "feat: an awesome feature"
   ```

4. Push your changes to your fork:

   ```sh
   git push origin your-branch-name
   ```

## Pull Requests

If you have an idea, before raising a pull request, we encourage you to file an issue to propose it, ensuring that we are aligned and reducing the risk of re-work.

We want you to succeed, and it can be discouraging to find that a lot of re-work is needed.

### Process

1. Ensure your fork is up-to-date with the upstream repository:

   ```sh
   git fetch upstream
   git checkout main
   git merge upstream/main
   ```

2. Rebase your feature branch onto the `main` branch:

   ```sh
   git checkout your-branch-name
   git rebase main
   ```

3. Create a pull request to the `main` branch of the upstream repository. Follow the pull request template and ensure that:
   - Your code passes all tests and lints.
   - Your pull request description clearly explains the changes and why they are needed.
4. Address any review comments. Make sure to push updates to the same branch on your fork.
