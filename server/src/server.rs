// server

use crate::api::{file_server, DownloadOutput, Input, ListOutput};
use crate::config::Config;
use std::fs::{self, File};
use std::io::Read;
use tokio::sync::mpsc;
use tonic::{transport::Server, Request, Response, Status};

/// Starts the gRPC server.
pub async fn serve(cfg: Config) -> anyhow::Result<()> {
    let address = cfg.address();
    let service = FileImpl::new(cfg);

    println!("Ready");
    Server::builder()
        .add_service(file_server::FileServer::new(service))
        .serve(address)
        .await?;

    Ok(())
}

type ListResult = Result<Response<ListOutput>, Status>;
type DownloadStream = mpsc::Receiver<Result<DownloadOutput, Status>>;
type DownloadResult = Result<Response<DownloadStream>, Status>;

#[derive(Debug)]
struct FileImpl {
    cfg: Config,
}

impl FileImpl {
    fn new(cfg: Config) -> Self {
        FileImpl { cfg }
    }
}

#[tonic::async_trait]
impl file_server::File for FileImpl {
    type DownloadStream = DownloadStream;

    /// Handles command "client list".
    async fn list(&self, _: Request<Input>) -> ListResult {
        Ok(Response::new(ListOutput {
            filename: self.cfg.tags(),
        }))
    }

    /// Handles command "client download <NAME>".
    async fn download(&self, request: Request<Input>) -> DownloadResult {
        const CHUNK_SIZE: usize = 1024;
        let tag = &request.get_ref().filename;
        let filename = self.cfg.filename(&tag);
        let md5sum = self.cfg.md5sum(&tag);

        // Determine size of file and open a channel
        let metadata = fs::metadata(&filename).unwrap();
        let len = metadata.len() as usize;
        let num_of_chunks = len / CHUNK_SIZE + 1;
        let (mut tx, rx) = mpsc::channel(num_of_chunks);

        tokio::spawn(async move {
            let mut index: i64 = 0;
            let mut current_size: usize = 0;
            let mut progress: f64;
            let mut file = File::open(filename)?;
            for _ in 0..num_of_chunks {
                let mut contents = vec![0; CHUNK_SIZE];
                let size = file.read(&mut contents)?;
                contents.truncate(size);
                current_size += size;
                progress = current_size as f64 / len as f64;
                let packet = DownloadOutput {
                    index,
                    progress,
                    contents,
                    md5sum: md5sum.clone(), // Transmit checksum of the whole file in each packet for simplicity
                };
                tx.send(Ok(packet)).await?;
                index += 1;
            }
            // explicit type annotation to help compiler derive return type of async block
            Ok::<(), anyhow::Error>(())
        });

        Ok(Response::new(rx))
    }
}
