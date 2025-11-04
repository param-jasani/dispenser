use crate::properties::{dir_props::*, file_props::*, traits::*};

fn display_props<T>(num: usize, property: &T)
where   
    T: Info + Permission + Dates
{
        println!("{}. {} -", num+1, property.location().expect("Err!! Retriving Location.").join(property.name().expect("Err!! Retriving name.")).to_str().expect("Err!! While converting path to string."));
        println!("Name: {}", property.name().expect("Err!! Retriving name."));
        println!("Type: {}", property.extension().expect("Err!! Can't retrive extension."));
        println!("Location: {}", property.location().expect("Err!! Retriving Location.").to_str().expect("Err!! While converting path to string."));
        let size = property.size().0;
        if size < 1_000.0 {
            println!("Size: {:.2} KB", size);
        }
        else if size >= 1_000.0 && size <= 1_000_000.0 {
            println!("Size: {:.2} MB", size / 1024.0 as f64);
        }
        else if size >= 1_000_000.0 && size <= 1_000_000_000.0 {
            println!("Size: {:.2} GB", size / (1024.0 * 1024.0) as f64);
        }
        else if size >= 1000_000_000.0 && size <= 1_000_000_000_000.0 {
            println!("Size: {:.2} TB", size / (1024.0 * 1024.0 * 1024.0) as f64);
        }
        else if size >= 1000_000_000_000.0 {
            println!("Size: {:.2} PB", size / (1024.0 * 1024.0 * 1024.0 * 1024.0) as f64);
        }
        else {
            println!("Size: Out of Range of this program!!");
        }
        println!("Created: {}", property.date_created().expect("Err!! Can't retrive date & time of creation."));
        println!("Modified: {}", property.date_modified().expect("Err!! Can't retrive date & time of last modification."));
        println!("Accessed: {}", property.date_accessed().expect("Err!! Can't retrive date & time of last access."));
        println!("Permissions: {:?}", property.access_configs());
        if property.extension().expect("Err!! Can't retrive extension.") == "Directory".to_string(){
            println!("Contains: {} Files, {} Folders", property.size().1, property.size().2);
        }
        println!();
        
}

fn display_iterator<T>(paths: &[T])
where T: Info + Permission + Dates
{
    for (num, property) in paths.iter().enumerate(){
        display_props(num, property);
    }
}

pub fn display_file_properties(file_paths: &Vec<std::path::PathBuf>){
    println!("\nProperties of the given file(s) are as follows: ");
    let file_properties = set_file_properties(&file_paths);
    display_iterator(&file_properties);
}

pub fn display_directory_properties(dir_paths: &Vec<std::path::PathBuf>, depth: u32){
    println!("\nProperties of the given directorie(s) are as follows: ");
    let dir_properties = set_directory_properties(&dir_paths, depth);
    display_iterator(&dir_properties);
    if depth > 0{
        display_childs(&dir_properties);
    }
}

pub fn display_file_checksum(file_paths: &Vec<std::path::PathBuf>){
    println!("\nChecksum of the given file(s) are as follows: ");
    let file_properties = set_file_properties(&file_paths);
    for property in file_properties{
        println!("{} => {}", property.name().expect("Err!! Retriving file name."), property.hash().expect("Err!! Retriving hash of file."));
    }
}

fn display_childs(dir_properties: &Vec<DirectoryProperties>){
    for property in dir_properties{
        if property.child_flag(){
            println!("\nProperties of Children item(s) of {} folder are as follows: ", property.location().expect("Err!! Retriving Location.").join(property.name().expect("Err!! Retriving file name.")).to_str().expect("Err!! While converting path to string."));
            match property.retrive_child_files() {
                Some(files) => {
                    if property.size().1 > 0{
                        println!("Children file(s) are -");
                        display_iterator(&files)
                    }
                },
                None => {},
            }
            match property.retrive_sub_folders() {
                Some(folders) => {
                    if property.size().2 > 0{
                        println!("Sub folder(s) are -");
                        display_iterator(&folders);
                        display_childs(&folders);
                    }
                },
                None => {},
            }   
        }
    }
}