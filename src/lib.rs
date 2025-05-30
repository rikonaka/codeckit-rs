const BASE32_MAP: [&str; 32] = [
    "A", "B", "C", "D", "E", "F", "G", "H", "I", "J", "K", "L", "M", "N", "O", "P", "Q", "R", "S",
    "T", "U", "V", "W", "X", "Y", "Z", "2", "3", "4", "5", "6", "7",
];

const BASE32_REVERSE_MAP: [u8; 256] = [
    255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
    255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
    255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 26, 27, 28, 29, 30, 31, 255, 255,
    255, 255, 255, 255, 255, 255, 255, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16,
    17, 18, 19, 20, 21, 22, 23, 24, 25, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
    255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
    255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
    255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
    255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
    255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
    255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
    255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
    255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
    255,
];

pub struct Base32;

impl Base32 {
    /// Encodes the input bytes into a Base32 string.
    pub fn encode(input: &[u8]) -> String {
        let mut ret = String::with_capacity(((input.len() + 4) / 5) * 8);
        let mut flag = 0;
        let mut prev: u8 = 0;

        for &i in input {
            match flag {
                0 => {
                    let ind = i >> 3;
                    prev = (i & 0b00000111) << 2;
                    ret.push_str(BASE32_MAP[ind as usize]);
                    flag = 1;
                }
                1 => {
                    let ind = prev + (i >> 6);
                    let ind_1 = (i & 0b00111111) >> 1;
                    ret.push_str(BASE32_MAP[ind as usize]);
                    ret.push_str(BASE32_MAP[ind_1 as usize]);
                    prev = (i & 0b00000001) << 4;
                    flag = 2;
                }
                2 => {
                    let ind = prev + (i >> 4);
                    prev = (i & 0b00001111) << 1;
                    ret.push_str(BASE32_MAP[ind as usize]);
                    flag = 3;
                }
                3 => {
                    let ind = prev + (i >> 7);
                    let ind_1 = (i & 0b01111111) >> 2;
                    ret.push_str(BASE32_MAP[ind as usize]);
                    ret.push_str(BASE32_MAP[ind_1 as usize]);
                    prev = (i & 0b00000011) << 3;
                    flag = 4;
                }
                4 => {
                    let ind = prev + (i >> 5);
                    let ind_1 = i & 0b00011111;
                    ret.push_str(BASE32_MAP[ind as usize]);
                    ret.push_str(BASE32_MAP[ind_1 as usize]);
                    prev = 0;
                    flag = 0;
                }
                _ => unreachable!(),
            }
        }

        ret.push_str(BASE32_MAP[prev as usize]);
        // add padding if necessary
        while ret.len() % 8 != 0 {
            ret.push('=');
        }
        ret
    }
    /// Decodes a Base32 string into a Vec<u8>.
    /// This function ignores invalid characters automatically and not returns an error.
    pub fn decode(input: &str) -> Vec<u8> {
        let mut ret = Vec::new();
        let mut prev: u8 = 0;
        let mut flag = 0;

        for c in input.chars() {
            if c == '=' {
                break;
            }
            let i_rev = BASE32_REVERSE_MAP[c as usize];
            // drop invalid characters and 255 means invalid character
            if i_rev == 255 {
                continue;
            }
            match flag {
                // i_rev(5bits) => prev
                0 => {
                    prev = i_rev << 3;
                    flag = 1;
                }
                // prev(5bits) + i(3bits)
                // new prev(2bits)
                1 => {
                    let ch = prev + (i_rev >> 2);
                    ret.push(ch);
                    prev = (i_rev & 0b00000011) << 6;
                    flag = 2;
                }
                // new prev(7bits) = prev(2bits) + i(5bits)
                2 => {
                    prev = prev + (i_rev << 1);
                    flag = 3;
                }
                // prev(7bits) + i(1bit)
                // new prev(4bits)
                3 => {
                    let ch = prev + (i_rev >> 4);
                    ret.push(ch);
                    prev = (i_rev & 0b00001111) << 4;
                    flag = 4;
                }
                // prev(4bits) + i(4bits)
                // new prev(1bits)
                4 => {
                    let ch = prev + (i_rev >> 1);
                    ret.push(ch);
                    prev = (i_rev & 0b00000001) << 7;
                    flag = 5;
                }
                // new prev(6) = prev(1bits) + i(5bits)
                5 => {
                    prev = prev + (i_rev << 2);
                    flag = 6;
                }
                // prev(6bits) + i(2bits)
                // new prev(3bits)
                6 => {
                    let ch = prev + (i_rev >> 3);
                    ret.push(ch);
                    prev = (i_rev & 0b00000111) << 5;
                    flag = 7;
                }
                // prev(3bits) + i(5bits)
                // new prev(0bits)
                7 => {
                    let ch = prev + i_rev;
                    ret.push(ch);
                    prev = 0;
                    flag = 0;
                }
                _ => unreachable!(),
            }
        }
        ret
    }
}

