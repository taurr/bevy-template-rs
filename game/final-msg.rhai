if variable::get("crate_type") != "lib" && variable::get("camera") != "3D" {
    file::delete("assets/cube.glb");
}

if variable::get("github_setup") == false {
    file::delete(".github");
    file::delete("CHANGELOG.md");
    file::delete("CODE_OF_CONDUCT.md");
    file::delete("CONTRIBUTING.md");
    file::delete("LICENSE-APACHE");
    file::delete("LICENSE-MIT");
}

if variable::get("within_cargo_project") {
    print();
    print("Remember to add this crate to the workspace in the parent Cargo.toml:");
    print("[workspace]");
    print("    members = [\"" + variable::get("project-name")+ "\"]");

    print();
}
