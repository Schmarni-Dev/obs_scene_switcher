use eframe::egui;
use obws::responses::{scene_items::SourceType, scenes::Scene};

use crate::ObsSwitcher;
impl ObsSwitcher {
    pub fn new(
        cc: &eframe::CreationContext<'_>,
        host: String,
        port: u16,
        password: String,
    ) -> Self {
        let mut style: egui::Style = (*cc.egui_ctx.style()).clone();
        style.visuals.button_frame = true;
        // style.visuals.
        cc.egui_ctx.set_style(style);
        Self {
            client: crate::blocking_obs_client::connect(host, port.clone(), Some(password))
                .expect("connect to ws server"),
            data: crate::SavedData {
                main_scene_name: "".to_owned(),
            },
        }
    }
}

impl ObsSwitcher {
    pub fn switch_inner_scene(&mut self, new_scene: &Scene) -> Option<()> {
        let inject = self.find_inject()?;
        let sources = self
            .client
            .get_scene_items(&self.data.main_scene_name)
            .ok()?;
        let w = sources
            .iter()
            .enumerate()
            .map(|t| {
                println!("{}: {} :{}", t.0, t.1.source_name, t.1.index);
                t.1
            })
            .nth((inject.index - 1) as usize)?;
        println!("{}", w.source_name);
        if w.source_name != "</Inject>" && w.source_type == SourceType::Scene {
            self.client
                .remove_scene_item(&self.data.main_scene_name, w.id)
                .ok()?;
        }
        let inject = self.find_inject()?;

        let id = self
            .client
            .create_scene_item(&self.data.main_scene_name, &new_scene.name)
            .unwrap();
        self.client
            .set_scene_item_index(id, inject.index, &self.data.main_scene_name)
            .unwrap();

        self.client
            .set_programm_scene(&self.data.main_scene_name)
            .unwrap();

        Some(())
    }
    pub fn find_inject(&self) -> Option<obws::responses::scene_items::SceneItem> {
        let sources = self
            .client
            .get_scene_items(&self.data.main_scene_name)
            .ok()?;

        sources
            .iter()
            .map(|source| source.clone())
            .find(|source| source.source_name == "<Inject>")
    }
    pub fn has_inject(&self, scene_name: &str) -> bool {
        if let Ok(sources) = self.client.get_scene_items(scene_name) {
            return sources
                .iter()
                .any(|source| source.source_name == "<Inject>");
        }
        return false;
    }
}

impl eframe::App for ObsSwitcher {
    fn on_close_event(&mut self) -> bool {
        println!("Bye");
        true
    }

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ctx.set_pixels_per_point(2.0);
            ui.horizontal(|ui| {
                ui.heading(format!("Main Scene: {} \n", self.main_scene_name));
            });
            let non_main_scenes = self
                .client
                .get_scenes()
                .unwrap()
                .scenes
                .into_iter()
                .filter(|s| !self.has_inject(&s.name))
                .rev()
                .collect::<Vec<_>>();
            let main_scenes = self
                .client
                .get_scenes()
                .unwrap()
                .scenes
                .into_iter()
                .filter(|s| self.has_inject(&s.name))
                .rev()
                .collect::<Vec<_>>();

            for scene in non_main_scenes {
                ui.horizontal(|ui| {
                    ui.label(format!("Scene: {}", scene.name));
                    if self.data.main_scene_name != scene.name
                        && ui.button("Switch to Scene!").clicked()
                    {
                        let _ = self.switch_inner_scene(&scene);
                    }
                });
            }
            ui.heading("\nSelect Main Scene:");
            for scene in main_scenes {
                ui.horizontal(|ui| {
                    if self.data.main_scene_name == scene.name {
                        return;
                    }

                    ui.label(format!("Scene: {}", scene.name));
                    if ui.button("Set as Main Scene!").clicked() {
                        self.data.main_scene_name = scene.name;
                    }
                });
            }
        });
    }
}
