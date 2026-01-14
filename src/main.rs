mod app;

use app::App;
use clap::Parser;

fn main() {
	let app: App = App::parse();

	println!("Hello, {}!", app.name);
}