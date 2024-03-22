use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Path to the audio file to transcribe
    #[arg(short, long)]
    file: String,
}

fn main() {
    let args = Args::parse();

    println!("Transcribing File: {}", args.file);
}
