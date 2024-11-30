use eframe::egui;

pub fn render_welcome(ui: &mut egui::Ui) {
     ui.vertical_centered(|ui| {
        // Add a mock logo or any graphic here
        //ui.add_sized([100.0, 100.0], egui::widgets::Image::new("path_to_logo.png", [100.0, 100.0]));
        ui.add_space(20.0);

        // Buttons section
        if ui.button("Open Last Opened Database").clicked() {
            //self.is_welcome = false;
            //self.view = View::AllTransactions; // Transition to the main view
        }

        if ui.button("New Database").clicked() {
            println!("Create New Database");
        }

        if ui.button("Open Existing Database").clicked() {
            println!("Open Existing Database");
        }

        ui.add_space(20.0);
        //ui.checkbox(&mut self.show_dialog_at_startup, "Show this dialog box at startup");

        // Exit button
        if ui.button("Exit").clicked() {
            std::process::exit(0);
        }
    });
}
