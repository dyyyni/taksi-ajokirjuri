use printpdf::*;
use std::fs::File;
use std::io::BufWriter;

pub fn generate_summary_pdf(summary: (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64)) {
    let (
        total_aloituslukema,
        total_ammattiajo,
        total_tuottamaton_ajo,
        total_yksityinen_ajo,
        total_loppulukema,
        total_käteisajotulot,
        total_pankkikorttitulot,
        total_luottokorttitulot,
        total_kela_suorakorvaus,
        total_taksikortti,
        total_laskutettavat
    ) = summary;

    let (doc, page1, layer1) = PdfDocument::new("Monthly Summary", Mm(210.0), Mm(297.0), "Layer 1");
    let current_layer = doc.get_page(page1).get_layer(layer1);

    let font = doc.add_builtin_font(BuiltinFont::Helvetica).unwrap();

    current_layer.use_text(format!("Total Matkamittarin Aloituslukema: {:.2}", total_aloituslukema), 12.0, Mm(10.0), Mm(280.0), &font);
    current_layer.use_text(format!("Total Ammattiajo: {:.2}", total_ammattiajo), 12.0, Mm(10.0), Mm(270.0), &font);
    current_layer.use_text(format!("Total Tuottamaton Ajo: {:.2}", total_tuottamaton_ajo), 12.0, Mm(10.0), Mm(260.0), &font);
    current_layer.use_text(format!("Total Yksityinen Ajo: {:.2}", total_yksityinen_ajo), 12.0, Mm(10.0), Mm(250.0), &font);
    current_layer.use_text(format!("Total Matkamittarin Loppulukema: {:.2}", total_loppulukema), 12.0, Mm(10.0), Mm(240.0), &font);
    current_layer.use_text(format!("Total Käteisajotulot: {:.2}", total_käteisajotulot), 12.0, Mm(10.0), Mm(230.0), &font);
    current_layer.use_text(format!("Total Pankkikorttitulot: {:.2}", total_pankkikorttitulot), 12.0, Mm(10.0), Mm(220.0), &font);
    current_layer.use_text(format!("Total Luottokorttitulot: {:.2}", total_luottokorttitulot), 12.0, Mm(10.0), Mm(210.0), &font);
    current_layer.use_text(format!("Total Kela Suorakorvaus: {:.2}", total_kela_suorakorvaus), 12.0, Mm(10.0), Mm(200.0), &font);
    current_layer.use_text(format!("Total Taksikortti: {:.2}", total_taksikortti), 12.0, Mm(10.0), Mm(190.0), &font);
    current_layer.use_text(format!("Total Laskutettavat: {:.2}", total_laskutettavat), 12.0, Mm(10.0), Mm(180.0), &font);

    doc.save(&mut BufWriter::new(File::create("monthly_summary.pdf").unwrap())).unwrap();
}