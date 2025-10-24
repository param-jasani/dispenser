mod properties;
mod path_error_handler;

use clap::{Parser}; 
use properties::file_props::create_file_properties;
use properties::traits::*;
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
        let file_properties = create_file_properties(file_paths);
        for file_property in file_properties{
            println!("{}", file_property.name().unwrap());
        }
    }
    if dir_paths.len() > 0 {
        
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
