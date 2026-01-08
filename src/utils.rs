#![allow(clippy::cast_possible_truncation)]

const PRIME32_1: u32 = 0x9E37_79B1;
const PRIME32_2: u32 = 0x85EB_CA77;
const PRIME32_3: u32 = 0xC2B2_AE3D;
const PRIME32_4: u32 = 0x27D4_EB2F;
const PRIME32_5: u32 = 0x1656_67B1;

/// Computes the 32-bit xxHash of the provided data.
///
/// This is a fast, non-cryptographic hash function suitable for checksums,
/// hash tables, and general-purpose hashing.
///
/// # Parameters
///
/// - `data`: The input byte slice to hash.
/// - `seed`: A seed value to initialize the hash state.
///
/// # Returns
///
/// Returns the 32-bit xxHash of `data`.
///
/// # Panics
///
/// Panics if an internal slice conversion fails. This can only occur if the
/// function attempts to read past the end of `data`, which should not happen
/// as long as the internal bounds checks remain correct.
///
/// # Examples
///
/// ```
/// let hash = xxhash32(b"hello", 0);
/// assert_ne!(hash, 0);
/// ```
#[must_use]
pub fn xxhash32(data: &[u8], seed: u32) -> u32 {
    let data_len = data.len() as u32;
    let mut offset = 0;

    let mut h32 = if data.len() >= 16 {
        let limit = data.len() - 16;
        let mut v1 = seed.wrapping_add(PRIME32_1).wrapping_add(PRIME32_2);
        let mut v2 = seed.wrapping_add(PRIME32_2);
        let mut v3 = seed;
        let mut v4 = seed.wrapping_sub(PRIME32_1);

        while offset <= limit {
            let val1 = u32::from_le_bytes(data[offset..offset + 4].try_into().unwrap());
            let val2 = u32::from_le_bytes(data[offset + 4..offset + 8].try_into().unwrap());
            let val3 = u32::from_le_bytes(data[offset + 8..offset + 12].try_into().unwrap());
            let val4 = u32::from_le_bytes(data[offset + 12..offset + 16].try_into().unwrap());

            v1 = v1
                .wrapping_add(val1.wrapping_mul(PRIME32_2))
                .rotate_left(13)
                .wrapping_mul(PRIME32_1);

            v2 = v2
                .wrapping_add(val2.wrapping_mul(PRIME32_2))
                .rotate_left(13)
                .wrapping_mul(PRIME32_1);

            v3 = v3
                .wrapping_add(val3.wrapping_mul(PRIME32_2))
                .rotate_left(13)
                .wrapping_mul(PRIME32_1);

            v4 = v4
                .wrapping_add(val4.wrapping_mul(PRIME32_2))
                .rotate_left(13)
                .wrapping_mul(PRIME32_1);

            offset += 16;
        }

        v1.rotate_left(1)
            .wrapping_add(v2.rotate_left(7))
            .wrapping_add(v3.rotate_left(12))
            .wrapping_add(v4.rotate_left(18))
    } else {
        seed.wrapping_add(PRIME32_5)
    };

    h32 = h32.wrapping_add(data_len);

    while offset + 4 <= data.len() {
        let val = u32::from_le_bytes(data[offset..offset + 4].try_into().unwrap());
        h32 = h32.wrapping_add(val.wrapping_mul(PRIME32_3));
        h32 = h32.rotate_left(17);
        h32 = h32.wrapping_mul(PRIME32_4);
        offset += 4;
    }

    while offset < data.len() {
        let val = u32::from(data[offset]);
        h32 = h32.wrapping_add(val.wrapping_mul(PRIME32_5));
        h32 = h32.rotate_left(11);
        h32 = h32.wrapping_mul(PRIME32_1);
        offset += 1;
    }

    h32 ^= h32 >> 15;
    h32 = h32.wrapping_mul(PRIME32_2);
    h32 ^= h32 >> 13;
    h32 = h32.wrapping_mul(PRIME32_3);
    h32 ^= h32 >> 16;

    h32
}
