use eframe::egui;
impl crate::ObsSwitcher {
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
impl eframe::App for crate::ObsSwitcher {
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
                        // self.client.set_preview_scene(&scene.name).unwrap();
                        test(self, &scene);
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
                        // let index = this.client.get_scene_item(main_scene, id).unwrap() + 1;
                        // this.client
                        //     .set_scene_item_index(source.id, source.index + 1, main_scene)
                        //     .unwrap();
                        // println!(
                        //     "{}",
                        // );
                        this.client
                            .set_programm_scene(&this.data.main_scene_name)
                            .unwrap();
                    }
                }
            }
        }
    }
}
