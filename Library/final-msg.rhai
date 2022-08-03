if variable::get("within_cargo_project") {
    print();
    print("Remember to add this crate to the workspace in the parent Cargo.toml:");
    print("[workspace]");
    print("    members = [\"" + variable::get("project-name")+ "\"]");

    print();
}
