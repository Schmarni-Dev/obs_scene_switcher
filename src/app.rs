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
                println!("{}: {} :{}", t.0, t.1.source_name,t.1.index);
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
    pub fn find_inject(&mut self) -> Option<obws::responses::scene_items::SceneItem> {
        let sources = self
            .client
            .get_scene_items(&self.data.main_scene_name)
            .ok()?;

        sources
            .iter()
            .map(|source| source.clone())
            .find(|source| source.source_name == "<Inject>")
    }
}

impl eframe::App for ObsSwitcher {
    fn on_close_event(&mut self) -> bool {
        println!("Bye");
        true
    }

    // #[tokio::main]
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            for scene in self.client.get_scenes().unwrap().scenes.into_iter().rev() {
                ui.horizontal(|ui| {
                    if self.data.main_scene_name == scene.name {
                        ui.heading(format!("Main Scene: {} \n", scene.name));
                    } else {
                        ui.heading(format!("Scene: {}", scene.name));
                    }
                    if self.data.main_scene_name != scene.name
                        && ui.button("Switch to Scene!").clicked()
                    {
                        let _ = self.switch_inner_scene(&scene);
                        // self.client.set_preview_scene(&scene.name).unwrap();
                        // test(self, &scene);
                        // self.client.set_preview_scene(&scene.name).unwrap();
                    }
                    if self.data.main_scene_name != scene.name
                        && ui.button("Set as Main Scene!").clicked()
                    {
                        self.data.main_scene_name = scene.name;
                    }
                });
            }
        });

        fn test(this: &mut crate::ObsSwitcher, scene: &obws::responses::scenes::Scene) {
            let sources = this
                .client
                .get_scene_items(&this.data.main_scene_name)
                .unwrap();
            // let index = find_inject(this.client, &this.data.main_scene_name)
            // ;
            // source.unwrap().index
            for i in 0..sources.len() {
                let source = &sources[i];

                if source.source_name == "<Inject>" {
                    match &sources.get(i - 1) {
                        Some(s) => {
                            println!("{}", s.source_name);
                            if s.source_name != "<Inject/>"
                                && s.source_type == obws::responses::scene_items::SourceType::Scene
                            {
                                this.client
                                    .remove_scene_item(&this.data.main_scene_name, s.id)
                                    .unwrap();
                            }
                        }
                        _ => {}
                    }
                }
            }
            let mut found_inject = false;
            let sources = this
                .client
                .get_scene_items(&this.data.main_scene_name)
                .unwrap();
            for i in 0..sources.len() {
                let source = &sources[i];
                if !found_inject {
                    // println!("{:?}", source.source_name == "<Inject>");
                    println!("{:?}", source.source_name == "<Inject>");
                    // println!("{:?}", source.index - 1);
                    found_inject = source.source_name == "<Inject>";
                    if found_inject {
                        let id = this
                            .client
                            .create_scene_item(&this.data.main_scene_name, &scene.name)
                            .unwrap();
                        this.client
                            .set_scene_item_index(id, source.index, &this.data.main_scene_name)
                            .unwrap();

                        this.client
                            .set_programm_scene(&this.data.main_scene_name)
                            .unwrap();
                    }
                }
            }
        }
    }
}

// TODO make work
