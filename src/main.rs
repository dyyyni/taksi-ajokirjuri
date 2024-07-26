mod db;
mod ui;
mod pdf;
mod config;

use config::CAR1;
use eframe::egui;
use eframe::{self, NativeOptions};
use chrono::{NaiveDate, Local};
use std::error::Error;
use std::time::Instant;

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
    message_set_time: Option<Instant>,
}

impl MyApp {
    fn new() -> Self {
        let mut app = Self {
            date: Local::now().naive_local().date(),
            car: CAR1.to_string(),
            ..Default::default()
        };

        if let Err(e) = ui::load_entries(&mut app) {
            eprintln!("Error fetching the initial values: {:?}", e);
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