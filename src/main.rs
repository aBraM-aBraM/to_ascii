use std::io::Write;

/// Converts a picture to a grayscale ascii
#[derive(clap::Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Source image path
    #[arg(short, long)]
    src: String,

    /// Path to save the ascii image to
    #[arg(short, long)]
    dst: String,
}

const GRAYSCALE: &str = "$@B%8&WM#*oahkbdpqwmZO0QLCJUYXzcvunxrjft/\\|()1{}[]?-_+~<>i!lI;:,\"^`'.";
fn main() -> std::io::Result<()> {
    let args = <Args as clap::Parser>::parse();

    let img = image::io::Reader::open(&args.src)?
        .with_guessed_format()?
        .decode()
        .expect("Failed to decode image");
    let mut img = img.grayscale();
    let img = img.as_mut_luma8().unwrap();

    let mut ascii_output = std::fs::File::create(args.dst).expect("destination already exists!");

    for x in 0..img.width() {
        for y in 0..img.height() {
            let pixel = img.get_pixel(x, y);
            let char =
                GRAYSCALE.as_bytes()[usize::from(pixel.0[0]) * GRAYSCALE.len() / 255] as char;
            ascii_output
                .write_all(format!("{}", char).as_str().as_bytes())
                .unwrap();
        }
        ascii_output.write_all(b"\n").unwrap();
    }

    Ok(())
}
