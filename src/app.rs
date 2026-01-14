use clap::Parser;

#[derive(Parser)]
#[command(version, about, long_about)]
pub struct App {
	#[arg(short, long)]
	pub name: String
}