const BASE58_MAP: [&str; 58] = [
    "1", "2", "3", "4", "5", "6", "7", "8", "9", "A", "B", "C", "D", "E", "F", "G", "H", "J", "K",
    "L", "M", "N", "P", "Q", "R", "S", "T", "U", "V", "W", "X", "Y", "Z", "a", "b", "c", "d", "e",
    "f", "g", "h", "i", "j", "k", "m", "n", "o", "p", "q", "r", "s", "t", "u", "v", "w", "x", "y",
    "z",
    // 'o', 'l' are omitted to avoid confusion with '0' and '1'
    // 'O', 'I' are omitted to avoid confusion with '0' and '1'
    // '0', '1' are omitted to avoid confusion with 'O' and 'I'
    // '2', '3', ..., '9' are included
];

const BASE58_REVERSE_MAP: [u8; 256] = [
    255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
    255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
    255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 0, 1, 2, 3, 4, 5, 6, 7, 8, 255, 255,
    255, 255, 255, 255, 255, 9, 10, 11, 12, 13, 14, 15, 16, 255, 17, 18, 19, 20, 21, 255, 22, 23,
    24, 25, 26, 27, 28, 29, 30, 31, 32, 255, 255, 255, 255, 255, 255, 33, 34, 35, 36, 37, 38, 39,
    40, 41, 42, 43, 255, 44, 45, 46, 47, 48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 255, 255, 255,
    255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
    255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
    255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
    255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
    255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
    255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
    255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
];

pub struct Base58;

impl Base58 {
    fn divmod58(num: &[u8]) -> (Vec<u8>, u8) {
        let mut quotient = Vec::new();
        let mut remainder: u8 = 0;
        for &digit in num {
            let value = (remainder as u32) * 256 + digit as u32;
            remainder = (value % 58) as u8;
            quotient.push((value / 58) as u8);
        }
        // remove leading zeros
        while quotient.len() > 1 && quotient[0] == 0 {
            quotient.remove(0);
        }
        (quotient, remainder)
    }
    /// Encodes the input bytes into a Base58 string.
    pub fn encode(input: &[u8]) -> String {
        let zeros = input.iter().take_while(|&&x| x == 0).count();

        let mut num = input.to_vec();
        let mut encoded = String::new();

        while !num.iter().all(|&x| x == 0) {
            let (quotient, remainder) = Self::divmod58(&num);
            encoded.push_str(BASE58_MAP[remainder as usize]);
            num = quotient;
        }

        for _ in 0..zeros {
            encoded.push('1');
        }

        encoded = encoded.chars().rev().collect();
        encoded
    }
    /// Decodes a Base58 string into a Vec<u8>.
    pub fn decode(input: &str) -> Vec<u8> {
        let mut num = vec![0u8];
        for c in input.chars() {
            let val = BASE58_REVERSE_MAP[c as usize];
            let mut carry = val as u32;
            if carry == 255 {
                // invalid character, skip it
                continue;
            }
            for n in num.iter_mut() {
                let total = *n as u32 * 58 + carry;
                *n = (total & 0xff) as u8;
                carry = total >> 8;
            }

            while carry > 0 {
                num.push((carry & 0xff) as u8);
                carry >>= 8;
            }
        }

        let mut n_zeros = 0;
        for c in input.chars() {
            if c == '1' {
                n_zeros += 1;
            } else {
                break;
            }
        }
        let mut result = vec![0u8; n_zeros];
        result.extend(num.iter().rev());
        result
    }
}

const BASE62_MAP: [&str; 62] = [
    "0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "A", "B", "C", "D", "E", "F", "G", "H", "I",
    "J", "K", "L", "M", "N", "O", "P", "Q", "R", "S", "T", "U", "V", "W", "X", "Y", "Z", "a", "b",
    "c", "d", "e", "f", "g", "h", "i", "j", "k", "l", "m", "n", "o", "p", "q", "r", "s", "t", "u",
    "v", "w", "x", "y", "z",
];

const BASE62_REVERSE_MAP: [u8; 256] = [
    255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
    255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
    255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 255, 255, 255,
    255, 255, 255, 255, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28,
    29, 30, 31, 32, 33, 34, 35, 255, 255, 255, 255, 255, 255, 36, 37, 38, 39, 40, 41, 42, 43, 44,
    45, 46, 47, 48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 255, 255, 255, 255, 255,
    255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
    255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
    255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
    255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
    255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
    255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
    255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
];

