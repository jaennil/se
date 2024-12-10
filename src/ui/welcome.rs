use eframe::egui;

pub fn render(ctx: &egui::Context) {
    egui::CentralPanel::default().show(ctx, |ui| {
        ui.vertical_centered(|ui| {
            //Add a mock logo or any graphic here
            //ui.add_sized([100.0, 100.0], egui::widgets::Image::new("path_to_logo.png", [100.0, 100.0]));
            //ui.add_space(20.0);

            if ui.button("Open Last Opened Database").clicked() {
                //app.state = State::AllTransactions;
                ctx.set_embed_viewports(true);
                ctx.show_viewport_immediate(
                    egui::ViewportId::from_hash_of("immediate_viewport"),
                    egui::ViewportBuilder::default(),
                    //.with_title("Immediate Viewport"),
                    //.with_inner_size([200.0, 100.0]),
                    |ctx, class| {
                        assert!(
                            class == egui::ViewportClass::Immediate,
                            "This egui backend doesn't support multiple viewports"
                        );
                    },
                );
            }

            if ui.button("New Database").clicked() {
                println!("Create New Database");
            }

            if ui.button("Open Existing Database").clicked() {
                println!("Open Existing Database");
            }

            if ui.button("User Interface Language").clicked() {}

            if ui.button("User Manual").clicked() {}

            if ui.button("Website").clicked() {}

            if ui.button("Support Forums").clicked() {}

            let mut checked = true;
            ui.checkbox(&mut checked, "Show this dialog box at startup");

            ui.add_space(20.0);

            if ui.button("Exit").clicked() {
                std::process::exit(0);
            }
        });
    });
}
