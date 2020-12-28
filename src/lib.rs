

use std::path::{Path};

use serde::{Serialize, Deserialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct ClientConfig {
  // If a keypair is not recorded we will generate one.
  //pub keypair: Option<schnorrkel::keys::Keypair>,
  pub keypair_bytes: Option<Vec<u8>>,

  #[serde(default)]
  pub following_pubkeys_bytes: Vec<Vec<u8>>,


  #[serde(skip)]
  pub keypair: Option<schnorrkel::keys::Keypair>,

  #[serde(skip)]
  pub following_pubkeys: Vec<schnorrkel::keys::PublicKey>,

  #[serde(skip)]
  pub relays: Vec<String>,


}

#[derive(Deserialize, Serialize, Debug)]
pub struct RelayConfig {
  #[serde(default = "default_relay_port")]
  pub port: usize,

}

fn default_relay_port() -> usize {
  8080
}


pub fn ensure_file_exists<P: AsRef<Path>>(file: &P, initial_content: &str) -> std::io::Result<()> {
  let file = file.as_ref();
  if let Some(parent_dir) = file.parent() {
    ensure_dir_exists(parent_dir)?;
  }
  if !file.exists() {
    std::fs::write(file, initial_content)?;
  }
  Ok(())
}

pub fn ensure_dir_exists<P: AsRef<Path>>(directory: P) -> std::io::Result<()> {
  let directory = directory.as_ref();
  if !directory.exists() {
    std::fs::create_dir_all(directory)?;
  }
  Ok(())
}

