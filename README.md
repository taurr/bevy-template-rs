# Bevy-template-rs

This is a small but feature rich set of [cargo-generate] templates for use when creating [Bevy] applications.

## Templates/Features

- **Binary template for the main game binary.**
  
  - Complete with camera setup, a very small 2D or 3D system and debug diagnostics.
  
  - Cargo runner configured to use [WASM Server Runner] in case of WASM target (`wasm32-unknown-unknown`)
  
  - Optionally includes setup of [vscode tasks].
  
  - Will utilize *fast compiles* as described in [The Bevy Book] when built using the vscode tasks. 
  
    If not using vscode, just use `cargo build --features bevy/dynamic`.
  
  - If enabling the feature `editor`, the project will attempt to use [bevy_editor_pls].

- **Library template for a small library crate.**
  
  - Includes example of integration testing.
  
  - Optionally includes setup of [vscode tasks].

- **Workflow template.**

  - Add a copy of the [Official Bevy Workflow template] to an existing project.
  
  - Template requires [cargo-generate] to be invoked with the `--init` parameter as it is *not inteded as a complete project*.

- Binary/Libray templates adjusts if expanded inside a existing cargo project.

## Usage

For further usage, or if having issues with `cargo-generate`, please have a look in the [cargo-generate book](https://cargo-generate.github.io/cargo-generate/index.html).

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
gh_username = "<YOUR GITHUB USERNAME>"
ide = "vscode|none"

[favorites.bevy]
git = "https://github.com/taurr/bevy-template-rs"

[favorites.bevy-bin]
git = "https://github.com/taurr/bevy-template-rs"
subfolder = Binary
```

After this, the template can be expanded using `cargo generate bevy`, or `cargo generate bevy-bin`.

[Bevy]:https://bevyengine.org
[cargo-generate]:https://github.com/cargo-generate/cargo-generate
[bevy_editor_pls]:https://github.com/jakobhellermann/bevy_editor_pls
[The Bevy Book]:https://bevyengine.org/learn/book
[Official Bevy Workflow template]:https://github.com/bevyengine/bevy_github_ci_template
[WASM Server Runner]: https://github.com/jakobhellermann/wasm-server-runner
[vscode]: https://code.visualstudio.com
[vscode tasks]: https://code.visualstudio.com/Docs/editor/tasks
