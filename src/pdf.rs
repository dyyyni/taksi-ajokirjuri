use printpdf::*;
use std::fs::File;
use std::io::BufWriter;

pub fn generate_summary_pdf(summary: (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64)) {
    let (total_aloituslukema, total_ammattiajo, total_tuottamaton_ajo, total_yksityinen_ajo, total_loppulukema,
         total_käteisajotulot, total_pankkikorttitulot, total_luottokorttitulot, total_kela_suorakorvaus, total_taksikortti, total_laskutettavat) = summary;

    let (doc, page1, layer1) = PdfDocument::new("Summary Report", Mm(210.0), Mm(297.0), "Layer 1");
    let current_layer = doc.get_page(page1).get_layer(layer1);

    let font = doc.add_builtin_font(BuiltinFont::Helvetica).unwrap();
    let font_size = 12.0;

    let mut y_position = Mm(270.0);

    current_layer.use_text("Ajokilometrit", font_size, Mm(15.0), y_position, &font);
    y_position -= Mm(10.0);

    let ajokilometrit_data = vec![
        ("Mittarin aloituslukema", total_aloituslukema, "km"),
        ("Ammattiajo", total_ammattiajo, "km"),
        ("Tuottamaton ajo", total_tuottamaton_ajo, "km"),
        ("Yksityinen ajo", total_yksityinen_ajo, "km"),
        ("Mittarin loppulukema", total_loppulukema, "km"),
    ];

    for (label, value, unit) in ajokilometrit_data {
        current_layer.use_text(format!("{}: {:.2} {}", label, value, unit), font_size, Mm(15.0), y_position, &font);
        y_position -= Mm(7.0);
    }

    y_position -= Mm(10.0);
    current_layer.use_text("Ajotulojen erittely", font_size, Mm(15.0), y_position, &font);
    y_position -= Mm(10.0);

    let ajotulojen_erittely_data = vec![
        ("Käteisajotulot", total_käteisajotulot, "€"),
        ("Pankkikorttitulot", total_pankkikorttitulot, "€"),
        ("Luottokorttitulot", total_luottokorttitulot, "€"),
        ("Kela suorakorvaus", total_kela_suorakorvaus, "€"),
        ("Taksikortti", total_taksikortti, "€"),
        ("Laskutettavat", total_laskutettavat, "€"),
    ];

    for (label, value, unit) in ajotulojen_erittely_data {
        current_layer.use_text(format!("{}: {:.2} {}", label, value, unit), font_size, Mm(15.0), y_position, &font);
        y_position -= Mm(7.0);
    }

    let output_file = File::create("summary_report.pdf").unwrap();
    let mut output_writer = BufWriter::new(output_file);

    doc.save(&mut output_writer).unwrap();
}