use event_listener::{Event};
use zbus::{interface, connection::Builder, zvariant::Value, proxy, Connection, conn};
use std::ffi::OsStr;
use std::fs;
use std::env;
use std::path::{ Path, PathBuf };
use notify::{Watcher, RecommendedWatcher, RecursiveMode};

#[proxy(
	interface = "org.iolobss.diary",
	default_service = "org.ioloboss.diary",
	default_path = "/org/ioloboss/diary"
)]
trait diary {
	fn move_file(&self, path: &str) -> zbus::Result<String>;
	fn get_date_folder(&self) -> zbus::Result<String>;
	fn update_date(&self) -> zbus::Result<String>;
}

struct Diary {
	date_folder: String,
	done: Event,
}

#[interface(name = "org.ioloboss.diary")]
impl Diary {
	async fn testing(&self, name: &str) -> String {
		println!("Got {}", name);
		format!("Testing {}", name)
	}

	async fn move_file(&mut self, path: &str) -> String {
		self.date_folder = generate_date_folder_path();
		let source_path = Path::new(path);
		let file_name: &OsStr;
		match source_path.file_name() {
			None => {
				return "File Doesn't Exist".to_string()
			},
			Some(name) => {
				file_name = name;
			},
		}
		let mut desitination_buf = PathBuf::from(&self.date_folder);
		desitination_buf.push(file_name);
		let destination_path = desitination_buf.as_path();
		match fs::rename(source_path, destination_path) {
			Ok(res) => {
				"Moved File Successfully".to_string()
			},
			Err(e) => {
				dbg!(e);
				"Failed to move file".to_string()
			},
		}
	}

	#[zbus(property)]
	async fn date_folder(&self) -> String {
		self.date_folder.clone()
	}

	#[zbus(property)]
	async fn set_date_folder(&mut self, name: String) {
		self.date_folder = name;
	}

	async fn get_date_folder(&self) -> String {
		self.date_folder.clone()
	}

	async fn update_date(&mut self) -> String {
		let date = chrono::Local::now();
		let formatted = format!(".diary/{}/", date.format("%Y-%m-%d"));
		let date_formatted = Path::new(&formatted);
		let mut path: PathBuf;
		match env::home_dir() {
			Some(home_path) => {
				path = home_path;
			}
			None => {
				return "Could not find Home Directory".to_string();
			}
		}
		path.push(date_formatted);
		fs::create_dir_all(path.clone());
		match path.to_str() {
			None => return "No Path".to_string(),
			Some(date_path) => {
				self.date_folder = date_path.to_string();
			},
		}
		"Successfully updated folder".to_string()
	}
}

fn generate_date_folder_path() -> String {
		let date = chrono::Local::now();
		let formatted = format!(".diary/{}/", date.format("%Y-%m-%d"));
		let date_formatted = Path::new(&formatted);
		let mut path: PathBuf;
		path = env::home_dir().unwrap();
		path.push(date_formatted);
		fs::create_dir_all(path.clone());
		match path.to_str() {
			None => return "./".to_string(),
			Some(date_path) => {
				return date_path.to_string();
			},
		}
}

fn added_to_folder(res: notify::Result<notify::Event>) {
	let dest_path = generate_date_folder_path();
	match res {
		Ok(event) => {
			if event.kind == notify::EventKind::Access(notify::event::AccessKind::Close(notify::event::AccessMode::Write)) {
				for path in event.paths {
					let source_path = path.as_path();
					let file_name: &OsStr;
					match source_path.file_name() {
						None => {
							println!("File {:?} Doesn't Exist", path);
							return;
						},
						Some(name) => {
							file_name = name;
						},
					}
					let mut desitination_buf = PathBuf::from(&dest_path);
					desitination_buf.push(file_name);
					let destination_path = desitination_buf.as_path();
					match fs::rename(source_path, destination_path) {
						Ok(res) => {
							println!("Successfully moved file at {:?}", path);
						},
						Err(e) => {
							dbg!(e);
						},
					}
				}
			}
		},
		Err(e) => println!("Watch Error: {:?}", e),
	}
}

#[tokio::main]
async fn main() -> zbus::Result<()> {

	let argv: Vec<String> = env::args().collect();
	let argc: usize = argv.len();
	if argc != 2 {
		println!("needs 2 arguments called binary and path to folder to watch");
		return Ok(());
	}
	
	let diary_instance = Diary {
		date_folder: "test".to_string(),
		done: event_listener::Event::new()
	};
	let done_listener = diary_instance.done.listen();
	let  _connection = Builder::session()?
		.name("org.ioloboss.diary")?
		.serve_at("/org/ioloboss/diary", diary_instance)?
		.build()
		.await?;

	let mut watcher =  RecommendedWatcher::new(added_to_folder, notify::Config::default()).unwrap();
	watcher.watch(argv[1].as_ref(), RecursiveMode::Recursive).unwrap();

	std::future::pending::<()>().await;
	
	Ok(())
}
