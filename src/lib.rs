/// Concatenates two `u16` values into a `u32`.
/// `high` becomes the upper 16 bits, `low` becomes the lower 16 bits.
#[inline]
pub const fn concat_u16_to_u32(high: u16, low: u16) -> u32 {
    ((high as u32) << 16) | (low as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_concat() {
        assert_eq!(concat_u16_to_u32(0x1234, 0xABCD), 0x1234ABCD);
        assert_eq!(concat_u16_to_u32(0x0000, 0x0000), 0x00000000);
        assert_eq!(concat_u16_to_u32(0xFFFF, 0xFFFF), 0xFFFFFFFF);
    }
}
