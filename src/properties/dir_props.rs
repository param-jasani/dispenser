use core::{f64};
use chrono::{Local, DateTime};
use faccess::PathExt;
use super::{AccessMethods, traits::*, file_props::*};
use std::{path::PathBuf, fs};

#[derive(Debug)]
pub(crate) struct DirectoryProperties {
    name: Option<String>,
    ptype: String,
    size: f64,
    location: Option<PathBuf>,
    date_created: Option<DateTime<Local>>,
    n_o_files: u32,
    n_o_folders: u32,
    access_permissions: Vec<AccessMethods>,
    children_items: Option<FileFolders>
}

#[derive(Debug)]
struct FileFolders{
    files: Vec<FileProperties>,
    folders: Vec<DirectoryProperties>
}

impl AllInfo for DirectoryProperties {
    fn info(&self) {
        println!("{:?}", self);
    }
}

pub fn set_directory_properties(dir_paths: &Vec<PathBuf>, rec_flag: bool) -> Vec<DirectoryProperties>{
    let mut dir_prop_struct_collection: Vec<DirectoryProperties> = Vec::new();
    for path in dir_paths {
        let metadata = path.metadata().expect("Error!! Cannot retrive metadata.");
        let name = path.file_name().and_then(|p| p.to_str()).map(|p| p.to_string());
        let ptype = "directory".to_string();
        let location = path.parent().map(|p| p.to_path_buf());
        let date_created: Option<DateTime<Local>> = metadata.created().ok().map(|dc| DateTime::from(dc));
        let (size,  n_o_files, n_o_folders, files, folders) = get_size_and_count(path, rec_flag);        
        let mut access_permissions: Vec<AccessMethods> = Vec::new();
        if path.readable() {
            access_permissions.push(AccessMethods::Read);
        }
        if path.writable() {
            access_permissions.push(AccessMethods::Write);
        }
        if path.executable() {
            access_permissions.push(AccessMethods::Execute);
        }
        if rec_flag{
            let file_properties = set_file_properties(&files);
            let dir_props= set_directory_properties(&folders, true);
            dir_prop_struct_collection.push(DirectoryProperties{ name, ptype, size, location, date_created, n_o_files, n_o_folders, access_permissions, children_items: Some(FileFolders{files: file_properties, folders: dir_props}) }); 
        }
        else {
            dir_prop_struct_collection.push(DirectoryProperties{ name, ptype, size, location, date_created, n_o_files, n_o_folders, access_permissions, children_items: None}); 
        }
    }
    dir_prop_struct_collection
}

fn get_size_and_count(dir_path: &PathBuf, rec_flag: bool) -> (f64, u32, u32, Vec<PathBuf>, Vec<PathBuf>) {
    let child_items = fs::read_dir(dir_path).expect("Cannot read sub directories and files.");
    let mut size: f64 = 0.0;
    let mut n_o_folders: u32 = 0;
    let mut n_o_files: u32 = 0;
    let mut files: Vec<PathBuf> = Vec::new();
    let mut folders: Vec<PathBuf> = Vec::new();
    for child in child_items{
        let sub_item = child.expect("Err!! occured while parsing directory").path();
        let sub_item_metadata = sub_item.metadata().expect("Err!! while retriving metadata of sub items.");
        if sub_item_metadata.is_dir(){
            n_o_folders += 1;
            size += sub_item_metadata.len() as f64;
            if rec_flag{
                folders.push(sub_item.to_path_buf());
            }
        }
        else if sub_item.metadata().expect("Err!! while retriving metadata of sub items.").is_file() {
            n_o_files += 1;
            size += sub_item_metadata.len() as f64;
            if rec_flag{
                files.push(sub_item.to_path_buf());
            }
        }
    }
    (size / 1024 as f64, n_o_files, n_o_folders, files, folders)
}