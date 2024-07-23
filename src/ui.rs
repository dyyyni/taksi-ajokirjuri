use eframe::egui;
use egui_extras::DatePickerButton;
use crate::MyApp;
use crate::db::{insert_entry, get_entry_by_date, Entry, get_monthly_summary};
use crate::pdf::generate_summary_pdf;
use rusqlite::Connection;
use chrono::Datelike;

pub fn build_ui(app: &mut MyApp, ctx: &egui::Context) {
    egui::CentralPanel::default().show(ctx, |ui| {
        ui.heading("Valitse päivämäärä");

        let previous_date = app.date;

        ui.add(DatePickerButton::new(&mut app.date).id_source("date_picker"));

        if app.date != previous_date {
            let conn = Connection::open("data/data.db").unwrap();
            if let Ok(Some(entry)) = get_entry_by_date(&conn, &app.date.to_string()) {
                app.matkamittarin_aloituslukema = entry.matkamittarin_aloituslukema.to_string();
                app.ammattiajo = entry.ammattiajo.to_string();
                app.tuottamaton_ajo = entry.tuottamaton_ajo.to_string();
                app.yksityinen_ajo = entry.yksityinen_ajo.to_string();
                app.matkamittarin_loppulukema = entry.matkamittarin_loppulukema.to_string();
                app.käteisajotulot = entry.käteisajotulot.to_string();
                app.pankkikorttitulot = entry.pankkikorttitulot.to_string();
                app.luottokorttitulot = entry.luottokorttitulot.to_string();
                app.kela_suorakorvaus = entry.kela_suorakorvaus.to_string();
                app.taksikortti = entry.taksikortti.to_string();
                app.laskutettavat = entry.laskutettavat.to_string();
            } else {
                app.matkamittarin_aloituslukema.clear();
                app.ammattiajo.clear();
                app.tuottamaton_ajo.clear();
                app.yksityinen_ajo.clear();
                app.matkamittarin_loppulukema.clear();
                app.käteisajotulot.clear();
                app.pankkikorttitulot.clear();
                app.luottokorttitulot.clear();
                app.kela_suorakorvaus.clear();
                app.taksikortti.clear();
                app.laskutettavat.clear();
            }
        }

        ui.heading("Ajokilometrit");

        egui::Grid::new("entry_grid")
            .num_columns(2)
            .spacing([20.0, 10.0])
            .striped(true)
            .show(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.label("Mittarin aloituslukema:       ");
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        ui.label("(km)");
                    });
                });
                ui.add(egui::TextEdit::singleline(&mut app.matkamittarin_aloituslukema).desired_width(100.0));
                ui.end_row();

                ui.horizontal(|ui| {
                    ui.label("Ammattiajo:");
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        ui.label("(km)");
                    });
                });
                ui.add(egui::TextEdit::singleline(&mut app.ammattiajo).desired_width(100.0));
                ui.end_row();

                ui.horizontal(|ui| {
                    ui.label("Tuottamaton ajo:");
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        ui.label("(km)");
                    });
                });
                ui.add(egui::TextEdit::singleline(&mut app.tuottamaton_ajo).desired_width(100.0));
                ui.end_row();

                ui.horizontal(|ui| {
                    ui.label("Yksityinen ajo:");
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        ui.label("(km)");
                    });
                });
                ui.add(egui::TextEdit::singleline(&mut app.yksityinen_ajo).desired_width(100.0));
                ui.end_row();

                ui.horizontal(|ui| {
                    ui.label("Mittarin loppulukema:");
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        ui.label("(km)");
                    });
                });
                ui.add(egui::TextEdit::singleline(&mut app.matkamittarin_loppulukema).desired_width(100.0));
                ui.end_row();

                ui.end_row(); // End the row before the heading

                ui.heading("Ajotulojen erittely");
                ui.end_row();

                ui.horizontal(|ui| {
                    ui.label("Käteisajotulot:");
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        ui.label("(€)");
                    });
                });
                ui.add(egui::TextEdit::singleline(&mut app.käteisajotulot).desired_width(100.0));
                ui.end_row();

                ui.horizontal(|ui| {
                    ui.label("Pankkikorttitulot:");
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        ui.label("(€)");
                    });
                });
                ui.add(egui::TextEdit::singleline(&mut app.pankkikorttitulot).desired_width(100.0));
                ui.end_row();

                ui.horizontal(|ui| {
                    ui.label("Luottokorttitulot:");
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        ui.label("(€)");
                    });
                });
                ui.add(egui::TextEdit::singleline(&mut app.luottokorttitulot).desired_width(100.0));
                ui.end_row();

                ui.horizontal(|ui| {
                    ui.label("Kela suorakorvaus:");
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        ui.label("(€)");
                    });
                });
                ui.add(egui::TextEdit::singleline(&mut app.kela_suorakorvaus).desired_width(100.0));
                ui.end_row();

                ui.horizontal(|ui| {
                    ui.label("Taksikortti:");
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        ui.label("(€)");
                    });
                });
                ui.add(egui::TextEdit::singleline(&mut app.taksikortti).desired_width(100.0));
                ui.end_row();

                ui.horizontal(|ui| {
                    ui.label("Laskutettavat:");
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        ui.label("(€)");
                    });
                });
                ui.add(egui::TextEdit::singleline(&mut app.laskutettavat).desired_width(100.0));
                ui.end_row();
            });

        ui.horizontal(|ui| {
            if ui.button("Tallenna").clicked() {
                let conn = Connection::open("data/data.db").unwrap();
                let entry = Entry {
                    date: app.date.to_string(),
                    matkamittarin_aloituslukema: app.matkamittarin_aloituslukema.parse().unwrap_or(0.0),
                    ammattiajo: app.ammattiajo.parse().unwrap_or(0.0),
                    tuottamaton_ajo: app.tuottamaton_ajo.parse().unwrap_or(0.0),
                    yksityinen_ajo: app.yksityinen_ajo.parse().unwrap_or(0.0),
                    matkamittarin_loppulukema: app.matkamittarin_loppulukema.parse().unwrap_or(0.0),
                    käteisajotulot: app.käteisajotulot.parse().unwrap_or(0.0),
                    pankkikorttitulot: app.pankkikorttitulot.parse().unwrap_or(0.0),
                    luottokorttitulot: app.luottokorttitulot.parse().unwrap_or(0.0),
                    kela_suorakorvaus: app.kela_suorakorvaus.parse().unwrap_or(0.0),
                    taksikortti: app.taksikortti.parse().unwrap_or(0.0),
                    laskutettavat: app.laskutettavat.parse().unwrap_or(0.0),
                };

                if let Err(e) = insert_entry(&conn, &entry) {
                    app.message = format!("Failed to save data: {}", e);
                } else {
                    app.message = "Data saved!".to_string();
                }
            }

            if ui.button("Luo Raportti").clicked() {
                let conn = Connection::open("data/data.db").unwrap();
                let month = format!("{:04}-{:02}", app.date.year(), app.date.month());
                match get_monthly_summary(&conn, &month) {
                    Ok(summary) => {
                        generate_summary_pdf(summary);
                        app.message = "Report generated!".to_string();
                    }
                    Err(e) => {
                        app.message = format!("Failed to generate report: {}", e);
                    }
                }
            }
        });

        ui.label(&app.message);
    });
}