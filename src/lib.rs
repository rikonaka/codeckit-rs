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
                _ => unreachable!(),
            }
        }

        match flag {
            1 => {
                ret.push_str(BASE32_MAP[prev as usize]);
                ret.push_str("======");
            }
            2 => {
                ret.push_str(BASE32_MAP[prev as usize]);
                ret.push_str("=====");
            }
            3 => {
                ret.push_str(BASE32_MAP[prev as usize]);
                ret.push_str("====");
            }
            _ => (),
        }

        ret
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

        match flag {
            1 => {
                ret.push_str(BASE64_MAP[prev as usize]);
                ret.push_str("==");
            }
            2 => {
                ret.push_str(BASE64_MAP[prev as usize]);
                ret.push_str("=");
            }
            _ => (),
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
            1 => {
                if prev != 0 {
                    ret.push(prev);
                }
            }
            2 => {
                if prev != 0 {
                    ret.push(prev);
                }
            }
            3 => {
                if prev != 0 {
                    ret.push(prev);
                }
            }
            _ => (), // ignore 0
        }
        ret
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn base64_encode() {
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
    }
    #[test]
    fn shift() {
        let x: u8 = 0b00001111;
        let y: u8 = x << 2;
        println!("{}", y);
    }
}
