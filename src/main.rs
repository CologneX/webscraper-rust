use std::fs::File;
use std::io::prelude::*;
fn main() {
    scrap_website();
}

fn scrap_website() {
    let response = reqwest::blocking::get("https://opc-ap1.invisiq.com/product/8d69b89e-b27d-4092-be87-82a8b746b0a8")
        .unwrap()
        .text()
        .unwrap();

    let mut file = File::create("scraped.txt").expect("Error encountered while creating file!");

    file.write_all(response.as_bytes())
        .expect("Error encountered while writing to file!");
}
