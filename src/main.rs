mod properties;
mod path_error_handler;
use clap::{Parser}; 
use properties::{traits::*, file_props::set_file_properties, dir_props::*};
use path_error_handler::path_checker_and_separator;


#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// get metadata of the file or directory.
    #[arg(short, long)]
    metadata: Vec<String>,
}


fn main() {
    let args = Args::parse();
    let (file_paths, dir_paths, symlink_paths, metadata_na, invalid_paths) = path_checker_and_separator(&args.metadata);
    let ascii_art = r##"
                ________   __      ________  _______    _______  _____  ___    ________  _______   _______   
                |"      "\ |" \    /"       )|   __ "\  /"     "|(\"   \|"  \  /"       )/"     "| /"      \  
                (.  ___  :)||  |  (:   \___/ (. |__) :)(: ______)|.\\   \    |(:   \___/(: ______)|:        | 
                |: \   ) |||:  |   \___  \   |:  ____/  \/    |  |: \.   \\  | \___  \   \/    |  |_____/   ) 
                (| (___\ |||.  |    __/  \\  (|  /      // ___)_ |.  \    \. |  __/  \\  // ___)_  //      /  
                |:       :)/\  |\  /" \   :)/|__/ \    (:      "||    \    \ | /" \   :)(:      "||:  __   \  
                (________/(__\_|_)(_______/(_______)    \_______) \___|\____\)(_______/  \_______)|__|  \___) 

                                  An advance all in one file and folder management tool.
    "##;
    println!("{}", ascii_art);
    if file_paths.len() > 0 {
        println!("\nProperties of the given files are as follows: ");
        let file_properties = set_file_properties(&file_paths);
        for (num, property) in file_properties.iter().enumerate(){
            println!("{}. {} -", num+1, property.location().expect("Err!! Retriving Location.").join(property.name().expect("Err!! Retriving file name.")).to_str().expect("Err!! While converting path to string."));
            println!("Name: {}", property.name().expect("Err!! Retriving file name."));
            println!("Extension(Type of file): {}", property.extension().expect("Err!! Can't retrive extension of file."));
            println!("Location: {}", property.location().expect("Err!! Retriving Location.").to_str().expect("Err!! While converting path to string."));
            if property.size() < 1_000.0 {
                println!("Size: {:.2} KB", property.size());
            }
            else if property.size() >= 1_000.0 && property.size() <= 1_000_000.0 {
                println!("Size: {:.2} MB", property.size() / 1024.0 as f64);
            }
            else if property.size() >= 1_000_000.0 && property.size() <= 1_000_000_000.0 {
                println!("Size: {:.2} GB", property.size() / (1024.0 * 1024.0) as f64);
            }
            else if property.size() >= 1000_000_000.0 && property.size() <= 1_000_000_000_000.0 {
                println!("Size: {:.2} TB", property.size() / (1024.0 * 1024.0 * 1024.0) as f64);
            }
            else if property.size() >= 1000_000_000_000.0 {
                println!("Size: {:.2} PB", property.size() / (1024.0 * 1024.0 * 1024.0 * 1024.0) as f64);
            }
            else {
                println!("Size: Out of Range of this program!!");
            }
            println!("Created: {}", property.date_created().expect("Err!! Can't retrive date & time of creation of file."));
            println!("Modified: {}", property.date_modified().expect("Err!! Can't retrive date & time of last modification of file."));
            println!("Accessed: {}", property.date_accessed().expect("Err!! Can't retrive date & time of last access of file."));
            println!("Permissions: {:?}", property.access_configs());
            // println!("Hash(SHA-256): {}", property.hash().expect("Unable to calculate hash of the given file."));
            println!();
        }
    }
    if dir_paths.len() > 0 {
        println!("\nProperties of the given directories are as follows: ");
        let directory_properties = set_directory_properties(&dir_paths, true);
        for (num, property) in directory_properties.iter().enumerate(){
            println!("{}. {} -", num+1, property.location().expect("Err!! Retriving Location.").join(property.name().expect("Err!! Retriving directory name.")).to_str().expect("Err!! While converting path to string."));
            println!("Name: {}", property.name().expect("Err!! Retriving directory name."));
            println!("Type: {}", property.extension().expect("Err!! Retriving extension metadata of directory."));
            println!("Location: {}", property.location().expect("Err!! Retriving Location.").to_str().expect("Err!! While converting path to string."));
            if property.size() < 1_000.0 {
                println!("Size: {:.2} KB", property.size());
            }
            else if property.size() >= 1_000.0 && property.size() <= 1_000_000.0 {
                println!("Size: {:.2} MB", property.size() / 1024.0 as f64);
            }
            else if property.size() >= 1_000_000.0 && property.size() <= 1_000_000_000.0 {
                println!("Size: {:.2} GB", property.size() / (1024.0 * 1024.0) as f64);
            }
            else if property.size() >= 1000_000_000.0 && property.size() <= 1_000_000_000_000.0 {
                println!("Size: {:.2} TB", property.size() / (1024.0 * 1024.0 * 1024.0) as f64);
            }
            else if property.size() >= 1000_000_000_000.0 {
                println!("Size: {:.2} PB", property.size() / (1024.0 * 1024.0 * 1024.0 * 1024.0) as f64);
            }
            else {
                println!("Size: Out of Range of this program!!");
            }
            println!("Created: {}", property.date_created().expect("Err!! Can't retrive date & time of creation of directory."));
            println!("Modified: {}", property.date_modified().expect("Err!! Can't retrive date & time of last modification of directory."));
            println!("Accessed: {}", property.date_accessed().expect("Err!! Can't retrive date & time of last access of directory."));
            println!("Permissions: {:?}", property.access_configs());
            println!();
        }
    }
    if symlink_paths.len() > 0 {
    
    }
    if metadata_na.len() > 0 {
        println!("\nCan't read the metadata of the following paths -");
        println!("{:?}", metadata_na);
    }
    if invalid_paths.len() > 0 {
        println!("\nFollowing paths are invalid or not found -");
        println!("{:?}", invalid_paths);
    }
}
