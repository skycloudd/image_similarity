use clap::{Parser, Subcommand};
use image;
use image_hasher::{HashAlg, HasherConfig, ImageHash};
use std::error::Error;
use std::process::exit;

#[derive(Parser)]
#[clap(version)]
#[clap(propagate_version = true)]
struct Args {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Compare {
        first: String,
        second: String,

        #[clap(short, long)]
        percentage: bool,

        #[clap(short, long)]
        algorithm: Option<String>,
    },
    Hash {
        path: String,

        #[clap(short, long)]
        algorithm: Option<String>,
    },
}

fn main() {
    let args = Args::parse();

    match run(args) {
        Ok(_) => exit(0),
        Err(e) => {
            eprintln!("error: {}", e);
            exit(1)
        }
    }
}

fn run(args: Args) -> Result<(), Box<dyn Error>> {
    match args.command {
        Commands::Compare {
            first,
            second,
            percentage,
            algorithm,
        } => {
            let hash_alg = match algorithm {
                Some(s) => match s.as_str() {
                    "mean" | "m" => HashAlg::Mean,
                    "gradient" | "g" => HashAlg::Gradient,
                    "vertgradient" | "v" => HashAlg::VertGradient,
                    "doublegradient" | "d" => HashAlg::DoubleGradient,
                    "blockhash" | "b" => HashAlg::Blockhash,
                    _ => return Err("unknown hash algorithm".into()),
                },
                None => HashAlg::Gradient,
            };

            let similarity = compare(&first, &second, hash_alg)?;

            match percentage {
                true => println!("{}%", similarity * 100.0),
                false => println!("{}", similarity),
            }
        }
        Commands::Hash { path, algorithm } => {
            let hash_alg = match algorithm {
                Some(s) => match s.as_str() {
                    "mean" | "m" => HashAlg::Mean,
                    "gradient" | "g" => HashAlg::Gradient,
                    "vertgradient" | "v" => HashAlg::VertGradient,
                    "doublegradient" | "d" => HashAlg::DoubleGradient,
                    "blockhash" | "b" => HashAlg::Blockhash,
                    _ => return Err("unknown hash algorithm".into()),
                },
                None => HashAlg::Gradient,
            };

            let hash = hash_image(&path, hash_alg)?;

            println!("{}", hash.to_base64())
        }
    }

    Ok(())
}

fn compare(first: &str, second: &str, hash_alg: HashAlg) -> Result<f32, Box<dyn Error>> {
    let image1 = image::open(first)?;
    let image2 = image::open(second)?;

    let hasher = HasherConfig::new().hash_alg(hash_alg).to_hasher();

    let hash1 = hasher.hash_image(&image1);
    let hash2 = hasher.hash_image(&image2);

    Ok(
        (hash1.as_bytes().len() * 8 - hash1.dist(&hash2) as usize) as f32
            / (hash1.as_bytes().len() * 8) as f32,
    )
}

fn hash_image(path: &str, hash_alg: HashAlg) -> Result<ImageHash, Box<dyn Error>> {
    let the_image = image::open(path)?;

    let hasher = HasherConfig::new().hash_alg(hash_alg).to_hasher();

    let hash = hasher.hash_image(&the_image);

    Ok(hash)
}
