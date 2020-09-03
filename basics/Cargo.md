# Cargo

Cargo is the package manager and build system. It is used to manage projects because cargo handles a lot of tasks - building, downloading libraries/dependencies.

## Creating a project with cargo

To create a new project with cargo run the following command.\

```bash
$ cargo new <project name>
```

this command would create a directory with your project name and also create a Cargo.toml and src/ directory containing main.rs file.

### Commands

- Build command - `$ cargo build`.
- Run command (To run your project) - `$ cargo run`.
- Build the project without creating an executable `$ cargo check`.
- Build release of your project `$ cargo build --release`.
