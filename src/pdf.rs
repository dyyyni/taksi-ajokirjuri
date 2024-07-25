use printpdf::*;
use std::fs::File;
use std::io::BufWriter;

use crate::MyApp;

pub fn generate_monthly_summary_pdf(
    summary: (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64),
    year_month: &str,
    car: &str,
) {
    let (
        total_aloituslukema, total_ammattiajo, total_tuottamaton_ajo,
        total_yksityinen_ajo, total_loppulukema,
        total_käteisajotulot, total_pankkikorttitulot, total_luottokorttitulot,
        total_kela_suorakorvaus, total_taksikortti, total_laskutettavat
    ) = summary;

    let (doc, page1, layer1) =
        PdfDocument::new("Summary Report", Mm(210.0), Mm(297.0), "Layer 1");
    let current_layer = doc.get_page(page1).get_layer(layer1);

    let font = doc.add_builtin_font(BuiltinFont::Helvetica).unwrap();
    let font_bold = doc.add_builtin_font(BuiltinFont::HelveticaBold).unwrap();
    let font_size = 12.0;
    let font_h1 = 20.0;

    let mut y_position = Mm(270.0);

    current_layer.use_text("Kuukausikohtainen yhteenveto", font_h1, Mm(15.0), y_position, &font_bold);
    draw_underline(&current_layer, Mm(15.0), y_position - Mm(1.5), Mm(105.0));
    y_position -= Mm(10.0);

    current_layer.use_text("Kuukausi:", font_size, Mm(15.0), y_position, &font);
    current_layer.use_text(year_month, font_size, Mm(40.0), y_position, &font);
    y_position -= Mm(7.5);
    current_layer.use_text("Auto:", font_size, Mm(15.0), y_position, &font);
    current_layer.use_text(car, font_size, Mm(40.0), y_position, &font);
    y_position -= Mm(15.0);

    current_layer.use_text("Ajokilometrit", font_h1, Mm(15.0), y_position, &font_bold);
    y_position -= Mm(15.0);

    let ajokilometrit_data = vec![
        ("Mittarin aloituslukema:", total_aloituslukema, "km"),
        ("Ammattiajo:", total_ammattiajo, "km"),
        ("Tuottamaton ajo:", total_tuottamaton_ajo, "km"),
        ("Yksityinen ajo:", total_yksityinen_ajo, "km"),
        ("Mittarin loppulukema:", total_loppulukema, "km"),
    ];

    for (label, value, unit) in ajokilometrit_data {
        current_layer.use_text(label, font_size, Mm(15.0), y_position, &font);
        current_layer.use_text(format!("{}", value), font_size, Mm(80.0), y_position, &font);
        current_layer.use_text(unit, font_size, Mm(100.0), y_position, &font);
        y_position -= Mm(10.0);
    }

    y_position -= Mm(10.0);
    current_layer.use_text("Ajotulojen erittely", font_h1, Mm(15.0), y_position, &font_bold);
    y_position -= Mm(15.0);

    let ajotulojen_erittely_data = vec![
        ("Käteisajotulot:", total_käteisajotulot, "€"),
        ("Pankkikorttitulot:", total_pankkikorttitulot, "€"),
        ("Luottokorttitulot:", total_luottokorttitulot, "€"),
        ("Kela suorakorvaus:", total_kela_suorakorvaus, "€"),
        ("Taksikortti:", total_taksikortti, "€"),
        ("Laskutettavat:", total_laskutettavat, "€"),
    ];

    for (label, value, unit) in ajotulojen_erittely_data {
        current_layer.use_text(label, font_size, Mm(15.0), y_position, &font);
        current_layer.use_text(format!("{:.2}", value), font_size, Mm(80.0), y_position, &font);
        current_layer.use_text(unit, font_size, Mm(100.0), y_position, &font);
        y_position -= Mm(10.0);
    }

    let output_file = match File::create("monthly_report.pdf") {
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

pub fn generate_daily_summary_pdf(app: &MyApp) {
    let (doc, page1, layer1) =
    PdfDocument::new("Summary Report", Mm(210.0), Mm(297.0), "Layer 1");
    let current_layer = doc.get_page(page1).get_layer(layer1);

    let font = doc.add_builtin_font(BuiltinFont::Helvetica).unwrap();
    let font_bold = doc.add_builtin_font(BuiltinFont::HelveticaBold).unwrap();
    let font_size = 12.0;
    let font_h1 = 20.0;

    let mut y_position = Mm(270.0);


    current_layer.use_text("Ajopäiväkirja", font_h1, Mm(15.0), y_position, &font_bold);
    draw_underline(&current_layer, Mm(15.0), y_position - Mm(1.5), Mm(45.0));
    y_position -= Mm(10.0);

    let mittarin_loppulukema =
     app.matkamittarin_aloituslukema.parse::<f64>().unwrap_or(0.0)
     + app.ammattiajo.parse::<f64>().unwrap_or(0.0)
     + app.tuottamaton_ajo.parse::<f64>().unwrap_or(0.0) 
     + app.yksityinen_ajo.parse::<f64>().unwrap_or(0.0);
    
    let ajokilometrit_data = vec![
        ("Mittarin aloituslukema:", &app.matkamittarin_aloituslukema, "km"),
        ("Ammattiajo:", &app.ammattiajo, "km"),
        ("Tuottamaton ajo:", &app.tuottamaton_ajo, "km"),
        ("Yksityinen ajo:", &app.yksityinen_ajo, "km"),
        ("Mittarin loppulukema:", &mittarin_loppulukema.to_string(), "km"),
    ];
    
    let output_file = match File::create("daily_report.pdf") {
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

fn draw_underline(layer: &PdfLayerReference, x: Mm, y: Mm, width: Mm) {
    let line = Line {
        points: vec![(Point::new(x, y), false), (Point::new(x + width, y), false)],
        is_closed: false,
    };

    layer.add_line(line);
}