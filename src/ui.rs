use eframe::egui;
use crate::MyApp;

pub fn build_ui(app: &mut MyApp, ctx: &egui::Context) {
    egui::CentralPanel::default().show(ctx, |ui| {
        ui.heading("Syötä tiedot");

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
            // Placeholder for save logic
            app.message = "Data saved!".to_string();
        }

        if ui.button("Generate Report").clicked() {
            // Placeholder for report generation logic
            app.message = "Report generated!".to_string();
        }

        ui.label(&app.message);
    });
}