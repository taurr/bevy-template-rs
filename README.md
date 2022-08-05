# Bevy-template-rs

This is a small but feature rich set of [cargo-generate] templates for use when creating [Bevy] applications.

## Templates/Features

- **Binary template for the main game binary.**
  
  - Complete with camera setup, a very small 2D or 3D system and debug diagnostics.
  
  - Optionally includes setup of [vscode tasks].
  
  - Cargo runner configured to use [WASM Server Runner] in case of WASM target (`wasm32-unknown-unknown`)
    - Starting the WASM development server using the optional [vscode tasks] uses [cargo-watch].
  
  - Will utilize *fast compiles* as described in [The Bevy Book] when built using the [vscode tasks] (*not for WASM target*). 
  
    If not using [vscode], just use `cargo build --features bevy/dynamic`.
  
  - If enabling the feature `editor`, the project will attempt to use [bevy_editor_pls].

- **Library template for a small library crate.**
  
  - Includes example of integration testing.
  
  - Optionally includes setup of [vscode tasks].

- **Workflow template.**

  - Add a copy of the [Official Bevy Workflow template] to an existing project.
  
  - Template requires [cargo-generate] to be invoked with the `--init` parameter as it is *not inteded as a complete project*.

- Binary/Libray templates adjusts if expanded inside a existing cargo project.

## Usage

To expand e.g a binary project, use:
```shell
cargo generate taurr/bevy-template-rs Binary
```

To include Github workflow files, use:
```shell
cd <PROJECT_DIR>
cargo generate taurr/bevy-template-rs Workflow --init --name <BINARY NAME>
```

Why the `--init` parameter? Well thats just to tell [cargo-generate] *not* to create a folder for the generated files.

For further usage, or if having issues with `cargo-generate`, please have a look in the [cargo-generate book](https://cargo-generate.github.io/cargo-generate/index.html).


![Template expansion](./assets/template-expansion.gif)

## Requirements

[cargo-generate] must be installed. The easiest way to do this is:

```shell
cargo install cargo-generate
```

## :bulb: Tips'n'tricks

If the template is used on a regular basis, [cargo-generate] allows to setup favorite templates and default variables.

To do this, open or create the file `$CARGO_HOME/cargo-generate.toml`, insert this:
```toml
[values]
gh_username = "<YOUR GITHUB USERNAME>"
ide = "vscode|none"

[favorites.bevy-bin]
git = "https://github.com/taurr/bevy-template-rs"
subfolder = "Binary"

[favorites.bevy-lib]
git = "https://github.com/taurr/bevy-template-rs"
subfolder = "Library"

[favorites.bevy-wf]
git = "https://github.com/taurr/bevy-template-rs"
subfolder = "Workflow"
is_init = true
```

After this, the template can be expanded using `cargo generate bevy-bin`, `cargo generate bevy-lib` or `cargo generate bevy-wf`.

:warning: Adding the workflow template through this favorite will no longer require the `--init` parameter, and thus will always expand in the current directory!

[Bevy]:https://bevyengine.org
[cargo-generate]:https://github.com/cargo-generate/cargo-generate
[bevy_editor_pls]:https://github.com/jakobhellermann/bevy_editor_pls
[The Bevy Book]:https://bevyengine.org/learn/book
[Official Bevy Workflow template]:https://github.com/bevyengine/bevy_github_ci_template
[WASM Server Runner]: https://github.com/jakobhellermann/wasm-server-runner
[vscode]: https://code.visualstudio.com
[vscode tasks]: https://code.visualstudio.com/Docs/editor/tasks
[cargo-watch]: https://github.com/watchexec/cargo-watch
