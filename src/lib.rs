

use std::path::{Path};

use serde::{Serialize, Deserialize, Serializer, Deserializer};

#[derive(Deserialize, Serialize, Debug)]
pub struct ClientConfig {

  #[serde(serialize_with = "keypair_as_base64", deserialize_with = "keypair_from_base64")]
  pub keypair: Option<schnorrkel::keys::Keypair>,

  #[serde(serialize_with = "publickeys_as_base64", deserialize_with = "publickeys_from_base64")]
  pub following_pubkeys: Vec<schnorrkel::keys::PublicKey>,

  #[serde()]
  pub relays: Vec<String>,

}

fn keypair_as_base64<S>(keypair: &Option<schnorrkel::keys::Keypair>, serializer: S) -> Result<S::Ok, S::Error>
  where S: Serializer
{
  if let Some(keypair) = keypair {
    return Ok( serializer.serialize_str(&base64::encode(&keypair.to_bytes()[..]))? );
  }
  return Ok( serializer.serialize_str("None")? );
}

fn keypair_from_base64<'de, D>(deserializer: D) -> Result<Option<schnorrkel::keys::Keypair>, D::Error>
  where D: Deserializer<'de>
{
  let str_base64: &str = Deserialize::deserialize(deserializer)?;
  if let Ok(bytes) = base64::decode(str_base64) {
    if let Ok(keypair) = schnorrkel::keys::Keypair::from_bytes(&bytes[..]) {
      return Ok(Some(keypair));
    }
  }
  Ok(None)
}


fn publickeys_as_base64<S>(pubkeys: &Vec<schnorrkel::keys::PublicKey>, serializer: S) -> Result<S::Ok, S::Error>
  where S: Serializer
{
  use serde::ser::SerializeSeq;
  let mut seq = serializer.serialize_seq(Some(pubkeys.len()))?;
  for key in pubkeys {
    let key_b64_s = base64::encode(&key.to_bytes()[..]);
    seq.serialize_element(&key_b64_s[..])?;
  }
  seq.end()
}

fn publickeys_from_base64<'de, D>(deserializer: D) -> Result<Vec<schnorrkel::keys::PublicKey>, D::Error>
  where D: Deserializer<'de>
{
  let mut pubkeys = vec![];
  let str_base64: Vec<String> = Deserialize::deserialize(deserializer)?;
  for s in str_base64 {
    // assume base64 encoding first
    if let Ok(bytes) = base64::decode(&s[..]) {
      if let Ok(pubkey) = schnorrkel::keys::PublicKey::from_bytes(&bytes[..]) {
        pubkeys.push(pubkey);
        continue;
      }
    }
    
    // Try hex
    if let Ok(bytes) = hex::decode(&s[..]) {
      
      // TODO debug why the public keys from nostr-client.netlify.app do not parse:
      // https://github.com/w3f/schnorrkel/blob/8fa2ad3e9fbf0b652c724df6a87a4b3c5500f759/src/points.rs#L93
      // https://docs.rs/curve25519-dalek/2.0.0/src/curve25519_dalek/ristretto.rs.html#212
      println!("res={:?}", schnorrkel::keys::PublicKey::from_bytes(&bytes[..]));

      if let Ok(pubkey) = schnorrkel::keys::PublicKey::from_bytes(&bytes[..]) {
        pubkeys.push(pubkey);
        continue;
      }
    }

    eprintln!("Unknown public key encoding for {}", &s);


  }
  Ok(pubkeys)
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

