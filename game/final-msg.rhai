if variable::get("crate_type") != "lib" && variable::get("camera") != "3D" {
    file::delete("assets/cube.glb");
}