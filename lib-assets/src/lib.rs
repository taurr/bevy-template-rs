use bevy_embasset::embasset_assets;

embasset_assets!(
    pub enum GameAssets {
        #[doc = "Example doc"]
        Icon = ".keepme"
    },
    pub struct GameAssetsIo {
        root = "../assets/"
    }
);
