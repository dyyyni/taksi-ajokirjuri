use printpdf::*;
use std::fs::File;
use std::io::BufWriter;

pub fn generate_summary_pdf(
    summary: (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64),
    year_month: &str,
    car: &str,
) {
    let (
        total_aloituslukema, total_ammattiajo, total_tuottamaton_ajo, total_yksityinen_ajo, total_loppulukema,
        total_käteisajotulot, total_pankkikorttitulot, total_luottokorttitulot, total_kela_suorakorvaus, total_taksikortti, total_laskutettavat
    ) = summary;

    let (doc, page1, layer1) = PdfDocument::new("Summary Report", Mm(210.0), Mm(297.0), "Layer 1");
    let current_layer = doc.get_page(page1).get_layer(layer1);

    let font = doc.add_builtin_font(BuiltinFont::Helvetica).unwrap();
    let font_bold = doc.add_builtin_font(BuiltinFont::HelveticaBold).unwrap();
    let font_size = 12.0;
    let font_h1 = 20.0;

    let mut y_position = Mm(270.0);

    current_layer.use_text("Kuukausikohtainen yhteenveto", font_h1, Mm(15.0), y_position, &font_bold);
    y_position -= Mm(10.0);

    current_layer.use_text(format!("Kuukausi:   {}", year_month), font_size, Mm(15.0), y_position, &font);
    y_position -= Mm(7.5);
    current_layer.use_text(format!("Auto:          {}", car), font_size, Mm(15.0), y_position, &font);
    y_position -= Mm(15.0);

    current_layer.use_text("Ajokilometrit", font_h1, Mm(15.0), y_position, &font_bold);
    y_position -= Mm(15.0);

    let ajokilometrit_data = vec![
        ("Mittarin aloituslukema", total_aloituslukema, "km"),
        ("Ammattiajo", total_ammattiajo, "km"),
        ("Tuottamaton ajo", total_tuottamaton_ajo, "km"),
        ("Yksityinen ajo", total_yksityinen_ajo, "km"),
        ("Mittarin loppulukema", total_loppulukema, "km"),
    ];

    for (label, value, unit) in ajokilometrit_data {
        current_layer.use_text(format!("{}: {:.2} {}", label, value, unit), font_size, Mm(15.0), y_position, &font);
        y_position -= Mm(10.0);
    }

    y_position -= Mm(10.0);
    current_layer.use_text("Ajotulojen erittely", font_h1, Mm(15.0), y_position, &font_bold);
    y_position -= Mm(15.0);

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
        y_position -= Mm(10.0);
    }

    let output_file = match File::create("summary_report.pdf") {
        Ok(file) => file,
        Err(e) => {
            eprintln!("Failed to create file: {}", e);
            return;
        }
    };
    let mut output_writer = BufWriter::new(output_file);

    if let Err(e) = doc.save(&mut output_writer) {
        eprintln!("Failed to save PDF: {}", e);
    }
}