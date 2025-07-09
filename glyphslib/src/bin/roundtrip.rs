use std::path::Path;

fn main() {
    let input = std::env::args().nth(1).expect("Please provide a file path");
    let output = std::env::args()
        .nth(2)
        .expect("Please provide an output file path");
    let start = std::time::Instant::now();
    let font = glyphslib::Font::load(Path::new(&input)).expect("Failed to read font file");
    let elapsed = start.elapsed();
    let start = std::time::Instant::now();
    println!("Loaded font in: {elapsed:?}");
    font.save(Path::new(&output))
        .expect("Failed to save font file");
    let elapsed = start.elapsed();
    println!("Saved font in: {elapsed:?}");
}