pub struct Base62;

impl Base62 {
    fn divmod62(num: &[u8]) -> (Vec<u8>, u8) {
        let mut quotient = Vec::new();
        let mut remainder: u8 = 0;
        for &digit in num {
            let value = (remainder as u32) * 256 + digit as u32;
            remainder = (value % 62) as u8;
            quotient.push((value / 62) as u8);
        }
        // remove leading zeros
        while quotient.len() > 1 && quotient[0] == 0 {
            quotient.remove(0);
        }
        (quotient, remainder)
    }
    /// Encodes the input bytes into a Base62 string.
    pub fn encode(input: &[u8]) -> String {
        let zeros = input.iter().take_while(|&&x| x == 0).count();

        let mut num = input.to_vec();
        let mut encoded = String::new();

        while !num.iter().all(|&x| x == 0) {
            let (quotient, remainder) = Self::divmod62(&num);
            encoded.push_str(BASE62_MAP[remainder as usize]);
            num = quotient;
        }

        for _ in 0..zeros {
            encoded.push('1');
        }

        encoded = encoded.chars().rev().collect();
        encoded
    }
    /// Decodes a Base62 string into a Vec<u8>.
    pub fn decode(input: &str) -> Vec<u8> {
        let mut num = vec![0u8];
        for c in input.chars() {
            let val = BASE62_REVERSE_MAP[c as usize];
            if val == 255 {
                // invalid character, skip it
                continue;
            }
            let mut carry = val as u32;
            for n in num.iter_mut() {
                let total = *n as u32 * 62 + carry;
                *n = (total & 0xff) as u8;
                carry = total >> 8;
            }

            while carry > 0 {
                num.push((carry & 0xff) as u8);
                carry >>= 8;
            }
        }

        let mut n_zeros = 0;
        for c in input.chars() {
            if c == '0' {
                n_zeros += 1;
            } else {
                break;
            }
        }
        let mut result = vec![0u8; n_zeros];
        result.extend(num.iter().rev());
        result
    }
}

const BASE64_MAP: [&str; 64] = [
    "A", "B", "C", "D", "E", "F", "G", "H", "I", "J", "K", "L", "M", "N", "O", "P", "Q", "R", "S",
    "T", "U", "V", "W", "X", "Y", "Z", "a", "b", "c", "d", "e", "f", "g", "h", "i", "j", "k", "l",
    "m", "n", "o", "p", "q", "r", "s", "t", "u", "v", "w", "x", "y", "z", "0", "1", "2", "3", "4",
    "5", "6", "7", "8", "9", "+", "/",
];

const BASE64_REVERSE_MAP: [u8; 256] = [
    255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
    255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
    255, 255, 255, 255, 255, 62, 255, 255, 255, 63, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 255,
    255, 255, 255, 255, 255, 255, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18,
    19, 20, 21, 22, 23, 24, 25, 255, 255, 255, 255, 255, 255, 26, 27, 28, 29, 30, 31, 32, 33, 34,
    35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49, 50, 51, 255, 255, 255, 255, 255,
    255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
    255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
    255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
    255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
    255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
    255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
    255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
];
pub struct Base64;

