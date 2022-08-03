if !variable::get("is_init") {
    abort("\
This template is designed to be added to a existing project.\n\
Please make sure you are in the root of the project to add the template files to,\n\
and re-run cargo-generate with the --init option.\n\
\n\
Example:\n\
cargo generate --git taurr/bevy-template-rs Workflow --init");
}
