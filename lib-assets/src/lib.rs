pub use bevy_embasset::EnumCount;

bevy_embasset::assets!(
    pub enum GameAssets {
        #[doc = "Example doc"]
        Icon = ".keepme"
    },
    pub struct GameAssetsIo {
        root = "../assets/"
    }
);
