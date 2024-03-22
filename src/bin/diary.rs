use std::env;
use std::error::Error;
use std::path::{ Path, PathBuf };
use tokio;

use zbus::{zvariant::Value, proxy, Connection, conn};

#[proxy(
	interface = "org.ioloboss.diary",
	default_service = "org.ioloboss.diary",
	default_path = "/org/ioloboss/diary"
)]
trait diary {
	fn move_file(&self, path: &str) -> zbus::Result<String>;
	fn get_date_folder(&self) -> zbus::Result<String>;
	fn update_date(&self) -> zbus::Result<String>;
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
	let argv: Vec<String> = env::args().collect();
	let argc: usize = argv.len();

	if (argc <= 1) || (argc >= 4) {
		println!("Wrong Ammount of arguments arguments");
		return Ok(());
	}

	let connection = Connection::session().await?;

	let proxy = diaryProxy::new(&connection).await?;

	let command: &str = &argv[1];

	match command {
		"add" => {
			let mut working_dir = env::current_dir()?;
			working_dir.push(Path::new(&argv[2]));
			let reply = proxy.move_file(working_dir.to_str().unwrap()).await?;
			println!("{}", reply);
		},
		"get" => {
			let reply = proxy.get_date_folder().await?;
			println!("{}", reply);
		},
		"update" => {
			let reply = proxy.update_date().await?;
			println!("{}", reply);
		},
		_ => {
			println!("{} is not a valid command", &argv[1]);
		},
	}
	
	Ok(())
}
