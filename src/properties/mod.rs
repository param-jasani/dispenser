pub mod dir_props;
pub mod file_props;
pub mod traits;

#[derive(Clone, Debug)]
pub enum AccessMethods {
    Read,
    Write,
    Execute,
}
