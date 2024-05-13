use pdfium_render::prelude::*;
const FILE1:&str = concat!(env!("CARGO_MANIFEST_DIR"), "pdf-test.pdf");
const FILE2: &str = concat!(env!("CARGO_MANIFEST_DIR"), "pdf-test.pdf");
const OUTFILE: &str = concat!(env!("CARGO_MANIFEST_DIR"), "tiletest.pdf");

fn main() -> Result<(), PdfiumError> {
    // For general comments about pdfium-render and binding to Pdfium, see export.rs.

    let pdfium = Pdfium::new(Pdfium::bind_to_statically_linked_library().unwrap());

    // Joins several existing test PDFs together into a new in-memory document, then tiles
    // the pages in that document into a new file.

    // Create a new blank document...

    let mut document = pdfium.create_new_pdf()?;

    // ... append the pages from three test files...

    let pages = document.pages_mut();
    println!("{FILE1}");

    let bytes1 = std::fs::read(&FILE1).unwrap();

    pages.append(&pdfium.load_pdf_from_file(
        FILE1,
        None,
    )?)?;
    pages.append(&pdfium.load_pdf_from_file(
        FILE2,
        None,
    )?)?;

    // ... and tile the pages into a new A3 landscape document,
    // saving the tiled document to a file.

    pages
        .tile_into_new_document(1, 2, PdfPagePaperSize::a3().landscape())?
        .save_to_file(OUTFILE)?;

    Ok(())
}
