use clap::{Args, Parser};

#[derive(Parser, Debug)]
#[command(
    author = "killersnake3",
    version = "1.0",
    about = "A command line tool to initialize a template for your Hytale Plugin"
)]
pub struct App {
    #[arg(short, long)]
    pub output_dir: String,
    #[arg(short, long)]
    pub group: String,
    #[arg(short, long, default_value_t = String::from("ExamplePlugin"))]
    pub name: String,
    #[arg(short, long, default_value_t = String::from(""))]
    pub description: String,
    #[arg(short, long, default_value_t = String::from(""))]
    pub website: String,
    #[clap(flatten)]
    pub authors: Author,
}

#[derive(Args, Debug)]
pub struct Author {
    #[arg(long, default_value_t = String::from(""))]
    pub author_name: String,
    #[arg(long, default_value_t = String::from(""))]
    pub author_email: String,
    #[arg(long, default_value_t = String::from(""))]
    pub author_url: String,
}
