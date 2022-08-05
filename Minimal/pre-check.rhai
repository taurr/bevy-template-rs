if variable::get("crate_type") != "bin" {
    abort("This is a template for an executable, and is not compatible with the --lib option");
}
