use tokio::runtime::Runtime;

// pub use crate::client::Message;

/// Established connection with a Redis server.
pub struct BlockingClient {
    /// The asynchronous `Client`.
    inner: obws::Client,

    /// A `current_thread` runtime for executing operations on the
    /// asynchronous client in a blocking manner.
    rt: Runtime,
}

pub fn connect(
    host: impl AsRef<str>,
    port: u16,
    password: Option<impl AsRef<str>>,
) -> crate::Result<BlockingClient> {
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()?;

    // Call the asynchronous connect method using the runtime.
    let inner = rt.block_on(obws::Client::connect(host, port, password))?;

    Ok(BlockingClient { inner, rt })
}
impl BlockingClient {
    pub fn get_scenes(&mut self) -> Result<obws::responses::scenes::Scenes, obws::Error> {
        self.rt.block_on(self.inner.scenes().list())
    }
    pub fn get_scene_items(
        &mut self,
        name: &str,
    ) -> Result<Vec<obws::responses::scene_items::SceneItem>, obws::Error> {
        self.rt.block_on(self.inner.scene_items().list(name))
    }
    pub fn get_scene_item(&mut self, scene_name: &str, item_id: i64) -> Result<u32, obws::Error> {
        self.rt
            .block_on(self.inner.scene_items().index(scene_name, item_id))
    }
    pub fn create_scene_item(
        &mut self,
        main_scene_name: &str,
        scene_name: &str,
    ) -> Result<i64, obws::Error> {
        self.rt.block_on(self.inner.scene_items().create(
            obws::requests::scene_items::CreateSceneItem {
                scene: main_scene_name,
                source: scene_name,
                enabled: Some(true),
            },
        ))
    }
    pub fn remove_scene_item(&mut self, scene_name: &str, item_id: i64) -> Result<(), obws::Error> {
        self.rt
            .block_on(self.inner.scene_items().remove(scene_name, item_id))
    }
    pub fn set_scene_item_index(
        &mut self,
        item_id: i64,
        index: u32,
        scene: &str,
    ) -> Result<(), obws::Error> {
        self.rt.block_on(self.inner.scene_items().set_index(
            obws::requests::scene_items::SetIndex {
                index,
                item_id,
                scene,
            },
        ))
    }
    pub fn set_programm_scene(&mut self, scene_name: &str) -> Result<(), obws::Error> {
        self.rt
            .block_on(self.inner.scenes().set_current_program_scene(scene_name))
    }
    pub fn set_preview_scene(&mut self, scene_name: &str) -> Result<(), obws::Error> {
        self.rt
            .block_on(self.inner.ui().set_studio_mode_enabled(true))?;
        self.rt
            .block_on(self.inner.scenes().set_current_preview_scene(scene_name))
    }
}
