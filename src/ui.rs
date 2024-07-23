use eframe::egui;
use egui_extras::DatePickerButton;
use crate::MyApp;
use crate::db::{insert_entry, get_entry_by_date, Entry, get_monthly_summary};
use crate::pdf::generate_summary_pdf;
use rusqlite::Connection;
use chrono::Datelike;

pub fn build_ui(app: &mut MyApp, ctx: &egui::Context) {
    egui::CentralPanel::default().show(ctx, |ui| {
        ui.heading("Syötä tiedot");

        // Store the previous date
        let previous_date = app.date;

        ui.horizontal(|ui| {
            ui.label("Päivämäärä:");
            ui.add(DatePickerButton::new(&mut app.date).id_source("date_picker"));
        });

        // Check if the date has changed
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

        ui.horizontal(|ui| {
            ui.label("Matkamittarin aloituslukema (km):");
            ui.text_edit_singleline(&mut app.matkamittarin_aloituslukema);
        });

        ui.horizontal(|ui| {
            ui.label("Ammattiajo (km):");
            ui.text_edit_singleline(&mut app.ammattiajo);
        });

        ui.horizontal(|ui| {
            ui.label("Tuottamaton ajo (km):");
            ui.text_edit_singleline(&mut app.tuottamaton_ajo);
        });

        ui.horizontal(|ui| {
            ui.label("Yksityinen ajo (km):");
            ui.text_edit_singleline(&mut app.yksityinen_ajo);
        });

        ui.horizontal(|ui| {
            ui.label("Matkamittarin loppulukema (km):");
            ui.text_edit_singleline(&mut app.matkamittarin_loppulukema);
        });

        ui.horizontal(|ui| {
            ui.label("Käteisajotulot (€):");
            ui.text_edit_singleline(&mut app.käteisajotulot);
        });

        ui.horizontal(|ui| {
            ui.label("Pankkikorttitulot (€):");
            ui.text_edit_singleline(&mut app.pankkikorttitulot);
        });

        ui.horizontal(|ui| {
            ui.label("Luottokorttitulot (€):");
            ui.text_edit_singleline(&mut app.luottokorttitulot);
        });

        ui.horizontal(|ui| {
            ui.label("Kela suorakorvaus (€):");
            ui.text_edit_singleline(&mut app.kela_suorakorvaus);
        });

        ui.horizontal(|ui| {
            ui.label("Taksikortti (€):");
            ui.text_edit_singleline(&mut app.taksikortti);
        });

        ui.horizontal(|ui| {
            ui.label("Laskutettavat (€):");
            ui.text_edit_singleline(&mut app.laskutettavat);
        });

        if ui.button("Save").clicked() {
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

        if ui.button("Generate Report").clicked() {
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

        ui.label(&app.message);
    });
}