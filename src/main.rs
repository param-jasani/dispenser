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
    if file_paths.len() > 0 {
        let file_properties = set_file_properties(&file_paths);
        for property in file_properties{

        }
    }
    if dir_paths.len() > 0 {
        let directory_properties = set_directory_properties(&dir_paths, false);
        for property in directory_properties{
            property.info();
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
