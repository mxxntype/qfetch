pub mod colors;
pub mod disk;
pub mod host;
pub mod os;
pub mod ram;
pub mod user;

pub trait FetchSource {
    /// Get the actual info string.
    fn info(&self) -> String;
    /// Get the icon associated with the source.
    fn icon(&self) -> char;
    /// Get a textual prefix to use instead of an icon.
    fn text_prefix(&self) -> String;
}
