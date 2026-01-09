#![allow(clippy::cast_possible_truncation)]
#![allow(clippy::cast_sign_loss)]
#![allow(clippy::cast_possible_wrap)]

pub struct MersenneTwister {
    mt: [u32; 624],
    mti: usize,
}

impl MersenneTwister {
    const N: usize = 624;
    const M: usize = 397;
    const MATRIX_A: u32 = 0x9908B0DF;
    const UPPER_MASK: u32 = 0x80000000;
    const LOWER_MASK: u32 = 0x7FFFFFFF;

    pub fn new(seed: u32) -> Self {
        let mut mt = [0u32; Self::N];
        mt[0] = seed;
        for i in 1..Self::N {
            let prev = mt[i - 1];
            let x = prev ^ (prev >> 30);
            mt[i] = (1812433253u64.wrapping_mul(x as u64) as u32).wrapping_add(i as u32);
        }
        Self { mt, mti: Self::N }
    }

    fn generate_numbers(&mut self) {
        for i in 0..(Self::N - Self::M) {
            let y = (self.mt[i] & Self::UPPER_MASK) | (self.mt[i + 1] & Self::LOWER_MASK);
            self.mt[i] =
                self.mt[i + Self::M] ^ (y >> 1) ^ (if y % 2 != 0 { Self::MATRIX_A } else { 0 });
        }
        for i in (Self::N - Self::M)..(Self::N - 1) {
            let y = (self.mt[i] & Self::UPPER_MASK) | (self.mt[i + 1] & Self::LOWER_MASK);
            self.mt[i] = self.mt[i + Self::M - Self::N]
                ^ (y >> 1)
                ^ (if y % 2 != 0 { Self::MATRIX_A } else { 0 });
        }
        let y = (self.mt[Self::N - 1] & Self::UPPER_MASK) | (self.mt[0] & Self::LOWER_MASK);
        self.mt[Self::N - 1] =
            self.mt[Self::M - 1] ^ (y >> 1) ^ (if y % 2 != 0 { Self::MATRIX_A } else { 0 });
        self.mti = 0;
    }

    fn genrand_int32(&mut self) -> u32 {
        if self.mti >= Self::N {
            self.generate_numbers();
        }
        let mut y = self.mt[self.mti];
        self.mti += 1;

        y ^= y >> 11;
        y ^= (y << 7) & 0x9D2C5680;
        y ^= (y << 15) & 0xEFC60000;
        y ^= y >> 18;

        y
    }

    fn genrand_int31(&mut self) -> u32 {
        self.genrand_int32() >> 1
    }

    pub fn next_bytes(&mut self, length: usize) -> Vec<u8> {
        let mut res = Vec::with_capacity(length + 4);
        while res.len() < length {
            res.extend_from_slice(&self.genrand_int31().to_le_bytes());
        }
        res.truncate(length);
        res
    }
}

pub fn xxhash32(input: &[u8], seed: u32) -> u32 {
    const PRIME32_1: u32 = 2654435761;
    const PRIME32_2: u32 = 2246822519;
    const PRIME32_3: u32 = 3266489917;
    const PRIME32_4: u32 = 668265263;
    const PRIME32_5: u32 = 374761393;

    let len = input.len();
    let mut h32;
    let mut p = 0;
    let end = len;

    if len >= 16 {
        let limit = end - 16;
        let mut v1 = seed.wrapping_add(PRIME32_1).wrapping_add(PRIME32_2);
        let mut v2 = seed.wrapping_add(PRIME32_2);
        let mut v3 = seed;
        let mut v4 = seed.wrapping_sub(PRIME32_1);

        while p <= limit {
            v1 = v1.wrapping_add(
                u32::from_le_bytes(input[p..p + 4].try_into().unwrap()).wrapping_mul(PRIME32_2),
            );
            v1 = v1.rotate_left(13).wrapping_mul(PRIME32_1);

            v2 = v2.wrapping_add(
                u32::from_le_bytes(input[p + 4..p + 8].try_into().unwrap()).wrapping_mul(PRIME32_2),
            );
            v2 = v2.rotate_left(13).wrapping_mul(PRIME32_1);

            v3 = v3.wrapping_add(
                u32::from_le_bytes(input[p + 8..p + 12].try_into().unwrap())
                    .wrapping_mul(PRIME32_2),
            );
            v3 = v3.rotate_left(13).wrapping_mul(PRIME32_1);

            v4 = v4.wrapping_add(
                u32::from_le_bytes(input[p + 12..p + 16].try_into().unwrap())
                    .wrapping_mul(PRIME32_2),
            );
            v4 = v4.rotate_left(13).wrapping_mul(PRIME32_1);

            p += 16;
        }

        h32 = v1
            .rotate_left(1)
            .wrapping_add(v2.rotate_left(7))
            .wrapping_add(v3.rotate_left(12))
            .wrapping_add(v4.rotate_left(18));
    } else {
        h32 = seed.wrapping_add(PRIME32_5);
    }

    h32 = h32.wrapping_add(len as u32);

    while p + 4 <= end {
        h32 = h32.wrapping_add(
            u32::from_le_bytes(input[p..p + 4].try_into().unwrap()).wrapping_mul(PRIME32_3),
        );
        h32 = h32.rotate_left(17).wrapping_mul(PRIME32_4);
        p += 4;
    }

    while p < end {
        h32 = h32.wrapping_add((input[p] as u32).wrapping_mul(PRIME32_5));
        h32 = h32.rotate_left(11).wrapping_mul(PRIME32_1);
        p += 1;
    }

    h32 ^= h32 >> 15;
    h32 = h32.wrapping_mul(PRIME32_2);
    h32 ^= h32 >> 13;
    h32 = h32.wrapping_mul(PRIME32_3);
    h32 ^= h32 >> 16;

    h32
}

