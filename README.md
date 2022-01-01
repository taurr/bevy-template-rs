# bevy-template-rs

This is a template for bootstraping binary or library projects using [Bevy].

By default, the generated template is configured to as fast as possible 
(see [Setup](https://bevyengine.org/learn/book/getting-started/setup/) in [The Bevy Book]).
This requires the Rust nightly compiler and a few others - to disable, just edit the generated 
files: `rust-toolchain.toml` and `.cargo/config.toml`.

## Usage

```shell
cargo generate taurr/bevy-template-rs
```

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