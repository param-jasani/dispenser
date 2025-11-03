mod properties;
mod path_error_handler;
mod display_props;
mod sorter;
use clap::{Parser, ValueEnum}; 
use properties::{dir_props::*};
use path_error_handler::path_checker_and_separator;
use display_props::{display_file_properties, display_directory_properties};


#[derive(Clone, Debug, ValueEnum)]
enum sort_flags{
    sn,
    se,
    sdm,
    sda,
    sdc
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// give path of the file or directory.
    #[arg(short, long, num_args = 1.., value_delimiter = ',', required = true)]
    paths: Vec<String>,

    /// sort files based on extension(se) - default, date created(sdc), date modified(sdm), date accessed(sda)
    #[arg(short, long, value_enum, default_value_t = sort_flags::sn)]
    sort: sort_flags,

    /// retrieve metadata of the given path.
    #[arg(short, long)]
    metadata: bool,

    /// view metadata in table format
    #[arg(short, long)]
    table: bool,

    /// depth level to traverse for directories
    #[arg(short, long, default_value_t = 0)]
    depth: u32,

    /// compress.
    #[arg(short, long)]
    compress: bool,

    /// extract.
    #[arg(short, long)]
    extract: bool,

    /// lock file using AES-256
    #[arg(short, long)]
    lock: bool,

    /// calculate SHA-256 checksum of files.
    #[arg(long)]
    checksum: bool,

}


fn main() {
    let args = Args::parse();
    let mut depth = Args::parse().depth;
    let (file_paths, dir_paths, symlink_paths, metadata_na, invalid_paths) = path_checker_and_separator(&args.paths);
    let ascii_art = r##"
                                  ______   __                                          
                                 |   _  \ |__.-----.-----.-----.-----.-----.-----.----.
                                 |.  |   \|  |__ --|  _  |  -__|     |__ --|  -__|   _|
                                 |.  |    |__|_____|   __|_____|__|__|_____|_____|__|  
                                 |:  1    /        |__|                                
                                 |::.. . /                                             
                                 `------'                                              

                                         An advance all in one file and folder 
                                                  management tool.
    "##;
    println!("{}", ascii_art);
    if file_paths.len() > 0 {
        display_file_properties(&file_paths);
    }
    if dir_paths.len() > 0 {
        display_directory_properties(&dir_paths, depth);
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
