fn main() {
    let home = rsb_downloader::home_dir(".rsb").expect("Could not determine home directory");
    println!("Home directory: {}", home.display());

    match rsb_downloader::download_asset(
        &home.join("example.txt"),
        vec!["https://nonexistent.wtf", "https://www.example.com"],
    ) {
        Err(e) => {
            eprintln!("Error downloading asset: {}", e);
            std::process::exit(1);
        }
        Ok(_) => println!("Downloaded example.txt"),
    }
}
