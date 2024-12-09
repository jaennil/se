use eframe::egui;
use strum::IntoEnumIterator as _;

use crate::models::transaction::{self, Transaction};

#[derive(Default)]
pub struct Home {
    pub state: State,
    pub utility_state: UtilityState,
    pub transaction: Transaction,
}

impl Home {
    pub fn render(&mut self, ctx: &egui::Context) {
        self.render_menu(ctx);
        self.render_toolbar(ctx);
        self.render_sidepanel(ctx);
        self.render_base(ctx);
        self.render_utility(ctx);
    }

    fn render_menu(&self, ctx: &egui::Context) {
        egui::TopBottomPanel::top("menu_bar").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                if ui.button("File").clicked() {
                    println!("File menu clicked!");
                }
                if ui.button("Accounts").clicked() {
                    println!("Accounts menu clicked!");
                }
                if ui.button("Tools").clicked() {
                    println!("Tools menu clicked!");
                }
                if ui.button("View").clicked() {
                    println!("View menu clicked!");
                }
                if ui.button("Help").clicked() {
                    println!("Help menu clicked!");
                }
            });
        });
    }

    fn render_toolbar(&mut self, ctx: &egui::Context) {
        egui::TopBottomPanel::top("toolbar").show(ctx, |ui| {
            ui.horizontal(|ui| {
                if ui
                    .add(egui::ImageButton::new(
                        egui::Image::new(egui::include_image!("../assets/new_database.png"))
                            .fit_to_exact_size(egui::Vec2 { x: 24.0, y: 24.0 }),
                    ))
                    .on_hover_ui(|ui| {
                        ui.label("New Database");
                    })
                    .clicked()
                {
                    self.utility_state = UtilityState::NewDatabase;
                };
                if ui
                    .add(egui::ImageButton::new(
                        egui::Image::new(egui::include_image!("../assets/new_transaction.png"))
                            .fit_to_exact_size(egui::Vec2 { x: 24.0, y: 24.0 }),
                    ))
                    .on_hover_ui(|ui| {
                        ui.label("New Transaction");
                    })
                    .clicked()
                {
                    self.utility_state = UtilityState::NewTransaction;
                };
            });
        });
    }

    fn render_sidepanel(&mut self, ctx: &egui::Context) {
        egui::SidePanel::left("menu_panel")
            .min_width(300.0)
            .show(ctx, |ui| {
                ui.collapsing("Dashboard", |ui| {
                    if ui.button("All Transactions").clicked() {
                        self.state = State::AllTransactions;
                    }

                    ui.collapsing(
                        "Favorites",
                        |ui| {
                            if ui.button("jaennil-account").clicked() {}
                        },
                    );
                    ui.collapsing("Cash Accounts", |ui| {
                        if ui.button("jaennil-account").clicked() {}
                    });
                    if ui.button("Assets").clicked() {
                        //self.view = View::Assets;
                    }
                    if ui.button("Scheduled Transactions").clicked() {}
                    if ui.button("Budget Planner").clicked() {}
                    if ui.button("Transaction Report").clicked() {}
                    ui.collapsing("Reports", |ui| {
                        ui.collapsing("Cash Flow", |ui| {
                            if ui.button("Daily").clicked() {}
                            if ui.button("Monthly").clicked() {}
                            if ui.button("Transactions").clicked() {}
                        });

                        ui.collapsing("Categories", |ui| {
                            if ui.button("Monthly").clicked() {}
                            if ui.button("Summary").clicked() {}
                            if ui.button("Where the Money Goes").clicked() {}
                            if ui.button("Where the Money Comes From").clicked() {}
                        });

                        if ui.button("Forecast Report").clicked() {}

                        ui.collapsing("Income vs Expenses", |ui| {
                            if ui.button("Monthly").clicked() {}
                            if ui.button("My Usage").clicked() {}
                            if ui.button("Payees").clicked() {}
                        });

                        if ui.button("My Usage").clicked() {}
                        if ui.button("Payees").clicked() {}

                        ui.collapsing("Summary of Accounts", |ui| {
                            if ui.button("Monthly").clicked() {}
                            if ui.button("Yearly").clicked() {}
                        });
                    });
                    if ui.button("General Report Manager").clicked() {}
                    if ui.button("Help").clicked() {}
                });
            });
    }

    fn render_base(&self, ctx: &egui::Context) {
        egui::CentralPanel::default().show(ctx, |ui| match self.state {
            _ => {}
        });
    }

    fn render_new_database(&mut self, ctx: &egui::Context) {
        ctx.show_viewport_immediate(
            egui::ViewportId::from_hash_of("new_database"),
            egui::ViewportBuilder::default().with_window_type(egui::X11WindowType::Utility),
            |ctx, class| {
                assert!(
                    class == egui::ViewportClass::Immediate,
                    "This egui backend doesn't support multiple viewports"
                );

                egui::CentralPanel::default().show(ctx, |ui| {
                    ui.label("New Database");
                });
                if ctx.input(|i| i.viewport().close_requested()) {
                    // Tell parent viewport that we should not show next frame:
                    self.utility_state = UtilityState::None;
                }
            },
        );
    }

    fn render_new_transaction(&mut self, ctx: &egui::Context) {
        ctx.show_viewport_immediate(
            egui::ViewportId::from_hash_of("new_transaction"),
            egui::ViewportBuilder::default().with_window_type(egui::X11WindowType::Utility),
            |ctx, class| {
                assert!(
                    class == egui::ViewportClass::Immediate,
                    "This egui backend doesn't support multiple viewports"
                );

                egui::CentralPanel::default().show(ctx, |ui| {
                    let ui_builder = egui::UiBuilder::new();
                    ui.scope_builder(ui_builder, |ui| {
                        egui::Grid::new("my_grid")
                            .num_columns(2)
                            .spacing([40.0, 4.0])
                            .show(ui, |ui| {
                                ui.label("Date");
                                ui.add(egui_extras::DatePickerButton::new(
                                    &mut self.transaction.date,
                                ));
                                ui.end_row();

                                ui.label("Status");
                                egui::ComboBox::from_id_salt("status_combo_box")
                                    .selected_text(self.transaction.status.as_ref())
                                    .show_ui(ui, |ui| {
                                        for status in transaction::Status::iter() {
                                            ui.selectable_value(
                                                &mut self.transaction.status,
                                                status.clone(),
                                                status.as_ref(),
                                            );
                                        }
                                    });
                                ui.end_row();

                                ui.label("Type");
                                egui::ComboBox::from_id_salt("type_combo_box")
                                    .selected_text(self.transaction.typ.as_ref())
                                    .show_ui(ui, |ui| {
                                        for typ in transaction::Type::iter() {
                                            ui.selectable_value(
                                                &mut self.transaction.typ,
                                                typ.clone(),
                                                typ.as_ref(),
                                            );
                                        }
                                    });
                                ui.end_row();

                                ui.label("Amount");
                                if ui
                                    .text_edit_singleline(&mut self.transaction.amount)
                                    .changed()
                                {
                                    only_numbers_repository(&mut self.transaction.amount);
                                }
                                ui.end_row();

                                ui.label("Account");
                                egui::ComboBox::from_id_salt("account_combo_box")
                                    .show_ui(ui, |ui| {});
                                ui.end_row();

                                ui.label("Payee");
                                egui::ComboBox::from_id_salt("payee_combo_box")
                                    .show_ui(ui, |ui| {});
                                ui.end_row();

                                ui.label("Category");
                                egui::ComboBox::from_id_salt("category_combo_box")
                                    .show_ui(ui, |ui| {});
                                ui.end_row();

                                ui.label("Tags");
                                egui::ComboBox::from_id_salt("tags_combo_box").show_ui(ui, |ui| {});
                                ui.end_row();
                            });
                    });
                    ui.horizontal(|ui| {
                        if ui.button("OK").clicked() {
                            dbg!(&self.transaction);
                            //app.database_connection.
                            self.utility_state = UtilityState::None;
                            //let transaction = Transaction{
                            //    date: date,
                            //    status: todo!(),
                            //    typ: todo!(),
                            //    amount: todo!(),
                            //    account: todo!(),
                            //    payee: todo!(),
                            //    category: todo!(),
                            //}
                        }
                        if ui.button("Cancel").clicked() {
                            self.utility_state = UtilityState::None;
                        }
                    });
                });
                if ctx.input(|i| i.viewport().close_requested()) {
                    self.utility_state = UtilityState::None;
                }
            },
        );
    }

    fn render_utility(&mut self, ctx: &egui::Context) {
        match self.utility_state {
            UtilityState::NewDatabase => self.render_new_database(ctx),
            UtilityState::NewTransaction => self.render_new_transaction(ctx),
            _ => {}
        }
    }
}

#[derive(Default)]
enum State {
    #[default]
    Dashboard,
    AllTransactions,
    Favorites,
    BankAccounts,
    Assets,
    ScheduledTransactions,
    BudgetPlanner,
    TransactionReport,
    CashFlow,
    Categories,
    ForecastReport,
    IncomeVsExpenses,
    MyUsage,
    Payees,
    SummaryOfAccounts,
    GeneralReportManager,
    Help,
}

#[derive(Default)]
enum UtilityState {
    #[default]
    None,
    NewDatabase,
    NewTransaction,
}

pub fn only_numbers_repository(s: &mut String) {
    // TODO: add regex for float numbers
    let re = regex::Regex::new(r"[^0-9]+").unwrap();
    *s = re.replace_all(s, "").to_string();
}
