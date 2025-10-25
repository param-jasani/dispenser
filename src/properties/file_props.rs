use core::{f64};
use std::{fs, path::PathBuf};
use faccess::PathExt;
use sha256::digest;
use chrono::{Local, DateTime};
use super::{AccessMethods, traits::*};


#[derive(Clone)]
pub(crate) struct FileProperties {
    name: Option<String>, 
    extension: Option<String>,
    size: f64,
    location: Option<PathBuf>, 
    date_created: Option<DateTime<Local>>,
    date_modified: Option<DateTime<Local>>,
    date_accessed: Option<DateTime<Local>>,
    access_permissions: Vec<AccessMethods>, 
}


impl Info for FileProperties {
    fn name(&self) -> Option<String> {
        self.name.to_owned()
    }
    
    fn extension(&self) -> Option<String> {
        self.extension.to_owned()
    }

    fn location(&self) -> Option<PathBuf> {
        self.location.to_owned()
    }

    fn size(&self) -> f64 {
        self.size
    }
}

impl Dates for FileProperties {
    fn date_created(&self) -> Option<DateTime<Local>> {
        self.date_created
    }

    fn date_modified(&self) -> Option<DateTime<Local>> {
        self.date_modified    
    }

    fn date_accessed(&self) -> Option<DateTime<Local>> {
        self.date_accessed
    }
}

impl Permission for FileProperties {
    fn access_configs(&self) -> Vec<AccessMethods> {
        self.access_permissions.to_owned()
    }
}

impl Hash for FileProperties{
    fn hash(&self) -> Option<String> {
        fs::read(self.location.to_owned().expect("Err!! Couldn't find the given location of file.").join(self.name.to_owned().expect("Err!! Unable to fetch file name."))).ok().map(|file_contents| digest(file_contents))
    }    
}


pub fn set_file_properties(file_path: &Vec<PathBuf>) -> Vec<FileProperties> {
    let mut file_prop_struct_collection: Vec<FileProperties> = Vec::new();
    for path in file_path {
        let metadata = path.metadata().expect("Error!! Cannot retrive metadata.");
        let name = path.file_name().and_then(|p| p.to_str()).map(|p| p.to_string());
        let extension = path.extension().and_then(|e| e.to_str()).map(|e| e.to_string());
        let size = metadata.len() as f64 / 1024 as f64;
        let location = path.parent().map(|p| p.to_path_buf());
        let date_created: Option<DateTime<Local>> = metadata.created().ok().map(|dc| DateTime::from(dc));
        let date_modified: Option<DateTime<Local>> = metadata.created().ok().map(|dm| DateTime::from(dm)); 
        let date_accessed: Option<DateTime<Local>> = metadata.created().ok().map(|da| DateTime::from(da)); 
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
        file_prop_struct_collection.push(FileProperties { name, extension, size, location, date_created, date_modified, date_accessed, access_permissions}); 
    }
    file_prop_struct_collection
}
