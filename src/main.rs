mod db;
mod ui;
mod pdf;

use eframe::egui;
use eframe::{self, NativeOptions};
use db::initialize_db;

#[derive(Default)]
struct MyApp {
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

    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        ui::build_ui(self, ctx);
    }
}

fn main() {
    if let Err(e) = initialize_db() {
        eprintln!("Failed to initialize the database: {}", e);
        return;
    }

    let native_options = NativeOptions::default();
    let _ =eframe::run_native(
        "Ajopäiväkirja",
        native_options,
        Box::new(|_cc| Ok(Box::new(MyApp::default()))),
    );
}
