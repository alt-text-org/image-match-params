use clap::{Parser, Subcommand};
use std::path::PathBuf;
use std::fs::File;
use std::io::{Error, ErrorKind};

mod stats;
mod draw;
use image_match::image::get_file_signature;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands
}

#[derive(Subcommand, Debug)]
enum Commands {
    GenSignature {
        path: PathBuf
    },
    SimilarityGrid {
        path: PathBuf
    },
    DrawDebug {
        path: PathBuf,
        out_path: PathBuf
    },
    Stats {
        path: PathBuf
    }
}

fn main() -> std::io::Result<()> {
    let args = Cli::parse();
    match args.command {
        Commands::Stats { path } => {
            stats::check_match_percentages(&path);
            Ok(())
        },
        Commands::SimilarityGrid { path } => {
            let (pics, grid) = stats::compare_matrix(&path);

            let pic_index = File::create("index.csv")?;
            let mut index_writer = csv::Writer::from_writer(pic_index);
            index_writer.write_record(&["path","signature"])?;
            let mut paths = Vec::with_capacity(pics.len());
            for (path, sig) in pics {
                index_writer.write_record(&[path.clone(),hex::encode(&sig.into_iter().map(|v| v as u8).collect::<Vec<u8>>())])?;
                paths.push(path)
            };

            let pic_grid = File::create("grid.csv")?;
            let mut grid_writer = csv::Writer::from_writer(pic_grid);
            grid_writer.write_record(&["left","left_path","right","right_path","distance"])?;
            let mut lpaths = paths.iter();
            for dists in grid {
                let left = lpaths.next().ok_or(Error::new(ErrorKind::Other, "Ran out of left paths!"))?;
                let mut rpaths = paths.iter();
                for dist in dists {
                    let right = rpaths.next().ok_or(Error::new(ErrorKind::Other, "Ran out of right paths!"))?;
                    let lparts: Vec<_> = left.rsplitn(2, "/").collect();
                    let rparts: Vec<_> = right.rsplitn(2, "/").collect();
                    grid_writer.write_record(&[lparts[0], lparts[1], rparts[0], rparts[1], &dist.to_string()])?;
                }
            }
            Ok(())
        },
        Commands::DrawDebug { path, out_path } => {
            draw::draw_debug(&path, &out_path);
            Ok(())
        },
        Commands::GenSignature { path } => {
            println!("{:x?}", get_file_signature(&path));
            Ok(())
        }
    }
}
