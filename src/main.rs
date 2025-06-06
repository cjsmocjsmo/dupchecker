use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::path::Path;
use std::io::Read;
// use std::ffi::OsStr;
// use std::os::unix::ffi::OsStrExt; // Required for .as_bytes() on Unix-like systems
// use opencv::prelude::*;
// use opencv::core::{Mat, Size};
// use opencv::imgcodecs::imread;
// use opencv::imgproc::resize;
// use opencv::imgproc::COLOR_BGR2GRAY;
use walkdir::WalkDir;
// use opencv::types::VectorOfu8;


fn calculate_image_hash(image_path: &str) -> Result<String, Box<dyn std::error::Error>> {
    // Open the image file
    let mut file = File::open(image_path)?;
    let mut buffer = Vec::new();

    // Read the file's contents into the buffer
    file.read_to_end(&mut buffer)?;

    // Calculate the MD5 hash of the file's contents
    let hash = md5::compute(buffer);

    // Return the hash as a hexadecimal string
    Ok(format!("{:x}", hash))
}

// Function to calculate the MD5 hash of an image
fn find_duplicate_images(folder_path: &str) -> Result<HashMap<String, Vec<String>>, Box<dyn std::error::Error>> {
    // Check if the folder exists
    if !Path::new(folder_path).is_dir() {
        return Err(format!("Folder not found at {}", folder_path).into());
    }

    // Get a list of image paths in the folder and subfolders
    let mut image_paths: Vec<String> = Vec::new();
    for entry in WalkDir::new(folder_path).into_iter().filter_map(|e| e.ok()) {
        let path = entry.path();
        if path.is_file() {
            if let Some(extension) = path.extension() {
                let extension_str = extension.to_str().unwrap_or("").to_lowercase();
                if ["png", "jpg", "jpeg", "gif", "bmp"].contains(&extension_str.as_str()) {
                    image_paths.push(path.to_string_lossy().to_string());
                }
            }
        }
    }

    if image_paths.is_empty() {
        println!("No images found in folder: {}", folder_path);
        return Ok(HashMap::new()); // Return an empty HashMap
    }

    // Calculate the hash for each image and store it in a HashMap
    let mut image_hashes: HashMap<String, Vec<String>> = HashMap::new();
    for image_path in image_paths {
        let image_hash = calculate_image_hash(&image_path)?; // Use the ? operator
        image_hashes.entry(image_hash).or_insert_with(Vec::new).push(image_path);
    }

    // Filter out entries that are not duplicates
    let duplicate_images: HashMap<String, Vec<String>> = image_hashes
        .into_iter()
        .filter(|(_, paths)| paths.len() > 1)
        .collect();

    Ok(duplicate_images)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Get the folder path from the user
    let mut folder_path = String::new();
    println!("Enter the path to the folder containing images: ");
    std::io::stdin().read_line(&mut folder_path)?;
    folder_path = folder_path.trim().to_string();

    // Find the duplicate images
    let duplicates = find_duplicate_images(&folder_path)?;

    // Print the results
    if duplicates.is_empty() {
        println!("No duplicate images found.");
    } else {
        println!("Duplicate images found:");
        for (image_hash, image_paths) in &duplicates { // Use a reference to avoid moving
            println!("Hash: {}", image_hash);
            for image_path in image_paths {
                println!("  - {}", image_path);
            }
        }

        // Optional: Delete duplicate images (use with caution!)
        println!("Do you want to delete the duplicate images? (yes/no): ");
        let mut delete_duplicates = String::new();
        std::io::stdin().read_line(&mut delete_duplicates)?;
        delete_duplicates = delete_duplicates.trim().to_lowercase();

        if delete_duplicates == "yes" {
            for (_, image_paths) in &duplicates { // Use a reference here as well
                // Keep the first image, delete the rest
                for image_path in image_paths.iter().skip(1) {
                    // Use Path::new to convert the string to a Path
                    if let Err(e) = fs::remove_file(Path::new(image_path)) {
                        eprintln!("Error deleting {}: {}", image_path, e); // Use eprintln! for errors
                    } else {
                        println!("Deleted: {}", image_path);
                    }
                }
            }
            println!("Duplicate images deleted.");
        } else {
            println!("Duplicate images not deleted.");
        }
    }

    Ok(()) // Return Ok(()) to indicate success
}


