use std::{fs::{create_dir_all, rename}, path::PathBuf};
use clap::ValueEnum;
use crate::properties::{dir_props::set_directory_properties, file_props::FileProperties, traits::{Child_Items, Dates, Info}};

#[derive(Clone, Debug, ValueEnum, PartialEq)]
pub enum sort_flags{
    sn,
    se,
    sdm,
    sda,
    sdc
}

fn sort_selector(sort_method: &sort_flags, parent_path: &PathBuf, file: &FileProperties) -> Option<PathBuf>{
    match sort_method {
        sort_flags::sda => {
            Some(parent_path.join(file.date_accessed().expect("Err!! Can't retrive date accessed.").date_naive().to_string()))
        },
        sort_flags::sdc => {
            Some(parent_path.join(file.date_created().expect("Err!! Can't retrive date accessed.").date_naive().to_string()))
        },
        sort_flags::sdm => {
            Some(parent_path.join(file.date_modified().expect("Err!! Can't retrive date accessed.").date_naive().to_string()))
        },
        sort_flags::se => {
            Some(parent_path.join(file.extension().expect("Err!! Can't retrive extension.")))
        }
        _ => {
            None
        }
    }
}

pub fn sort_files(dir_paths: &Vec<PathBuf>, sort_method: sort_flags, rec_value: u32){
    let dir_properties = set_directory_properties(&dir_paths, rec_value);
    for property in dir_properties{
        let files = property.retrive_child_files().expect("No files are there in this directory.");
        for file in files{
            let parent_path = property.location().expect("Err!! Retriving Location.").join(property.name().expect("Err!! Retriving file name."));
            let sort_path = sort_selector(&sort_method, &parent_path, &file).expect("Err!! Not able to retrive sort method");
            let file_as_path = PathBuf::from(file.name().expect("Err!! Retriving name"));
            if !sort_path.exists(){
                create_dir_all(&sort_path).expect("Err!! Failed to create directory.");
            }
            let _ = rename(parent_path.join(&file_as_path), sort_path.join(&file_as_path));
        }
    }
}