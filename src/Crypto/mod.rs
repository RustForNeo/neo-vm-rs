#![feature(core_intrinsics)]

mod Murmur32;


/// Rotate the bits in `value` to the left by `offset` bits.
#[inline(always)]
pub fn rotate_left(value: u32, offset: u32) -> u32 {
    std::intrinsics::rotate_left(value, offset)
}

/// Rotate the bits in `value` to the left by `offset` bits.
#[inline(always)]
pub fn rotate_left_u64(value: u64, offset: u32) -> u64 {
    std::intrinsics::rotate_left(value, offset as u64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rotate_left() {
        assert_eq!(rotate_left(0x01234567, 8), 0x23456701);
        assert_eq!(rotate_left(0x01234567, 16), 0x34560123);
        assert_eq!(rotate_left(0x01234567, 24), 0x67012345);
        assert_eq!(rotate_left(0x01234567, 32), 0x01234567);
    }

    #[test]
    fn test_rotate_left_u64() {
        assert_eq!(rotate_left_u64(0x0123456789abcdef, 8), 0x23456789abcdef01);
        assert_eq!(rotate_left_u64(0x0123456789abcdef, 16), 0x3456789abcdef012);
        assert_eq!(rotate_left_u64(0x0123456789abcdef, 24), 0x56789abcdef01234);
        assert_eq!(rotate_left_u64(0x0123456789abcdef, 32), 0x89abcdef01234567);
        assert_eq!(rotate_left_u64(0x0123456789abcdef, 40), 0xbcdef0123456789a);
        assert_eq!(rotate_left_u64(0x0123456789abcdef, 48), 0xdef0123456789abc);
        assert_eq!(rotate_left_u64(0x0123456789abcdef, 56), 0xef0123456789abcd);
        assert_eq!(rotate_left_u64(0x0123456789abcdef, 64), 0x0123456789abcdef);
    }
}