// config

use anyhow::anyhow;
use checksum;
use std::collections::HashSet;
use std::default::Default;
use std::io::{self, Write};
use std::net::SocketAddr;
use std::path::PathBuf;
use std::str;

pub const APP: &str = "fileserver";
const FILENAME: &str = "fileserver.conf";

/// Initializes configuration.
pub fn init() -> anyhow::Result<()> {
    let filename = PathBuf::from(FILENAME);

    if filename.exists() {
        return Err(anyhow!("config already created: {}", FILENAME));
    }

    let _: Config = confy::load_path(filename)?;
    println!("Created {}", FILENAME);

    Ok(())
}

/// Loads configuration from file.
pub fn load() -> anyhow::Result<Config> {
    let filename = PathBuf::from(FILENAME);

    if !filename.exists() {
        return Err(anyhow!("file {} already exists", FILENAME));
    }

    let cfg = confy::load_path(filename)?;
    println!("Checking...");
    check(cfg)
}

/// Checks configuration.
fn check(cfg: Config) -> anyhow::Result<Config> {
    let mut tags = HashSet::<String>::new();
    let mut files = HashSet::<PathBuf>::new();

    for i in &cfg.items {
        print!("{} ", &i.tag);
        io::stdout().flush().unwrap();

        if tags.contains(&i.tag) {
            return Err(anyhow!("duplicated tag {}", i.tag));
        }
        tags.insert(i.tag.clone());

        if files.contains(&i.filename) {
            return Err(anyhow!("duplicated file {}", i.filename.to_str().unwrap()));
        }
        files.insert(i.filename.clone());

        if !i.filename.exists() {
            return Err(anyhow!(
                "file does not exist: {}",
                i.filename.to_str().unwrap()
            ));
        }

        let sum = checksum::md5file(&i.filename);
        if sum != i.md5sum {
            return Err(anyhow!(
                "file {} has wrong checksum in config, should be {}",
                i.filename.to_str().unwrap(),
                sum
            ));
        }

        println!("[ok]");
    }

    Ok(cfg)
}

/// Represents configuration.
#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    port: u16,
    items: Vec<Item>,
}

/// Represents one item of the list of items.
#[derive(Debug, Serialize, Deserialize)]
struct Item {
    tag: String,
    filename: PathBuf,
    md5sum: String,
}

/// Methods of `Config`.
impl Config {
    pub fn address(&self) -> SocketAddr {
        let addr = format!("[::1]:{}", self.port);
        addr.parse().unwrap()
    }

    pub fn tags(&self) -> Vec<String> {
        self.items.iter().map(|i| i.tag.clone()).collect()
    }

    pub fn filename(&self, tag: &str) -> PathBuf {
        self.items
            .iter()
            .find(|i| i.tag == tag)
            .unwrap()
            .filename
            .clone()
    }

    pub fn md5sum(&self, tag: &str) -> String {
        self.items
            .iter()
            .find(|i| i.tag == tag)
            .unwrap()
            .md5sum
            .clone()
    }
}

/// `Config` implements `Default`.
impl Default for Config {
    fn default() -> Self {
        Self {
            port: 50051,
            items: vec![Item::default()],
        }
    }
}

/// `Item` implements `Default`.
impl Default for Item {
    fn default() -> Self {
        Self {
            tag: String::from("TAG"),
            filename: PathBuf::from("/etc/passwd"),
            md5sum: String::from("garbage"),
        }
    }
}
