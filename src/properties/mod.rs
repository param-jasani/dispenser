pub mod traits;
pub mod file_props;
pub mod dir_props;

#[derive(Clone, Debug)]
pub enum AccessMethods {
    Read,
    Write,
    Execute,
}
