use std::fs;
use std::io::{self, Error, ErrorKind};
use std::path::PathBuf;
use std::env;


#[cfg(all(feature = "sync", feature = "async"))]
compile_error!("Features `sync` and `async` cannot be enabled at the same time.");


#[cfg(feature = "sync")]
pub fn download_asset(path: &PathBuf, urls: Vec<&str>) -> Result<(), std::io::Error> {

    if path.exists() {
        return Ok(());
    }

    if path.is_dir() {
        return Err(Error::new(
            ErrorKind::InvalidInput,
            "Provided target is a directory and not a file",
        ));
    }

    if urls.is_empty() {
        return Err(Error::new(ErrorKind::InvalidInput, "No URL provided"));
    }

    init_parent_path(path)?;

    for url in urls {
        match try_download_sync(path, url) {
            Ok(()) => return Ok(()),
            Err(e) => eprintln!("Failed to download from {}: {}", url, e),
        }
       
    }
    Err(Error::new(ErrorKind::Other, "All download attempts failed"))
   
}


#[cfg(feature = "async")]
pub async fn download_asset(path: &PathBuf, urls: Vec<&str>) -> Result<(), std::io::Error> {
    if path.exists() {
        return Ok(());
    }

    if path.is_dir() {
        return Err(Error::new(
            ErrorKind::InvalidInput,
            "Provided target is a directory and not a file",
        ));
    }

    if urls.is_empty() {
        return Err(Error::new(ErrorKind::InvalidInput, "No URL provided"));
    }

    init_parent_path(path)?;

    for url in urls {
        match try_download_async(path, url).await {
            Ok(()) => return Ok(()),
            Err(e) => eprintln!("Failed to download from {}: {}", url, e),
        }
       
    }
    Err(Error::new(ErrorKind::Other, "All download attempts failed"))
}




#[cfg(feature = "async")]
async fn try_download_async(path: &PathBuf, url: &str) -> Result<(), std::io::Error> {
    let response = reqwest::get(url).await
        .map_err(|e| Error::new(ErrorKind::NotFound, format!("Request error: {}", e)))?;
    let bytes = response
        .bytes()
        .await
        .map_err(|e| Error::new(ErrorKind::InvalidData, format!("Read bytes error: {}", e)))?;

    let mut file = fs::File::create(&path)?;
    io::copy(&mut bytes.as_ref(), &mut file)?;
    Ok(())
}

#[cfg(feature = "sync")]
fn try_download_sync(path: &PathBuf, url: &str) -> Result<(), std::io::Error> {
    let response = reqwest::blocking::get(url)
        .map_err(|e| Error::new(ErrorKind::NotFound, format!("Request error: {}", e)))?;
    let bytes = response
        .bytes()
        .map_err(|e| Error::new(ErrorKind::InvalidData, format!("Read bytes error: {}", e)))?;

    let mut file = fs::File::create(&path)?;
    io::copy(&mut bytes.as_ref(), &mut file)?;
    Ok(())
}

pub fn home_dir(subdir: &str) -> Result<PathBuf, std::io::Error> {
    let dir = env::home_dir()
        .ok_or_else(|| Error::new(ErrorKind::NotFound, "Could not determine home directory"))?;
    Ok(dir.join(subdir))
}

fn init_parent_path(path: &PathBuf) -> Result<(), std::io::Error> {
    let parent = path.parent().ok_or_else(|| {
        Error::new(
            ErrorKind::InvalidInput,
            "Provided path has no parent directory",
        )
    })?;
    if !parent.exists() {
        fs::create_dir_all(parent)?;
    }
    Ok(())
}

