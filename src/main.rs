mod db;
mod ui;
mod pdf;

use eframe::egui;
use eframe::{self, NativeOptions};
use db::initialize_db;
use chrono::{NaiveDate, Local};
use std::error::Error;

#[derive(Default)]
struct MyApp {
    date: NaiveDate,
    matkamittarin_aloituslukema: String,
    ammattiajo: String,
    tuottamaton_ajo: String,
    yksityinen_ajo: String,
    matkamittarin_loppulukema: String,
    käteisajotulot: String,
    pankkikorttitulot: String,
    luottokorttitulot: String,
    kela_suorakorvaus: String,
    taksikortti: String,
    laskutettavat: String,
    message: String,
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        ui::build_ui(self, ctx);
    }
}

fn main() -> Result<(), Box<dyn Error>> {

    initialize_db()?;

    let native_options = NativeOptions::default();

    eframe::run_native(
        "Ajopäiväkirja",
        native_options,
        Box::new(|_cc| Ok(Box::new(MyApp {
            date: Local::now().naive_local().date(),
            ..Default::default()
        }))),
    )?;

    Ok(())
}