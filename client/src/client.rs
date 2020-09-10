//! Client.

use crate::api::{file_client::FileClient, Input};
use anyhow::anyhow;
use checksum;
use std::fs::File;
use std::io::Write;
use std::mem;
use std::path::PathBuf;
use tonic::{transport::Channel, Request};

const SERVER: &str = "http://[::1]:50051";

/// Calls RPC `stop` which should shutdown the server.
pub async fn stop() -> anyhow::Result<()> {
    let channel = Channel::from_static(SERVER).connect().await?;
    let mut caller = FileClient::new(channel);

    let req = Request::new(Input::default());
    let _ = caller.stop(req).await;
    // No need to handle errors here: server must shutdown anyway

    Ok(())
}

/// Calls RPC `list` and prints all tags.
pub async fn list() -> anyhow::Result<()> {
    let channel = Channel::from_static(SERVER).connect().await?;
    let mut caller = FileClient::new(channel);

    let req = Request::new(Input::default());
    let resp = caller.list(req).await?.into_inner();
    resp.filename.iter().for_each(|f| println!("{}", f));

    Ok(())
}

/// Calls RPC `download` and gets requested file.
pub async fn download(name: &str) -> anyhow::Result<()> {
    let channel = Channel::from_static(SERVER).connect().await?;
    let mut caller = FileClient::new(channel);

    let request = Request::new(Input {
        filename: String::from(name),
    });

    let mut stream = caller.download(request).await?.into_inner();
    let filename = PathBuf::from(format!("{}.bin", name)); // arbitrary local filename
    let mut total_size = 0;
    let mut md5sum = String::default();

    let mut file = File::create(&filename)?;
    while let Some(packet) = stream.message().await? {
        if md5sum.is_empty() {
            md5sum = packet.md5sum;
        } else {
            assert_eq!(md5sum, packet.md5sum);
        }
        total_size += file.write(&packet.contents[..])?;
        print!("\r{}%", f64::round(packet.progress * 100.0) as usize);
    }
    mem::drop(file);
    println!(
        " Downloaded {} bytes into {}",
        total_size,
        filename.to_str().unwrap()
    );

    if md5sum != checksum::md5file(&filename) {
        return Err(anyhow!("downloaded with errors, try again"));
    }

    Ok(())
}
