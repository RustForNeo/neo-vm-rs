
mod Crypto;
mod Exception;
mod Script;
mod Types;

pub use Crypto::*;
pub use Exception::*;
pub use Script::*;
pub use Types::*;


pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
