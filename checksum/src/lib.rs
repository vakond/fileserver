//! Checksum.

use std::fs::File;
use std::io::{BufReader, Read};
use std::path::Path;

/// Calculates MD5 sum of the contents of the file
/// and returns it's string representation.
pub fn md5file(filename: &Path) -> String {
    const CHUNK_SIZE: usize = 1024 * 1024;

    let file = File::open(filename).unwrap();
    let mut reader = BufReader::new(file);
    let mut buf = vec![0; CHUNK_SIZE];
    let mut sum = md5::Context::new();

    while let Ok(size) = reader.read(&mut buf) {
        if size == 0 {
            break;
        }
        if size < CHUNK_SIZE {
            // should be last chunk if any
            buf.truncate(size);
        }
        sum.consume(&buf);
    }

    let digest = sum.compute();
    format!("{:x}", digest)
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::distributions::Alphanumeric;
    use rand::{thread_rng, Rng};
    use std::env;
    use std::fs::File;
    use std::io::Write;
    use std::mem;

    #[test]
    fn it_works() {
        let random_str: String = thread_rng().sample_iter(&Alphanumeric).take(10).collect();
        let filename = env::temp_dir().join(&random_str);

        let mut file = File::create(&filename).unwrap();
        file.write("Make it work, then make it beautiful, then if you really, really have to, make it fast.".as_bytes())
            .unwrap();
        mem::drop(file);

        let sum = md5file(&filename);
        assert_eq!(sum, "201c948041f7567ee51cac5793f51e72");
    }
}