pub fn base64_encode(input: &[u8]) -> String {
    const CHARS: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let mut output = String::new();
    let mut i = 0;
    while i < input.len() {
        let b0 = input[i];
        let b1 = if i + 1 < input.len() { input[i + 1] } else { 0 };
        let b2 = if i + 2 < input.len() { input[i + 2] } else { 0 };

        let c0 = b0 >> 2;
        let c1 = ((b0 & 3) << 4) | (b1 >> 4);
        let c2 = ((b1 & 15) << 2) | (b2 >> 6);
        let c3 = b2 & 63;

        output.push(CHARS[c0 as usize] as char);
        output.push(CHARS[c1 as usize] as char);
        if i + 1 < input.len() {
            output.push(CHARS[c2 as usize] as char);
        } else {
            output.push('=');
        }
        if i + 2 < input.len() {
            output.push(CHARS[c3 as usize] as char);
        } else {
            output.push('=');
        }

        i += 3;
    }
    output
}

pub fn base64_decode(input: &str) -> Option<Vec<u8>> {
    let input = input.trim_end_matches('=');
    let mut output = Vec::new();
    let mut buf = 0u32;
    let mut bits = 0;

    for c in input.bytes() {
        let val = match c {
            b'A'..=b'Z' => c - b'A',
            b'a'..=b'z' => c - b'a' + 26,
            b'0'..=b'9' => c - b'0' + 52,
            b'+' => 62,
            b'/' => 63,
            _ => return None,
        };
        buf = (buf << 6) | val as u32;
        bits += 6;
        if bits >= 8 {
            bits -= 8;
            output.push((buf >> bits) as u8);
        }
    }
    Some(output)
}

pub fn create_key(name: &str, size: usize) -> Vec<u8> {
    let seed = xxhash32(name.as_bytes(), 0);
    let mut mt = MersenneTwister::new(seed);
    mt.next_bytes(size)
}

pub fn xor_bytes(data: &mut [u8], key: &[u8]) {
    if key.is_empty() {
        return;
    }
    for (i, b) in data.iter_mut().enumerate() {
        *b ^= key[i % key.len()];
    }
}

pub fn decrypt_string(encoded: &str, key: &[u8]) -> String {
    let mut raw = match base64_decode(encoded) {
        Some(b) => b,
        None => return String::from(encoded),
    };

    xor_bytes(&mut raw, key);

    let safe_len = raw.len() - (raw.len() % 2);
    let u16_vec: Vec<u16> = raw[..safe_len]
        .chunks_exact(2)
        .map(|chunk| u16::from_le_bytes([chunk[0], chunk[1]]))
        .collect();

    String::from_utf16_lossy(&u16_vec)
}

pub fn decrypt_i64(val: i64, key: &[u8]) -> i64 {
    let mut bytes = val.to_le_bytes();
    xor_bytes(&mut bytes, key);
    i64::from_le_bytes(bytes)
}

pub fn decrypt_u64(val: u64, key: &[u8]) -> u64 {
    let mut bytes = val.to_le_bytes();
    xor_bytes(&mut bytes, key);
    u64::from_le_bytes(bytes)
}

pub fn decrypt_i32(val: i32, key: &[u8]) -> i32 {
    let mut bytes = val.to_le_bytes();
    xor_bytes(&mut bytes, key);
    i32::from_le_bytes(bytes)
}
