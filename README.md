# bevy-template-rs

This is a template for bootstraping binary or library projects using [Bevy].

- Latest (0.5) [Bevy] version, or GitHub main version, are both supported.
- The template can optionally setup the project for using "fast compiles" as described in [The Bevy Book].
- Basic GitHub workflow for building/testing the resulting project
- Contains `assets` folder for your game assets
- Code example for doing integration tests
- VSCode launch configuration for executable/unit-tests and integration test

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
[cargo-generate]:https://github.com/cargo-generate/cargo-generate
[The Bevy Book]:https://bevyengine.org/learn/book