impl Base64 {
    /// Encodes the input bytes into a Base64 string.
    /// ```rust
    /// use codeckit::Base64;
    ///
    /// fn main() {
    ///     let test_str = "test";
    ///     let encoded = Base64::encode(test_str.as_bytes());
    ///     println!("{}", encoded);
    ///     let original = Base64::decode(&encoded);
    ///     let original = String::from_utf8(original).unwrap();
    ///     println!("{:?}", original);
    /// }
    /// ```
    pub fn encode(input: &[u8]) -> String {
        let mut ret = String::with_capacity(((input.len() + 2) / 3) * 4);
        let mut flag = 0;
        let mut prev: u8 = 0;

        for &i in input {
            match flag {
                0 => {
                    let ind = i >> 2;
                    prev = (i & 0b00000011) << 4;
                    ret.push_str(BASE64_MAP[ind as usize]);
                    flag = 1;
                }
                1 => {
                    let ind = prev + (i >> 4);
                    prev = (i & 0b00001111) << 2;
                    ret.push_str(BASE64_MAP[ind as usize]);
                    flag = 2;
                }
                2 => {
                    let ind = prev + (i >> 6);
                    let ind_1 = i & 0b00111111;
                    ret.push_str(BASE64_MAP[ind as usize]);
                    ret.push_str(BASE64_MAP[ind_1 as usize]);
                    prev = 0;
                    flag = 0;
                }
                _ => unreachable!(),
            }
        }

        ret.push_str(BASE64_MAP[prev as usize]);
        // add padding if necessary
        while ret.len() % 4 != 0 {
            ret.push('=');
        }
        ret
    }
    /// Decodes a Base64 string into a Vec<u8>.
    /// This function ignores invalid characters automatically and not returns an error.
    pub fn decode(input: &str) -> Vec<u8> {
        let mut ret: Vec<u8> = Vec::with_capacity((input.len() / 4) * 3);
        let mut flag: u8 = 0;
        let mut prev: u8 = 0;
        for i in input.chars() {
            if i == '=' {
                break;
            }
            let i_rev = BASE64_REVERSE_MAP[i as usize];
            // drop invalid characters and 255 means invalid character
            if i_rev == 255 {
                continue;
            }
            match flag {
                0 => {
                    prev = i_rev << 2;
                    flag = 1;
                }
                1 => {
                    let ch = prev + ((i_rev & 0b00110000) >> 4);
                    if ch != 0 {
                        ret.push(ch);
                    }
                    prev = (i_rev & 0b00001111) << 4;
                    flag = 2;
                }
                2 => {
                    let ch = prev + ((i_rev & 0b00111100) >> 2);
                    if ch != 0 {
                        ret.push(ch);
                    }
                    prev = (i_rev & 0b00000011) << 6;
                    flag = 3;
                }
                3 => {
                    let ch = prev + (i_rev & 0b00111111);
                    if ch != 0 {
                        ret.push(ch);
                    }
                    prev = 0;
                    flag = 0;
                }
                _ => unreachable!(),
            }
        }
        match flag {
            1 | 2 | 3 => {
                if prev != 0 {
                    ret.push(prev);
                }
            }
            _ => (), // ignore 0
        }
        ret
    }
}

const BASE64_URL_MAP: [&str; 64] = [
    "A", "B", "C", "D", "E", "F", "G", "H", "I", "J", "K", "L", "M", "N", "O", "P", "Q", "R", "S",
    "T", "U", "V", "W", "X", "Y", "Z", "a", "b", "c", "d", "e", "f", "g", "h", "i", "j", "k", "l",
    "m", "n", "o", "p", "q", "r", "s", "t", "u", "v", "w", "x", "y", "z", "0", "1", "2", "3", "4",
    "5", "6", "7", "8", "9", "-", "_",
];

const BASE64_URL_REVERSE_MAP: [u8; 256] = [
    255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
    255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
    255, 255, 255, 255, 255, 255, 255, 62, 255, 255, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 255,
    255, 255, 255, 255, 255, 255, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18,
    19, 20, 21, 22, 23, 24, 25, 255, 255, 255, 255, 63, 255, 26, 27, 28, 29, 30, 31, 32, 33, 34,
    35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49, 50, 51, 255, 255, 255, 255, 255,
    255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
    255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
    255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
    255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
    255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
    255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
    255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
];

pub struct Base64Url;

impl Base64Url {
    /// Encodes the input bytes into a Base64Url string.
    /// If padding is true, it will add '=' padding characters to the end of the string.
    pub fn encode(input: &[u8], padding: bool) -> String {
        let mut ret = String::with_capacity(((input.len() + 2) / 3) * 4);
        let mut flag = 0;
        let mut prev: u8 = 0;

        for &i in input {
            match flag {
                0 => {
                    let ind = i >> 2;
                    prev = (i & 0b00000011) << 4;
                    ret.push_str(BASE64_URL_MAP[ind as usize]);
                    flag = 1;
                }
                1 => {
                    let ind = prev + (i >> 4);
                    prev = (i & 0b00001111) << 2;
                    ret.push_str(BASE64_URL_MAP[ind as usize]);
                    flag = 2;
                }
                2 => {
                    let ind = prev + (i >> 6);
                    let ind_1 = i & 0b00111111;
                    ret.push_str(BASE64_URL_MAP[ind as usize]);
                    ret.push_str(BASE64_URL_MAP[ind_1 as usize]);
                    prev = 0;
                    flag = 0;
                }
                _ => unreachable!(),
            }
        }

        ret.push_str(BASE64_URL_MAP[prev as usize]);
        if padding {
            // add padding if necessary
            while ret.len() % 4 != 0 {
                ret.push('=');
            }
        }
        ret
    }
    /// Decodes a Base64Url string into a Vec<u8>.
    pub fn decode(input: &str) -> Vec<u8> {
        let mut ret: Vec<u8> = Vec::with_capacity((input.len() / 4) * 3);
        let mut flag: u8 = 0;
        let mut prev: u8 = 0;
        for i in input.chars() {
            if i == '=' {
                break;
            }
            let i_rev = BASE64_URL_REVERSE_MAP[i as usize];
            // drop invalid characters and 255 means invalid character
            if i_rev == 255 {
                continue;
            }
            match flag {
                0 => {
                    prev = i_rev << 2;
                    flag = 1;
                }
                1 => {
                    let ch = prev + ((i_rev & 0b00110000) >> 4);
                    if ch != 0 {
                        ret.push(ch);
                    }
                    prev = (i_rev & 0b00001111) << 4;
                    flag = 2;
                }
                2 => {
                    let ch = prev + ((i_rev & 0b00111100) >> 2);
                    if ch != 0 {
                        ret.push(ch);
                    }
                    prev = (i_rev & 0b00000011) << 6;
                    flag = 3;
                }
                3 => {
                    let ch = prev + (i_rev & 0b00111111);
                    if ch != 0 {
                        ret.push(ch);
                    }
                    prev = 0;
                    flag = 0;
                }
                _ => unreachable!(),
            }
        }
        match flag {
            1 | 2 | 3 => {
                if prev != 0 {
                    ret.push(prev);
                }
            }
            _ => (), // ignore 0
        }
        ret
    }
}

