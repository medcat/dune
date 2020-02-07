#[macro_use]
extern crate serde;
#[macro_use]
extern crate failure;

pub mod document;
pub mod index;

pub use self::document::Document;
pub use self::document::Value;
pub use self::index::Index;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
