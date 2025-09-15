

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let home = rsb_downloader::home_dir(".rsb").expect("Could not determine home directory");
    println!("Home directory: {}", home.display()); 

    match rsb_downloader::download_asset(
        &home.join("example.txt"),
        vec!["https://nonexistent.wtf", "https://www.example.com"],
    ).await {
        Err(e) => {
            eprintln!("Error downloading asset: {}", e);
            std::process::exit(1);
        }
        Ok(_) => println!("Downloaded example.txt"),
    }

    Ok(())
}