pub struct Ascii85;

impl Ascii85 {
    fn divmod85(num: &[u8]) -> (Vec<u8>, u8) {
        let mut quotient = Vec::new();
        let mut remainder: u8 = 0;
        for &digit in num {
            let value = (remainder as u32) * 256 + digit as u32;
            remainder = (value % 85) as u8;
            quotient.push((value / 85) as u8);
        }
        // remove leading zeros
        while quotient.len() > 1 && quotient[0] == 0 {
            quotient.remove(0);
        }
        (quotient, remainder)
    }
    /// Encodes a 4-byte chunk into a Ascii85 string.
    /// This function is used internally by the `encode` method.
    fn inner_encode(input: &[u8]) -> String {
        if input.iter().all(|&x| x == 0) {
            return "z".to_string(); // special case for all zeros
        } else {
            let mut encoded = String::new();
            let mut num = input.to_vec();

            while !num.iter().all(|&x| x == 0) {
                let (quotient, remainder) = Self::divmod85(&num);
                let remainder_char = (remainder + 33) as char;
                encoded.push(remainder_char);
                num = quotient;
            }
            encoded.chars().rev().collect()
        }
    }
    /// Encodes the input bytes into a Base85 string.
    pub fn encode(input: &[u8]) -> String {
        let mut num = input.to_vec();
        let mut padding: u8 = 0;
        while num.len() % 4 != 0 {
            num.push(0); // pad with zeros to make the length a multiple of 4
            padding += 1;
        }

        let ind = num.len() / 4;
        let mut ret = Vec::new();
        for i in 0..ind {
            let start = i * 4;
            let end = start + 4;
            let chunk = &num[start..end];
            let encoded_chunk = Self::inner_encode(chunk);

            let encoded_chunk = if i == ind - 1 {
                let encoded_chunk = match padding {
                    0 => encoded_chunk,                   // no padding
                    1 => encoded_chunk[0..4].to_string(), // keep first 4 chars
                    2 => encoded_chunk[0..3].to_string(), // keep first 3 chars
                    3 => encoded_chunk[0..2].to_string(), // keep first 2 chars
                    _ => unreachable!(),                  // should never happen
                };
                encoded_chunk
            } else {
                encoded_chunk
            };
            ret.push(encoded_chunk);
        }
        ret.join("")
    }
    /// Decodes 5 characters of Base85 into a Vec<u8>.
    /// This function is used internally by the `decode` method.
    fn inner_decode(input_chars: &[u8]) -> Vec<u8> {
        let mut num: Vec<u8> = vec![0];
        for &c in input_chars {
            if c > 84 {
                // invalid character, skip it
                continue;
            }
            if c + 33 == ('z' as u8) {
                // special case for all zeros
                num.extend(vec![0, 0, 0, 0]);
            } else {
                let mut carry = c as u32;
                for n in num.iter_mut() {
                    let total = *n as u32 * 85 + carry;
                    *n = (total & 0xff) as u8;
                    carry = total >> 8;
                }

                while carry > 0 {
                    num.push((carry & 0xff) as u8);
                    carry >>= 8;
                }
            }
        }
        num.reverse();
        num
    }
    /// Decodes a Base85 string into a Vec<u8>.
    pub fn decode(input: &str) -> Vec<u8> {
        let mut ret = Vec::new();
        if input.len() == 0 {
            return ret; // empty input
        }

        let mut input_chars: Vec<u8> = input.chars().into_iter().map(|x| x as u8 - 33).collect();
        let mut padding: u8 = 0;
        while input_chars.len() % 5 != 0 {
            input_chars.push(84); // pad with 84 to make the length a multiple of 5
            padding += 1;
        }

        let ind = input_chars.len() / 5;
        for i in 0..ind {
            let start = i * 5;
            let end = start + 5;
            let chunk = &input_chars[start..end];
            let decoded_chunk = Self::inner_decode(chunk);

            let decoded_chunk = if i == ind - 1 {
                // remove padding
                match padding {
                    0 => decoded_chunk,                // no padding
                    1 => decoded_chunk[0..3].to_vec(), // keep first 3 bytes
                    2 => decoded_chunk[0..2].to_vec(), // keep first 2 bytes
                    3 => decoded_chunk[0..1].to_vec(), // keep first 1 bytes
                    _ => unreachable!(),               // should never happen
                }
            } else {
                decoded_chunk
            };
            ret.extend(decoded_chunk);
        }
        ret
    }
}

