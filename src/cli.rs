use bevy::log::Level;

#[derive(clap::Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[arg(short, long, default_value_t=Level::INFO)]
    pub log_level: Level,
}
