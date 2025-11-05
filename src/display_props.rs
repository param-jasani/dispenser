use crate::{
    path_error_handler::unwrapper,
    properties::{dir_props::*, file_props::*, traits::*},
};

fn display_props<T>(num: usize, property: &T)
where
    T: Info + Permission + Dates,
{
    println!("{}. {} -", num + 1, unwrapper(property, "Path"));
    println!("Name: {}", unwrapper(property, "Name"));
    println!("Type: {}", unwrapper(property, "Type"));
    println!("Location: {}", unwrapper(property, "Location"));
    let size = property.size().0;
    match size {
        0.0..1_000.0 => {
            println!("Size: {:.2} KB", size);
        }
        1_000.0..1_000_000.0 => {
            println!("Size: {:.2} MB", size / 1024.0 as f64);
        }
        1_000_000.0..1_000_000_000.0 => {
            println!("Size: {:.2} GB", size / (1024.0 * 1024.0) as f64);
        }
        1_000_000_000.0..1_000_000_000_000.0 => {
            println!("Size: {:.2} TB", size / (1024.0 * 1024.0 * 1024.0) as f64);
        }
        1_000_000_000_000.0.. => {
            println!(
                "Size: {:.2} PB",
                size / (1024.0 * 1024.0 * 1024.0 * 1024.0) as f64
            );
        }
        _ => {
            println!("Size: Out of Range of this program!!");
        }
    };
    println!("Created: {}", unwrapper(property, "Date Created"));
    println!("Modified: {}", unwrapper(property, "Date Modified"));
    println!("Accessed: {}", unwrapper(property, "Date Accessed"));
    println!("Permissions: {:?}", property.access_configs());
    if unwrapper(property, "Type") == "Directory".to_string() {
        println!(
            "Contains: {} Files, {} Folders",
            property.size().1,
            property.size().2
        );
    }
    println!();
}

fn display_iterator<T>(paths: &[T])
where
    T: Info + Permission + Dates,
{
    for (num, property) in paths.iter().enumerate() {
        display_props(num, property);
    }
}

pub fn display_file_properties(file_paths: &Vec<std::path::PathBuf>) {
    println!("\nProperties of the given file(s) are as follows: ");
    let file_properties = set_file_properties(&file_paths);
    display_iterator(&file_properties);
}

pub fn display_directory_properties(dir_paths: &Vec<std::path::PathBuf>, depth: u32) {
    println!("\nProperties of the given directorie(s) are as follows: ");
    let dir_properties = set_directory_properties(&dir_paths, depth);
    display_iterator(&dir_properties);
    if depth > 0 {
        display_childs(&dir_properties);
    }
}

pub fn display_file_checksum(file_paths: &Vec<std::path::PathBuf>) {
    println!("\nChecksum of the given file(s) are as follows: ");
    let file_properties = set_file_properties(&file_paths);
    for property in file_properties {
        println!(
            "{} => {}",
            unwrapper(&property, "Name"),
            property.hash().expect("Err!! Retrieving hash of file.")
        );
    }
}

fn display_childs(dir_properties: &Vec<DirectoryProperties>) {
    for property in dir_properties {
        if property.child_flag() {
            println!(
                "\nProperties of Children item(s) of {} folder are as follows: ",
                unwrapper(property, "Path")
            );
            match property.retrive_child_files() {
                Some(files) => {
                    if property.size().1 > 0 {
                        println!("Children file(s) are -");
                        display_iterator(&files)
                    }
                }
                None => {}
            }
            match property.retrive_sub_folders() {
                Some(folders) => {
                    if property.size().2 > 0 {
                        println!("Sub folder(s) are -");
                        display_iterator(&folders);
                        display_childs(&folders);
                    }
                }
                None => {}
            }
        }
    }
}
