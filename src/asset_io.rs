use bevy::{
    asset::AssetIo,
    prelude::{App, AssetServer, Plugin},
    tasks::IoTaskPool,
};
use std::path::{Path, PathBuf};

// TODO: move plugin into it's own crate
// TODO: move most of `build.rs` t the same crate
// TODO: write tests and docs
// TODO: release on crates.io

// TODO: allow for fall through to normal file system
// TODO: allow some kind of protocol plugins for e.g. loading from web services

#[derive(Default)]
pub struct InMemoryAssetPlugin;

impl Plugin for InMemoryAssetPlugin {
    fn build(&self, app: &mut App) {
        let task_pool = app
            .world
            .get_resource::<IoTaskPool>()
            .expect("`IoTaskPool` resource not found.")
            .0
            .clone();

        app.insert_resource(AssetServer::new(InMemoryAssetIo::preloaded(), task_pool));
    }
}

include!(concat!(env!("OUT_DIR"), "/include_all_assets.rs"));

pub struct InMemoryAssetIo {
    loaded: std::collections::HashMap<&'static Path, &'static [u8]>,
}

impl Default for InMemoryAssetIo {
    fn default() -> Self {
        InMemoryAssetIo::new()
    }
}

impl InMemoryAssetIo {
    #[must_use]
    pub fn new() -> Self {
        InMemoryAssetIo {
            loaded: std::collections::HashMap::new(),
        }
    }

    #[must_use]
    pub fn preloaded() -> Self {
        let mut new = InMemoryAssetIo {
            loaded: std::collections::HashMap::new(),
        };
        include_all_assets(&mut new);
        new
    }

    #[allow(dead_code)]
    pub fn add_entity(&mut self, path: &'static Path, data: &'static [u8]) {
        self.loaded.insert(path, data);
    }
}

impl AssetIo for InMemoryAssetIo {
    fn load_path<'a>(
        &'a self,
        path: &'a Path,
    ) -> bevy::utils::BoxedFuture<'a, Result<Vec<u8>, bevy::asset::AssetIoError>> {
        Box::pin(async move {
            self.loaded
                .get(path)
                .map(|b| b.to_vec())
                .ok_or_else(|| bevy::asset::AssetIoError::NotFound(path.to_path_buf()))
        })
    }

    #[allow(clippy::needless_collect)]
    fn read_directory(
        &self,
        path: &Path,
    ) -> Result<Box<dyn Iterator<Item = PathBuf>>, bevy::asset::AssetIoError> {
        let paths: Vec<_> = self
            .loaded
            .keys()
            .filter(|loaded_path| loaded_path.starts_with(path))
            .map(|t| t.to_path_buf())
            .collect();
        Ok(Box::new(paths.into_iter()))
    }

    fn is_directory(&self, path: &Path) -> bool {
        let as_folder = path.join("");
        self.loaded
            .keys()
            .any(|loaded_path| loaded_path.starts_with(&as_folder) && loaded_path != &path)
    }

    fn watch_path_for_changes(&self, _path: &Path) -> Result<(), bevy::asset::AssetIoError> {
        Ok(())
    }

    fn watch_for_changes(&self) -> Result<(), bevy::asset::AssetIoError> {
        Ok(())
    }
}
