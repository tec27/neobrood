use std::env;
use std::fs::File;
use std::path::PathBuf;

use directories::UserDirs;
use neobrood::create_app;

use neobrood::settings::GameSettings;

#[cfg(feature = "mimalloc")]
#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

fn main() {
    let user_dirs = UserDirs::new().expect("Couldn't get user directories!");
    let documents_dir = user_dirs
        .document_dir()
        .expect("Couldn't get Documents directory!");
    let settings_file = documents_dir
        .join("Starcraft")
        .join("neobrood-settings.json");
    // NOTE(tec27): We avoid using any tracing functions for logging here as that won't be
    // initialized until Bevy's LogPlugin is
    let settings = match File::open(settings_file) {
        Ok(mut file) => serde_json::from_reader::<_, GameSettings>(&mut file).unwrap_or_else(|e| {
            eprintln!(
                "Using default settings due to error parsing settings file: {}",
                e
            );
            GameSettings::default()
        }),
        Err(e) => {
            eprintln!(
                "Using default settings due to error reading settings file: {}",
                e
            );
            GameSettings::default()
        }
    };

    let maps = env::args()
        .skip(1)
        .flat_map(|path| {
            let mut path = PathBuf::from(path);
            if !path.is_absolute() {
                // Bevy will treat relative paths as relative to `assets/`, so we "fix" that here so
                // any relative paths are relative to the program dir
                path = env::current_dir().unwrap_or(PathBuf::from("..")).join(path);
            }

            if path.is_dir() {
                path.read_dir()
                    .expect("Couldn't read specified map directory")
                    .filter_map(|entry| {
                        let entry = entry.expect("Couldn't read directory entry");
                        let path = entry.path();
                        let extension = path.extension().map_or("".into(), |s| {
                            s.to_ascii_lowercase().to_string_lossy().to_string()
                        });
                        if extension == "scm" || extension == "scx" {
                            Some(path)
                        } else {
                            None
                        }
                    })
                    .collect()
            } else {
                vec![path]
            }
        })
        .collect::<Vec<_>>();

    let mut app = create_app(settings, maps);
    app.run();
}
