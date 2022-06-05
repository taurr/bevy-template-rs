# Bevy-template-rs

Here are a few `cargo-generate` templates for use when creating [bevy] applications.

## Templates

### Game

This is a template for starting a new game.

- Binary
- Latest (0.7) [Bevy] version, or GitHub main version (as a patch), are both supported.
- The template will utilize *"fast compiles"* as described in [The Bevy Book] when building using
  the vscode tasks.
- Basic GitHub workflow for building/testing the resulting project
- VSCode launch configuration for executable/unit-tests and integration test
- VSCode tasks

### crate-plugin

A simple template for creating a plugin with [Bevy].

- Library
- Latest (0.7) [Bevy] version, or GitHub main version (as a patch), are both supported.
- The template will utilize *"fast compiles"* as described in [The Bevy Book] when building using
  the vscode tasks.
- Code example for doing integration tests
- Basic GitHub workflow for building/testing the resulting project
- VSCode launch configuration for debugging unit and integration tests
- VSCode tasks

## Usage

```shell
cargo generate taurr/bevy-template-rs
```

![Template expansion](./assets/template-expansion.gif)

## Requirements

[cargo-generate] must be installed. The easiest way to do this is:

```shell
cargo install cargo-generate
```

## Tips'n'tricks

If the template is used on a regular basis, [cargo-generate] allows to setup favorite templates and default variables.

To do this, open or create the file `$CARGO_HOME/cargo-generate.toml`, insert this:
```toml
[favorites.bevy]
path = "taurr/bevy-template-rs"
```

After this, the template can be expanded using a simple:

```shell
cargo generate bevy
```

[Bevy]:https://bevyengine.org
[cargo-generate]:https://github.com/cargo-generate/cargo-generate
[The Bevy Book]:https://bevyengine.org/learn/book