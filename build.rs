use std::{
    env,
    fs::{self, File},
    io::Write,
    path::{Path, PathBuf},
};

fn asset_folder() -> PathBuf {
    Path::new(&env::var("CARGO_MANIFEST_DIR").unwrap()).join("assets")
}

fn main() {
    let out_dir = env::var_os("OUT_DIR").unwrap();
    generate_include_all_assets(out_dir);
}

fn generate_include_all_assets(out_dir: std::ffi::OsString) {
    let dest_path = Path::new(&out_dir).join("include_all_assets.rs");
    let mut file = File::create(&dest_path).unwrap();
    file.write_all(
        "pub fn include_all_assets(#[allow(unused)] in_memory: &mut crate::asset_io::InMemoryAssetIo){\n".as_ref(),
    )
    .unwrap();
    let dir = asset_folder();
    visit_dirs(&dir)
        .iter()
        .map(|path| (path, path.strip_prefix(&dir).unwrap()))
        .for_each(|(fullpath, path)| {
            file.write_all(
                format!(
                    r#"in_memory.add_entity(std::path::Path::new({:?}), include_bytes!({:?}));
"#,
                    path.to_string_lossy(),
                    fullpath.to_string_lossy()
                )
                .as_ref(),
            )
            .unwrap();
        });
    file.write_all("}".as_ref()).unwrap();
    cargo_emit::rerun_if_changed!(dir.display());
}

fn visit_dirs(dir: &Path) -> Vec<PathBuf> {
    let mut collected = vec![];
    if dir.is_dir() {
        for entry in fs::read_dir(dir).unwrap() {
            let entry = entry.unwrap();
            let path = entry.path();
            if path.is_dir() {
                collected.append(&mut visit_dirs(&path));
            } else {
                collected.push(path);
            }
        }
    }
    collected
}
