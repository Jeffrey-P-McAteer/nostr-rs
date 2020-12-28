
use std::path::PathBuf;
use std::fs;
use std::env;

mod lib;

fn main() {
  let client_config = get_client_config_pathbuf();

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

fn get_client_config_pathbuf() -> PathBuf {
  dirs_next::config_dir().unwrap_or(PathBuf::from("."))
    .join("nostr")
    .join("client.toml")
}

fn main_with_data(mut config: lib::ClientConfig) {
  if config.keypair.is_none() {
    println!("No keypair found in config, generating one and printing to screen...");
    let keypair = schnorrkel::Keypair::generate();
    config.keypair = Some(keypair.clone());
    
    if let Ok(config_str) = toml::to_string(&config) {
      println!("====== config.toml ======");
      println!("{}", config_str);
      println!("");
      println!("# Copy the above into your ~/.config/nostr/client.toml file");
      println!("# Your public key is \"{}\". ", base64::encode(keypair.public.to_bytes()) );
      println!("# Run 'nostr-c print-pubkey' to print your public key at any time.");
    }

    return;
  }
  
  let args: Vec<String> = env::args().collect();

  match args.get(1).unwrap_or(&"get-updates".to_string()).as_str() {
    "help" => {
      print_help();
    }

    "gen-keypair" => {
      let keypair = schnorrkel::Keypair::generate();
      println!("{}", base64::encode(keypair.to_bytes()));
    }

    "print-pubkey" => {
      if let Some(keypair) = config.keypair.clone() {
        println!("{}", base64::encode(keypair.public.to_bytes()));
      }
      else {
        println!("No keypair found in {}!", &get_client_config_pathbuf().to_string_lossy());
      }
    }
    "print-following" => {
      for key in config.following_pubkeys {
        println!("{}", base64::encode(key.to_bytes()));
      }
    }
    "print-relays" => {
      for relay in config.relays {
        println!("{}", relay);
      }
    }
    
    "get-updates" => {
      get_updates(&config);
    }

    "publish" => {
      publish(&config, args.get(2));
    }

    unk => {
      println!("Unknown command: {}", unk);
      print_help();
    }
  }

}

fn get_updates(config: &lib::ClientConfig) {
  for relay in &config.relays {
    if let Err(e) = get_updates_single(config, &relay[..]) {
      println!("{}:{}: {}", std::file!(), std::line!(), e);
    }
  }
}

fn get_updates_single(config: &lib::ClientConfig, relay_host: &str) -> Result<(), Box<dyn std::error::Error> > {
  let event_source = sse_client::EventSource::new(format!("{}/listen_events?session=", relay_host).as_str())?;

  Ok(())
}


fn publish(config: &lib::ClientConfig, msg: Option<&String>) {
  let mut buffer = String::new();
  let msg = match msg {
    Some(msg) => msg,
    None => {
      use std::io::Read;
      // Read from stdin until EOF and use that
      let stdin = std::io::stdin();
      match stdin.lock().read_to_string(&mut buffer) {
        Ok(_num) => {},
        Err(e) => {
          println!("{}:{}: {}", std::file!(), std::line!(), e);
        }
      }
      &buffer
    }
  };

  println!("TODO publish {}", msg);

}


fn print_help() {
  println!(r#"Usage: nostr-c command [options]
Where command is one of:

  help              Print this help text

  gen-keypair       Generate a new keypair, printed in base64

  print-pubkey      Print your public key
  print-following   Print everyone you are following
  print-relays      Print the relays you are using

  get-updates       Query all relays for all public keys you follow and print results

"#);
}
