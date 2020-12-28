
use std::path::PathBuf;
use std::fs;

mod lib;

fn main() {
  let relay_config = dirs_next::config_dir().unwrap_or(PathBuf::from("."))
    .join("nostr")
    .join("relay.toml");
  
  if let Err(e) = lib::ensure_file_exists(&relay_config, include_str!("relay.toml")) {
    println!("{}:{}: {}", std::file!(), std::line!(), e);
  }
  
  match fs::read(&relay_config) {
    Ok(config_bytes) => {
      match toml::from_slice(&config_bytes) {
        Ok(config) => {
          main_with_data(config);
        }
        Err(e) => {
          println!("{}:{}: {}", std::file!(), std::line!(), e);
        }
      }
    }
    Err(e) => {
      println!("{}:{}: {}", std::file!(), std::line!(), e);
    }
  }
  
}

fn main_with_data(config: lib::RelayConfig) {
  
  println!("main_with_data({:?})", &config);

}
