
use std::path::PathBuf;
use std::fs;

mod lib;

fn main() {
  let client_config = dirs_next::config_dir().unwrap_or(PathBuf::from("."))
    .join("nostr")
    .join("client.toml");

  if let Err(e) = lib::ensure_file_exists(&client_config, include_str!("client.toml")) {
    println!("{}:{}: {}", std::file!(), std::line!(), e);
  }

  match fs::read(&client_config) {
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

fn main_with_data(mut config: lib::ClientConfig) {
  if config.keypair_bytes.is_none() {
    println!("No keypair_bytes found in config, generating one and printing to screen...");
    let keypair = schnorrkel::Keypair::generate();
    config.keypair = Some(keypair.clone());
    config.keypair_bytes = Some(keypair.to_bytes().to_vec());

    if let Ok(config_str) = toml::to_string(&config) {
      println!("====== config.toml ======");
      println!("{}", config_str);
      println!("");
      println!("# Copy the above into your ~/.config/nostr/client.toml file");
    }

    return;
  }

  if config.keypair.is_none() {
    // attempt to gen from byte array
    match schnorrkel::Keypair::from_bytes(&config.keypair_bytes.clone().expect("Apready checked this")) {
      Ok(keypair) => {
        config.keypair = Some(keypair);
      }
      Err(e) => {
        println!("{}:{}: {}", std::file!(), std::line!(), e);
        return;
      }
    }
  }
  
  
  
  println!("main_with_data({:?})", &config);

}


