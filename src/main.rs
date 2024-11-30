use eframe::egui;
mod welcome;

fn main() {
    let native_options = eframe::NativeOptions::default();
    // TODO:
    let _ = eframe::run_native("My egui App", native_options, Box::new(|cc| Ok(Box::new(App::new(cc)))));
}

#[derive(Default)]
struct App {
    state: State,
    view: View,
}

impl App {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        // Customize egui here with cc.egui_ctx.set_fonts and cc.egui_ctx.set_visuals.
        // Restore app state using cc.storage (requires the "persistence" feature).
        // Use the cc.gl (a glow::Context) to create graphics shaders and buffers that you can use
        // for e.g. egui::PaintCallback.
        Self::default()
    }

    fn render_home(&self, ui: &mut egui::Ui) {
        ui.heading("Home");
    }

    fn render_all_transactions(&self, ui: &mut egui::Ui) {
        ui.heading("All Transactions");
    }

    fn render_assets(&self, ui: &mut egui::Ui) {
        ui.heading("Assets");
    }
}

#[derive(Default)]
enum State {
    #[default]
    Welcome,
    Home,
}

#[derive(Default)]
enum View {
    #[default]
    AllTransactions,
    Assets,
}

impl eframe::App for App {
   fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
       egui::CentralPanel::default().show(ctx, |ui| {
            match self.state {
                State::Welcome => welcome::render_welcome(ui),
                State::Home => self.render_home(ui),
            }
       });
       // egui::TopBottomPanel::top("menu_bar").show(ctx, |ui| {
       //     egui::menu::bar(ui, |ui| {
       //         if ui.button("File").clicked() {
       //             println!("File menu clicked!");
       //         }
       //         if ui.button("Accounts").clicked() {
       //             println!("Accounts menu clicked!");
       //         }
       //         if ui.button("Tools").clicked() {
       //             println!("Tools menu clicked!");
       //         }
       //         if ui.button("View").clicked() {
       //             println!("View menu clicked!");
       //         }
       //         if ui.button("Help").clicked() {
       //             println!("Help menu clicked!");
       //         }
       //     });
       // });
       //
       // // Toolbar with icons
       // egui::TopBottomPanel::top("toolbar").show(ctx, |ui| {
       //     ui.horizontal(|ui| {
       //         for i in 0..18 {
       //             if ui.add(egui::Button::new(egui::RichText::new("🔘").size(24.0))).clicked() {
       //                 println!("Button {} clicked!", i + 1);
       //             }
       //         }
       //     });
       // });
       //
       //
       //  egui::SidePanel::left("menu_panel").min_width(300.0).show(ctx, |ui| {
       //     ui.collapsing("Dashboard", |ui| {
       //         if ui.button("All Transactions").clicked() {
       //             self.view = View::AllTransactions;
       //         }
       //
       //         ui.collapsing("Favorites", |ui| {
       //             ui.button("jaenni-account");
       //         });
       //         ui.collapsing("Cash Accounts", |ui| {
       //             ui.button("jaenni-account");
       //         });
       //         if ui.button("Assets").clicked() {
       //             self.view = View::Assets;
       //         }
       //         ui.button("Scheduled Transactions");
       //         ui.button("Budget Planner");
       //         ui.button("Transaction Report");
       //         ui.collapsing("Reports", |ui| {
       //             ui.collapsing("Cash Flow", |ui| {
       //                 ui.button("Daily");
       //                 ui.button("Monthly");
       //                 ui.button("Transactions");
       //             });
       //
       //             ui.collapsing("Categories", |ui| {
       //                 ui.button("Monthly");
       //                 ui.button("Summary");
       //                 ui.button("Where the Money Goes");
       //                 ui.button("Where the Money Comes From");
       //             });
       //
       //             ui.button("Forecast Report");
       //
       //             ui.collapsing("Income vs Expenses", |ui| {
       //                 ui.button("Monthly");
       //                 ui.button("My Usage");
       //                 ui.button("Payees");
       //             });
       //
       //             ui.button("My Usage");
       //             ui.button("Payees");
       //
       //             ui.collapsing("Summary of Accounts", |ui| {
       //                 ui.button("Monthly");
       //                 ui.button("Yearly");
       //             });
       //         });
       //         ui.button("General Report Manager");
       //         ui.button("Help");
       //     });
       //
       // });
       //
       //egui::CentralPanel::default().show(ctx, |ui| {
       //     match self.view {
       //         View::AllTransactions => self.render_all_transactions(ui),
       //         View::Assets => self.render_assets(ui),
       //     }
       //});
   }
}
