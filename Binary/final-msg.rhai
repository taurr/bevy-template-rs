if variable::get("camera") != "3D" {
    file::delete("assets/cube.glb");
}
if variable::get("camera") != "2D" {
    file::delete("assets/icon.png");
}
file::delete("src/{{crate_name}}.rs");

if !variable::get("within_cargo_project") {
    print();
    print("WASM support is enabled in this project.");
    print("To use it, make sure to run: rustup target install wasm32-unknown-unknown");
    print("and: cargo install -f wasm-server-runner");
    print();
    print("You can then launch a small development server using: cargo watch -- cargo run --target wasm32-unknown-unknown");
    if variable::get("ide") == "vscode" {
        print("A vscode build task for launching the development server has been added.");
    }
} else {
    print();
    print("Remember to add this crate to the workspace in the parent Cargo.toml:");
    print("[workspace]");
    print("    members = [\"" + variable::get("project-name")+ "\"]");
}
print();
