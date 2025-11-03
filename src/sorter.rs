use std::{fs::{create_dir_all, rename}, path::PathBuf};
use clap::ValueEnum;
use crate::properties::{dir_props::set_directory_properties, traits::{Child_Items, Info}};

#[derive(Clone, Debug, ValueEnum, PartialEq)]
pub enum sort_flags{
    sn,
    se,
    sdm,
    sda,
    sdc
}
pub fn sort_files(dir_paths: &Vec<PathBuf>, sort_method: sort_flags, rec_value: u32){
    let dir_properties = set_directory_properties(&dir_paths, rec_value);
    for property in dir_properties{
        let files = property.retrive_child_files().expect("No files are there in this directory.");
        for file in files{
            let parent_path = property.location().expect("Err!! Retriving Location.").join(property.name().expect("Err!! Retriving file name."));
            let sort_path = parent_path.join(file.extension().expect("Err!! Can't retrive extension."));
            let file_as_path = PathBuf::from(file.name().expect("Err!! Retriving name"));
            if !sort_path.exists(){
                create_dir_all(&sort_path).expect("Err!! Failed to create directory.");
            }
            let _ = rename(parent_path.join(&file_as_path), sort_path.join(&file_as_path));
        }
    }
    match sort_method {
        sort_flags::sda => {

        },
        sort_flags::sdc => {

        },
        sort_flags::sdm => {

        },
        sort_flags::se => {

        }
        _ => {
            println!("Err!! Can't sort files. Not able to retrieve sort parameter.")
        }
    }
}