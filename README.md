# Bevy-template-rs

Here are a few `cargo-generate` templates for use when creating [bevy] applications.

## Templates

### Game

This is a template for starting a new game.

- Binary
- Latest (0.6) [Bevy] version, or GitHub main version (as a patch), are both supported.
- The template can optionally setup the project for using "fast compiles" as described in [The Bevy Book].
- Basic GitHub workflow for building/testing the resulting project
- Code example for doing integration tests
- VSCode launch configuration for executable/unit-tests and integration test
- VSCode tasks

### lib-plugin

A simple template for creating a plugin with [Bevy].

- Library
- Code example for doing integration tests
- VSCode launch configuration for debugging unit and integration tests
- VSCode tasks

### lib-assets

A library for hosting embedded assets by utilizing the [Bevy-Embasset] crate.

- Library
- Assets folder for assets that should be embedded
- VSCode launch configuration for debugging unit-tests
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
[values]
github_username = "<your github username>" # used in bevy template files where appropriate

[favorites.bevy]
path = "taurr/bevy-template-rs"
```

After this, the template can be expanded using a simple:

```shell
cargo generate bevy
```

[Bevy]:https://bevyengine.org
[Bevy-Embasset]:https://github.com/taurr/bevy-embasset
[cargo-generate]:https://github.com/cargo-generate/cargo-generate
[The Bevy Book]:https://bevyengine.org/learn/book