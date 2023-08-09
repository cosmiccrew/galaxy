mod indexer;
mod mapper;

use std::path::PathBuf;

use anyhow::Result;

use clap::Parser;
use clap::Subcommand;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// For creating an index of what names map to which sprite sheet tiles.
    Indexer {
        /// Path of the sprite_sheet file
        input_file: PathBuf,

        /// Where the  ".spritesheet.ron" file should be output to - defaults to the same path as the file:
        /// e.g. a file at "assets/ui/buttons.png" would become "assets/ui/buttons.spritesheet.ron"
        #[arg(short, long, value_name = "FILE")]
        output_file: Option<PathBuf>,
    },
    Mapper {},
}

#[show_image::main]
fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Indexer {
            input_file,
            output_file,
        } => {
            indexer::indexer(input_file, output_file)?;
        }
        Commands::Mapper {} => todo!(),
    }

    Ok(())
}
