mod app;
pub mod blocking_obs_client;

// #[derive(Default)]
pub struct ObsSwitcher {
    // pub scenes: obws::responses::scenes::Scenes,
    pub client: crate::blocking_obs_client::BlockingClient,
    pub data: SavedData,
}

pub struct SavedData {
    pub main_scene_name: String,
}

pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Result<T> = std::result::Result<T, Error>;
