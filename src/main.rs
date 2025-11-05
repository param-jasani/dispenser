mod display_props;
mod path_error_handler;
mod properties;
mod sorter;
use clap::Parser;
use display_props::{display_directory_properties, display_file_checksum, display_file_properties};
use path_error_handler::path_checker_and_separator;
use sorter::{SortFlags, sort_files};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// give path of the file or directory.
    #[arg(short, long, num_args = 1.., value_delimiter = ',', required = true)]
    paths: Vec<String>,

    /// sort files based on extension(se) - default, date created(sdc), date modified(sdm), date accessed(sda)
    #[arg(short, long, value_enum, default_value_t = SortFlags::SN)]
    sort: SortFlags,

    /// retrieve metadata of the given path.
    #[arg(short, long)]
    metadata: bool,

    /// depth level to traverse for directories
    #[arg(short, long, default_value_t = 0)]
    depth: u32,

    /// calculate SHA-256 checksum of files.
    #[arg(long)]
    checksum: bool,
}

fn main() {
    let args = Args::parse();
    let mut depth = Args::parse().depth;
    let (file_paths, dir_paths, metadata_na, invalid_paths) =
        path_checker_and_separator(&args.paths);
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
    if args.metadata {
        if file_paths.len() > 0 {
            display_file_properties(&file_paths);
        }
        if dir_paths.len() > 0 {
            display_directory_properties(&dir_paths, depth);
        }
    }
    if args.checksum {
        if file_paths.len() > 0 {
            display_file_checksum(&file_paths);
        }
    }
    if args.sort != SortFlags::SN {
        if dir_paths.len() > 0 {
            if depth == 0 {
                depth = 1;
            }
            sort_files(&dir_paths, args.sort, depth);
        }
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
