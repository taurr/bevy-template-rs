# Bevy-template-rs

This is a small but feature rich `cargo-generate` template for use when creating [bevy] applications.

## Features

- Binary or Library. Select which by passing `--bin` or `--lib` to `cargo-generate`. 
- Binary optionally sets up a 2D or 3D camera.
- (Currently disabled) Binary expansion setup to use [bevy_editor_pls](https://github.com/jakobhellermann/bevy_editor_pls).
Just pass `--features editor` when building (or use the right task in vscode).
- Template adjusts if expanded inside a existing cargo project.
- Library expansion includes complete integration test example.
- The template will utilize *"fast compiles"* as described in [The Bevy Book] when building using
  the vscode tasks. If not using vscode, just use `cargo build --features bevy/dynamic`.
- Basic GitHub workflow for building/testing the resulting project (optional)
- Latest (0.8) [Bevy] version, or GitHub main version (as a patch), are both supported.
- VSCode launch configuration for executable/unit-tests and integration test
- VSCode tasks

## Usage

```shell
cargo generate taurr/bevy-template-rs [--lib]
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