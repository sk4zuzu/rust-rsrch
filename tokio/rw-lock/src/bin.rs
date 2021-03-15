use anyhow::Result;
use hex;
use nix::fcntl::{FlockArg, flock};
use sha1::{Digest, Sha1};
use std::os::unix::io::AsRawFd;
use std::path::{Path, PathBuf};
use std::thread::current;
use tempdir::TempDir;
use tokio::fs::{File, OpenOptions, rename};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::spawn;
use tokio::time::{sleep, Duration};

const BUF_SIZE: usize = 1024;

async fn open_file_ro(path: &Path) -> Result<File> {
    let file = OpenOptions::new()
        .read(true)
        .write(false)
        .open(path)
        .await?;
    Ok(file)
}

async fn open_file_wo(path: &Path) -> Result<File> {
    let file = OpenOptions::new()
        .read(false)
        .write(true)
        .truncate(true)
        .create(true)
        .open(path)
        .await?;
    Ok(file)
}

async fn write_or_read(path: PathBuf) -> Result<()> {
    log::debug!("{:?}", current());

    let mut buf = vec![0 as u8; BUF_SIZE].into_boxed_slice();
    let mut hasher = Sha1::new();

    if path.exists() {
        log::debug!("R1");
        let mut file = open_file_ro(path.as_path()).await?;
        loop {
            let n = file.read(&mut buf).await?;
            if n == 0 {
                break;
            } else {
                hasher.update(&buf[..n]);
            }
        }
        let hash = hasher.finalize();
        log::debug!("{:?}: {:?} (R1)", path.as_path(), hex::encode(hash.as_slice()));
        return Ok(())
    }

    let dir = path.parent().unwrap();
    let name = path.file_name().unwrap().to_str().unwrap();
    let tmp_name = format!("{}.{}", name, "tmp");
    let tmp_path = dir.join(tmp_name);

    if !tmp_path.exists() {
        let mut tmp_file = open_file_wo(tmp_path.as_path()).await?;
        if let Ok(()) = flock(tmp_file.as_raw_fd(), FlockArg::LockExclusiveNonblock) {
            log::debug!("W");
            for _ in 1..=256_i32 {
                let payload = b"kyk-lyk\r\n";
                tmp_file.write(&payload[..]).await?;
                hasher.update(&payload[..]);
            }
            drop(tmp_file);
            let hash = hasher.finalize();
            log::debug!("{:?}: {:?} (W)", path.as_path(), hex::encode(hash.as_slice()));
            rename(tmp_path.as_path(), path.as_path()).await?;
            return Ok(())
        }
    }

    {
        log::debug!("R2");
        let mut tmp_file = open_file_ro(tmp_path.as_path()).await?;
        while tmp_path.exists() {
            loop {
                let n = tmp_file.read(&mut buf).await?;
                if n == 0 {
                    break;
                } else {
                    hasher.update(&buf[..n]);
                }
            }
        }
        drop(tmp_file);
        let hash = hasher.finalize();
        log::debug!("{:?}: {:?} (R2)", tmp_path.as_path(), hex::encode(hash.as_slice()));
        return Ok(())
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    pretty_env_logger::init();

    let work_dir = TempDir::new("rw-lock")?;
    log::info!("Work {:?}", work_dir.path());

    let path = work_dir.path().join("1");
    log::info!("Path {:?}", path.as_path());

    {
        spawn(write_or_read(path.clone()));
        spawn(write_or_read(path.clone()));

        spawn(write_or_read(path.clone()));
        spawn(write_or_read(path.clone()));

        spawn(write_or_read(path.clone()));
        spawn(write_or_read(path.clone()));

        spawn(write_or_read(path.clone()));
        spawn(write_or_read(path.clone()));

        sleep(Duration::from_secs(4)).await;
    }

    {
        spawn(write_or_read(path.clone()));
        spawn(write_or_read(path.clone()));

        spawn(write_or_read(path.clone()));
        spawn(write_or_read(path.clone()));

        spawn(write_or_read(path.clone()));
        spawn(write_or_read(path.clone()));

        spawn(write_or_read(path.clone()));
        spawn(write_or_read(path.clone()));

        sleep(Duration::from_secs(4)).await;
    }

    {
        spawn(write_or_read(path.clone()));
        spawn(write_or_read(path.clone()));

        spawn(write_or_read(path.clone()));
        spawn(write_or_read(path.clone()));

        spawn(write_or_read(path.clone()));
        spawn(write_or_read(path.clone()));

        spawn(write_or_read(path.clone()));
        spawn(write_or_read(path.clone()));

        sleep(Duration::from_secs(4)).await;
    }

    Ok(())
}
