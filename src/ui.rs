use eframe::egui;
use egui_extras::DatePickerButton;
use rusqlite::Connection;
use chrono::Datelike;

use crate::MyApp;
use crate::db::{insert_entry, get_entry_by_date_and_car, Entry, get_monthly_summary};
use crate::config::{CAR1, CAR2, CAR3};

pub fn load_entries(app: &mut MyApp) -> Result<(), Box<dyn std::error::Error>> {
    let conn = Connection::open("data/data.db")?;
    if let Ok(Some(entry)) = get_entry_by_date_and_car(&conn, &app.date.to_string(), &app.car) {
        app.matkamittarin_aloituslukema = entry.matkamittarin_aloituslukema.to_string();
        app.ammattiajo = entry.ammattiajo.to_string();
        app.tuottamaton_ajo = entry.tuottamaton_ajo.to_string();
        app.yksityinen_ajo = entry.yksityinen_ajo.to_string();
        app.matkamittarin_loppulukema = entry.matkamittarin_loppulukema.to_string();
        app.käteisajotulot = entry.käteisajotulot.to_string();
        app.pankkikorttitulot = entry.pankkikorttitulot.to_string();
        app.kela_suorakorvaus = entry.kela_suorakorvaus.to_string();
        app.taksikortti = entry.taksikortti.to_string();
        app.laskutettavat = entry.laskutettavat.to_string();
    } else {
        clear_ui_entries(app);
    }
    Ok(())
}

fn clear_ui_entries(app: &mut MyApp) {
    app.matkamittarin_aloituslukema.clear();
    app.ammattiajo.clear();
    app.tuottamaton_ajo.clear();
    app.yksityinen_ajo.clear();
    app.matkamittarin_loppulukema.clear();
    app.käteisajotulot.clear();
    app.pankkikorttitulot.clear();
    app.kela_suorakorvaus.clear();
    app.taksikortti.clear();
    app.laskutettavat.clear(); 
} 

pub fn build_ui(app: &mut MyApp, ctx: &egui::Context) {
    egui::CentralPanel::default().show(ctx, |ui| {
        ui.heading("Valitse päivämäärä");

        let previous_date = app.date;
        let previous_car = app.car.clone();

        ui.add_sized(
            [200.0, 40.0],
            DatePickerButton::new(&mut app.date).id_source("date_picker"));

        ui.add_space(15.0);

        ui.heading("Valitse auto");
        egui::ComboBox::from_label("")
            .selected_text(&app.car)
            .show_ui(ui, |ui| {
                ui.selectable_value(&mut app.car, CAR1.to_string(), CAR1);
                ui.selectable_value(&mut app.car, CAR2.to_string(), CAR2);
                ui.selectable_value(&mut app.car, CAR3.to_string(), CAR3);
            });

        ui.add_space(15.0);

        if app.date != previous_date || app.car != previous_car {
            if let Err(e) = load_entries(app) {
                println!("Error loading entries: {:?}", e);
            }
        }

        egui::Grid::new("entry_grid")
            .num_columns(3)
            .spacing([20.0, 10.0])
            .striped(true)
            .show(ui, |ui| {
                ui.heading("Ajokilometrit");
                ui.end_row();

                ui.label("Mittarin aloituslukema:");
                ui.add_sized(
                    [200.0, 20.0],
                    egui::TextEdit::singleline(&mut app.matkamittarin_aloituslukema));
                ui.label("km");
                ui.end_row();

                ui.label("Ammattiajo:");
                ui.add_sized(
                    [200.0, 20.0],
                    egui::TextEdit::singleline(&mut app.ammattiajo));
                ui.label("km");
                ui.end_row();

                ui.label("Tuottamaton ajo:");
                ui.add_sized(
                    [200.0, 20.0],
                    egui::TextEdit::singleline(&mut app.tuottamaton_ajo));
                ui.label("km");
                ui.end_row();

                ui.label("Yksityinen ajo:");
                ui.add_sized(
                    [200.0, 20.0],
                    egui::TextEdit::singleline(&mut app.yksityinen_ajo));
                ui.label("km");
                ui.end_row();

                ui.label("Mittarin loppulukema:");
                ui.add_sized(
                    [200.0, 20.0],
                    egui::TextEdit::singleline(&mut app.matkamittarin_loppulukema));
                ui.label("km");
                ui.end_row();

                ui.heading("Ajotulojen erittely");
                ui.end_row();

                ui.label("Käteisajotulot:");
                ui.add_sized(
                    [200.0, 20.0],
                    egui::TextEdit::singleline(&mut app.käteisajotulot));
                ui.label("€");
                ui.end_row();

                ui.label("Pankkikorttitulot:");
                ui.add_sized(
                    [200.0, 20.0],
                    egui::TextEdit::singleline(&mut app.pankkikorttitulot));
                ui.label("€");
                ui.end_row();

                ui.label("Kela suorakorvaus:");
                ui.add_sized(
                    [200.0, 20.0],
                    egui::TextEdit::singleline(&mut app.kela_suorakorvaus));
                ui.label("€");
                ui.end_row();

                ui.label("Taksikortti:");
                ui.add_sized(
                    [200.0, 20.0],
                    egui::TextEdit::singleline(&mut app.taksikortti));
                ui.label("€");
                ui.end_row();

                ui.label("Laskutettavat:");
                ui.add_sized(
                    [200.0, 20.0],
                    egui::TextEdit::singleline(&mut app.laskutettavat));
                ui.label("€");
                ui.end_row();
            });

        ui.add_space(20.0);

        ui.horizontal(|ui| {
            if ui.button("Tallenna").clicked() {
                let conn = Connection::open("data/data.db").unwrap();
                let entry = Entry {
                    date: app.date.to_string(),
                    car: app.car.clone(),
                    matkamittarin_aloituslukema: app.matkamittarin_aloituslukema.parse().unwrap_or(0.0),
                    ammattiajo: app.ammattiajo.parse().unwrap_or(0.0),
                    tuottamaton_ajo: app.tuottamaton_ajo.parse().unwrap_or(0.0),
                    yksityinen_ajo: app.yksityinen_ajo.parse().unwrap_or(0.0),
                    matkamittarin_loppulukema: app.matkamittarin_loppulukema.parse().unwrap_or(0.0),
                    käteisajotulot: app.käteisajotulot.parse().unwrap_or(0.0),
                    pankkikorttitulot: app.pankkikorttitulot.parse().unwrap_or(0.0),
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

            if ui.button("Luo kuukausiraportti").clicked() {
                let conn = Connection::open("data/data.db").unwrap();
                let year_month = format!("{:04}-{:02}", app.date.year(), app.date.month());
                match get_monthly_summary(&conn, &year_month, &app.car) {
                    Ok(summary) => {
                        crate::pdf::generate_monthly_summary_pdf(summary, &year_month, &app.car);
                        app.message = "Kuukausiraportti valmis!".to_string();
                    }
                    Err(e) => {
                        app.message = format!("Failed to generate report: {}", e);
                    }
                }
            }

            if ui.button("Luo päiväraportti").clicked() {
                crate::pdf::generate_daily_summary_pdf(&app);
                app.message = "Päiväraportti valmis!".to_string();
            }

            if ui.button("Tyhjennä kentät").clicked() {
                clear_ui_entries(app);
                app.message = "Kentät tyhjennetty!".to_string();
            }
        });

        ui.label(&app.message);
    });
}