const BASE85_GIT_MAP: [&str; 85] = [
    "0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "A", "B", "C", "D", "E", "F", "G", "H", "I",
    "J", "K", "L", "M", "N", "O", "P", "Q", "R", "S", "T", "U", "V", "W", "X", "Y", "Z", "a", "b",
    "c", "d", "e", "f", "g", "h", "i", "j", "k", "l", "m", "n", "o", "p", "q", "r", "s", "t", "u",
    "v", "w", "x", "y", "z", "!", "#", "$", "%", "&", "(", ")", "*", "+", "-", ";", "<", "=", ">",
    "?", "@", "^", "_", "`", "{", "|", "}", "~",
];

const BASE85_GIT_REVERSE_MAP: [u8; 256] = [
    255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
    255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 62, 255, 63, 64, 65, 66,
    255, 67, 68, 69, 70, 255, 71, 255, 255, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 255, 72, 73, 74, 75, 76,
    77, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32,
    33, 34, 35, 255, 255, 255, 78, 79, 80, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49,
    50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 81, 82, 83, 84, 255, 255, 255, 255, 255, 255,
    255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
    255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
    255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
    255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
    255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
    255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
    255, 255, 255, 255, 255, 255, 255, 255, 255,
];

pub struct Base85Git;

impl Base85Git {
    fn divmod85(num: &[u8]) -> (Vec<u8>, u8) {
        let mut quotient = Vec::new();
        let mut remainder: u8 = 0;
        for &digit in num {
            let value = (remainder as u32) * 256 + digit as u32;
            remainder = (value % 85) as u8;
            quotient.push((value / 85) as u8);
        }
        // remove leading zeros
        while quotient.len() > 1 && quotient[0] == 0 {
            quotient.remove(0);
        }
        (quotient, remainder)
    }
    /// Encodes a 4-byte chunk into a Ascii85 string.
    /// This function is used internally by the `encode` method.
    fn inner_encode(input: &[u8]) -> String {
        let mut encoded = String::new();
        let mut num = input.to_vec();

        while !num.iter().all(|&x| x == 0) {
            let (quotient, remainder) = Self::divmod85(&num);
            let remainder_char = BASE85_GIT_MAP[remainder as usize];
            encoded.push_str(remainder_char);
            num = quotient;
        }
        encoded.chars().rev().collect()
    }
    /// Encodes the input bytes into a Base85 string.
    pub fn encode(input: &[u8]) -> String {
        let mut num = input.to_vec();
        let mut padding: u8 = 0;
        while num.len() % 4 != 0 {
            num.push(0); // pad with zeros to make the length a multiple of 4
            padding += 1;
        }

        let ind = num.len() / 4;
        let mut ret = Vec::new();
        for i in 0..ind {
            let start = i * 4;
            let end = start + 4;
            let chunk = &num[start..end];
            let encoded_chunk = Self::inner_encode(chunk);

            let encoded_chunk = if i == ind - 1 {
                match padding {
                    0 => encoded_chunk,                   // no padding
                    1 => encoded_chunk[0..4].to_string(), // keep first 4 chars
                    2 => encoded_chunk[0..3].to_string(), // keep first 3 chars
                    3 => encoded_chunk[0..2].to_string(), // keep first 2 chars
                    _ => unreachable!(),                  // should never happen
                }
            } else {
                encoded_chunk
            };
            ret.push(encoded_chunk);
        }
        ret.join("")
    }
    /// Decodes 5 characters of Base85 into a Vec<u8>.
    /// This function is used internally by the `decode` method.
    fn inner_decode(input_chars: &[u8]) -> Vec<u8> {
        let mut num: Vec<u8> = vec![0];
        for &c in input_chars {
            if c > 84 {
                // invalid character, skip it
                continue;
            }
            let mut carry = c as u32;
            for n in num.iter_mut() {
                let total = *n as u32 * 85 + carry;
                *n = (total & 0xff) as u8;
                carry = total >> 8;
            }

            while carry > 0 {
                num.push((carry & 0xff) as u8);
                carry >>= 8;
            }
        }
        num.reverse();
        num
    }
    /// Decodes a Base85 string into a Vec<u8>.
    pub fn decode(input: &str) -> Vec<u8> {
        let mut ret = Vec::new();
        if input.len() == 0 {
            return ret; // empty input
        }

        let mut input_chars: Vec<u8> = input
            .chars()
            .into_iter()
            .map(|x| BASE85_GIT_REVERSE_MAP[x as usize])
            .collect();
        let mut padding: u8 = 0;
        while input_chars.len() % 5 != 0 {
            input_chars.push(84); // pad with 84 to make the length a multiple of 5
            padding += 1;
        }

        let ind = input_chars.len() / 5;
        for i in 0..ind {
            let start = i * 5;
            let end = start + 5;
            let chunk = &input_chars[start..end];
            let decoded_chunk = Self::inner_decode(chunk);

            let decoded_chunk = if i == ind - 1 {
                // remove padding
                match padding {
                    0 => decoded_chunk,                // no padding
                    1 => decoded_chunk[0..3].to_vec(), // keep first 3 bytes
                    2 => decoded_chunk[0..2].to_vec(), // keep first 2 bytes
                    3 => decoded_chunk[0..1].to_vec(), // keep first 1 bytes
                    _ => unreachable!(),               // should never happen
                }
            } else {
                decoded_chunk
            };
            ret.extend(decoded_chunk);
        }
        ret
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_base85_git() {
        let test = "hello";
        // println!("{:?}", test.as_bytes());
        let output = Base85Git::encode(test.as_bytes());
        println!("{}", output);
        let output = Base85Git::decode(&output);
        let output = String::from_utf8(output).unwrap();
        println!("{:?}", output);

        let test = "中文测试";
        // println!("{:?}", test.as_bytes());
        let output = Base85Git::encode(test.as_bytes());
        println!("{}", output);
        let output = Base85Git::decode(&output);
        let output = String::from_utf8(output).unwrap();
        println!("{:?}", output);
    }
    #[test]
    fn test_ascii85() {
        let test = "hello";
        // let test = "test";
        println!("{:?}", test.as_bytes());
        let output = Ascii85::encode(test.as_bytes());
        println!("{}", output);
        let output = Ascii85::decode(&output);
        println!("{:?}", output);
        let output = String::from_utf8(output).unwrap();
        println!("{:?}", output);

        let test = "testz00";
        println!("{:?}", test.as_bytes());
        let output = Ascii85::encode(test.as_bytes());
        println!("{}", output);
        let output = Ascii85::decode(&output);
        println!("{:?}", output);
        let output = String::from_utf8(output).unwrap();
        println!("{:?}", output);

        let test = "中文测试";
        println!("{:?}", test.as_bytes());
        let output = Ascii85::encode(test.as_bytes());
        println!("{}", output);
        let output = Ascii85::decode(&output);
        println!("{:?}", output);
        let output = String::from_utf8(output).unwrap();
        println!("{:?}", output);
    }
    #[test]
    fn test_base64_url() {
        let test = "test";
        let output = Base64Url::encode(test.as_bytes(), false);
        println!("{}", output);
        let output = Base64Url::decode(&output);
        let output = String::from_utf8(output).unwrap();
        println!("{:?}", output);

        let test = "fasdfa";
        // println!("{:?}", test.as_bytes());
        let output = Base64Url::encode(test.as_bytes(), false);
        println!("{}", output);
        let output = Base64Url::decode(&output);
        let output = String::from_utf8(output).unwrap();
        println!("{:?}", output);

        let test = "中文测试";
        // println!("{:?}", test.as_bytes());
        let output = Base64Url::encode(test.as_bytes(), false);
        println!("{}", output);
        let output = Base64Url::decode(&output);
        let output = String::from_utf8(output).unwrap();
        println!("{:?}", output);
    }
    #[test]
    fn test_base62() {
        let test = "test";
        let output = Base62::encode(test.as_bytes());
        println!("{}", output);
        let output = Base62::decode(&output);
        let output = String::from_utf8(output).unwrap();
        println!("{:?}", output);

        let test = "fasdfa";
        // println!("{:?}", test.as_bytes());
        let output = Base62::encode(test.as_bytes());
        println!("{}", output);
        let output = Base62::decode(&output);
        let output = String::from_utf8(output).unwrap();
        println!("{:?}", output);

        let test = "中文测试";
        // println!("{:?}", test.as_bytes());
        let output = Base62::encode(test.as_bytes());
        println!("{}", output);
        let output = Base62::decode(&output);
        let output = String::from_utf8(output).unwrap();
        println!("{:?}", output);
    }
    #[test]
    fn test_base58() {
        let test = "test";
        let output = Base58::encode(test.as_bytes());
        println!("{}", output);
        let output = Base58::decode(&output);
        let output = String::from_utf8(output).unwrap();
        println!("{:?}", output);

        let test = "fasdfa";
        // println!("{:?}", test.as_bytes());
        let output = Base58::encode(test.as_bytes());
        println!("{}", output);
        let output = Base58::decode(&output);
        let output = String::from_utf8(output).unwrap();
        println!("{:?}", output);

        let test = "中文测试";
        // println!("{:?}", test.as_bytes());
        let output = Base58::encode(test.as_bytes());
        println!("{}", output);
        let output = Base58::decode(&output);
        let output = String::from_utf8(output).unwrap();
        println!("{:?}", output);
    }
    #[test]
    fn test_base32() {
        let test = "test";
        let output = Base32::encode(test.as_bytes());
        println!("{}", output);
        let output = Base32::decode(&output);
        let output = String::from_utf8(output).unwrap();
        println!("{:?}", output);

        let test = "fasdfa";
        // println!("{:?}", test.as_bytes());
        let output = Base32::encode(test.as_bytes());
        println!("{}", output);
        let output = Base32::decode(&output);
        let output = String::from_utf8(output).unwrap();
        println!("{:?}", output);

        let test = "中文测试";
        let output = Base32::encode(test.as_bytes());
        println!("{}", output);
        let output = Base32::decode(&output);
        let output = String::from_utf8(output).unwrap();
        println!("{:?}", output);
    }
    #[test]
    fn test_base64() {
        let test = "test";
        let output = Base64::encode(test.as_bytes());
        println!("{}", output);
        let output = Base64::decode(&output);
        let output = String::from_utf8(output).unwrap();
        println!("{:?}", output);

        let test = "fasdfa";
        let output = Base64::encode(test.as_bytes());
        println!("{}", output);
        let output = Base64::decode(&output);
        let output = String::from_utf8(output).unwrap();
        println!("{:?}", output);

        let test = "中文测试";
        let output = Base64::encode(test.as_bytes());
        println!("{}", output);
        let output = Base64::decode(&output);
        let output = String::from_utf8(output).unwrap();
        println!("{:?}", output);
    }
    #[test]
    fn script1() {
        // generate the base64 map
        let gen_map = |input: &str| {
            let mut res = Vec::new();
            for b in input.chars() {
                let fmt_str = format!("\"{}\"", b);
                res.push(fmt_str);
            }
            let res_str = res.join(", ");
            println!("[{}]", res_str);
        };

        let gen_res_map = |input: &str| {
            let mut test: Vec<u8> = vec![255; 256];
            for (i, b) in input.chars().into_iter().enumerate() {
                let b_u8 = b as u8;
                test[b_u8 as usize] = i as u8;
            }
            println!("{:?}", test);
        };

        let base64 = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
        println!(">>>>>>>>>>>>>>");
        gen_map(base64);
        println!(">>>>>>>>>>>>>>");
        gen_res_map(base64);

        let base32 = "ABCDEFGHIJKLMNOPQRSTUVWXYZ234567";
        println!(">>>>>>>>>>>>>>");
        gen_map(base32);
        println!(">>>>>>>>>>>>>>");
        gen_res_map(base32);

        let base58 = "123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz";
        println!(">>>>>>>>>>>>>>");
        gen_map(base58);
        println!(">>>>>>>>>>>>>>");
        gen_res_map(base58);

        let base62 = "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";
        println!(">>>>>>>>>>>>>>");
        gen_map(base62);
        println!(">>>>>>>>>>>>>>");
        gen_res_map(base62);

        let base64url = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-_";
        println!(">>>>>>>>>>>>>>");
        gen_map(base64url);
        println!(">>>>>>>>>>>>>>");
        gen_res_map(base64url);

        let base85git =
            "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz!#$%&()*+-;<=>?@^_`{|}~";
        println!(">>>>>>>>>>>>>>");
        gen_map(base85git);
        println!(">>>>>>>>>>>>>>");
        gen_res_map(base85git);

        let chars = base85git.chars().collect::<Vec<char>>();
        let w85 = chars[82];
        println!("w85: {}", w85);
        let w55 = chars[55];
        println!("w55: {}", w55);
    }
    #[test]
    fn shift() {
        let x: u8 = 0b00001111;
        let y: u8 = x << 2;
        println!("{}", y);
    }
}
