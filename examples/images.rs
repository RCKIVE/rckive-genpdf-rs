use std::env;

use rckive_genpdf::Alignment;
use rckive_genpdf::Element as _;
use rckive_genpdf::{elements, fonts, style};

const FONT_DIRS: &[&str] = &[
    "/usr/share/fonts/liberation",
    "/usr/share/fonts/truetype/liberation",
];
const DEFAULT_FONT_NAME: &'static str = "LiberationSans";

const IMAGE_PATH_JPG: &'static str = "examples/images/test_image.jpg";
const IMAGE_PATH_BMP: &'static str = "examples/images/test_image.bmp";
const IMAGE_PATH_PNG: &'static str = "examples/images/test_image.png";

fn main() {
    let args: Vec<_> = env::args().skip(1).collect();
    if args.len() != 1 {
        panic!("Missing argument: output file");
    }
    let output_file = &args[0];

    let font_dir = FONT_DIRS
        .iter()
        .filter(|path| std::path::Path::new(path).exists())
        .next()
        .expect("Could not find font directory");
    let default_font =
        fonts::from_files(font_dir, DEFAULT_FONT_NAME, Some(fonts::Builtin::Helvetica))
            .expect("Failed to load the default font family");

    let mut doc = rckive_genpdf::Document::new(default_font);
    doc.set_title("rckive_genpdf Demo Document");
    doc.set_minimal_conformance();
    doc.set_line_spacing(1.25);

    let mut decorator = rckive_genpdf::SimplePageDecorator::new();
    decorator.set_margins(10);
    decorator.set_header(|page| {
        let mut layout = elements::LinearLayout::vertical();
        if page > 1 {
            layout.push(
                elements::Paragraph::new(format!("Page {}", page)).aligned(Alignment::Center),
            );
            layout.push(elements::Break::new(1));
        }
        layout.styled(style::Style::new().with_font_size(10))
    });
    doc.set_page_decorator(decorator);

    doc.push(
        elements::Paragraph::new("rckive_genpdf Image Tests")
            .aligned(Alignment::Center)
            .styled(style::Style::new().bold().with_font_size(20)),
    );
    doc.push(elements::Break::new(1.5));
    doc.push(elements::Paragraph::new(
        "You may also: override the position, dpi, and/or default line-breaks, etc. See image here =>"
    ));

    doc.push(
        elements::Image::from_path(IMAGE_PATH_JPG)
            .expect("Unable to load alt image")
            .with_position(rckive_genpdf::Position::new(170, -10)) // far over to right and down
            .with_clockwise_rotation(90.0),
    );

    // adding a break to avoid the image posted above with an "absolute image.
    doc.push(elements::Break::new(2));

    // IMAGE FILE TYPE HANDLING:
    doc.push(elements::Paragraph::new(
        "Table with image format/scaling tests:",
    ));
    let mut img_table = elements::TableLayout::new(vec![2, 2, 2, 2]);
    img_table.set_cell_decorator(elements::FrameCellDecorator::new(true, true, false));
    img_table
        .row()
        .element(elements::Text::new("Format").padded(1))
        .element(elements::Text::new("1:1").padded(1))
        .element(elements::Text::new("Half Size").padded(1))
        .element(elements::Text::new("Double Size").padded(1))
        .push()
        .expect("Invalid row");
    for (ftype, path) in vec![
        ("BMP", IMAGE_PATH_BMP),
        ("JPG", IMAGE_PATH_JPG),
        ("PNG", IMAGE_PATH_PNG),
    ] {
        let img = elements::Image::from_path(path).expect("invalid image");
        let mut row = img_table
            .row()
            .element(elements::Paragraph::new(ftype).padded(1));
        for scale in &[1.0, 0.5, 2.0] {
            row.push_element(
                img.clone()
                    .with_scale(rckive_genpdf::Scale::new(*scale, *scale))
                    .framed(style::LineStyle::new())
                    .padded(1),
            );
        }
        row.push().expect("Invalid row");
    }
    doc.push(img_table);

    doc.push(elements::Break::new(2));
    doc.push(elements::Paragraph::new(
        "Table with image rotation/offset calculation tests:",
    ));
    let mut rot_table = elements::TableLayout::new(vec![2, 2, 2, 2, 2, 2, 2]);
    rot_table.set_cell_decorator(elements::FrameCellDecorator::new(true, true, false));

    let mut heading_row: Vec<Box<dyn rckive_genpdf::Element>> =
        vec![Box::new(elements::Text::new("Rot").padded(1))];
    let mut pos_row: Vec<Box<dyn rckive_genpdf::Element>> =
        vec![Box::new(elements::Text::new("Positive").padded(1))];
    let mut neg_row: Vec<Box<dyn rckive_genpdf::Element>> =
        vec![Box::new(elements::Text::new("Negative").padded(1))];

    let img = elements::Image::from_path(IMAGE_PATH_JPG).expect("invalid image");
    for rot in &[30, 45, 90, 120, 150, 180] {
        heading_row.push(Box::new(elements::Text::new(format!("{}°", rot)).padded(1)));
        let rot = f64::from(*rot);
        pos_row.push(Box::new(
            img.clone()
                .with_clockwise_rotation(rot)
                .framed(style::LineStyle::new())
                .padded(1),
        ));
        neg_row.push(Box::new(
            img.clone()
                .with_clockwise_rotation(rot * -1.0)
                .framed(style::LineStyle::new())
                .padded(1),
        ));
    }

    rot_table.push_row(heading_row).expect("Invalid row");
    rot_table.push_row(pos_row).expect("Invalid row");
    rot_table.push_row(neg_row).expect("Invalid row");
    doc.push(rot_table);

    doc.render_to_file(output_file)
        .expect("Failed to write output file");
}
