mod db;
mod ui;
mod pdf;
mod config;

use config::CAR1;
use eframe::egui;
use eframe::{self, NativeOptions};
use chrono::{NaiveDate, Local};
use std::error::Error;
use rusqlite::Connection;

#[derive(Default)]
struct MyApp {
    date: NaiveDate,
    car: String,
    matkamittarin_aloituslukema: String,
    ammattiajo: String,
    tuottamaton_ajo: String,
    yksityinen_ajo: String,
    matkamittarin_loppulukema: String,
    käteisajotulot: String,
    pankkikorttitulot: String,
    kela_suorakorvaus: String,
    taksikortti: String,
    laskutettavat: String,
    message: String,
}

impl MyApp {
    fn new() -> Self {
        let mut app = Self {
            date: Local::now().naive_local().date(),
            car: CAR1.to_string(),
            ..Default::default()
        };
        let conn = Connection::open("data/data.db").unwrap();
        match crate::db::get_entry_by_date_and_car(&conn, &app.date.to_string(), &app.car) {
            Ok(Some(entry)) => {
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
            }
            _ => eprintln!("Error fetching the initial values."),
        }
        return app;
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        ui::build_ui(self, ctx);
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    db::initialize_db()?;
    let native_options = NativeOptions::default();

    eframe::run_native(
        "Ajopäiväkirja",
        native_options,
        Box::new(|_cc| Ok(Box::new(MyApp::new()))),
    )?;

    Ok(())
}