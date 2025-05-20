const BASE64_MAP: [&str; 64] = [
    "A", "B", "C", "D", "E", "F", "G", "H", "I", "J", "K", "L", "M", "N", "O", "P", "Q", "R", "S",
    "T", "U", "V", "W", "X", "Y", "Z", "a", "b", "c", "d", "e", "f", "g", "h", "i", "j", "k", "l",
    "m", "n", "o", "p", "q", "r", "s", "t", "u", "v", "w", "x", "y", "z", "0", "1", "2", "3", "4",
    "5", "6", "7", "8", "9", "+", "/",
];

const BASE64_REVERSE_MAP: [u8; 463] = [
    255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
    255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
    255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
    255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
    255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
    255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
    255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
    255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
    255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
    255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
    255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
    255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
    255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
    255, 255, 255, 255, 255, 255, 255, 255, 255, 62, 255, 255, 255, 63, 52, 53, 54, 55, 56, 57, 58,
    59, 60, 61, 255, 255, 255, 255, 255, 255, 255, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13,
    14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 255, 255, 255, 255, 255, 255, 26, 27, 28, 29,
    30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49, 50, 51, 255,
    255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
    255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
    255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
    255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
    255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
    255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
    255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
];

pub struct Base64;

impl Base64 {
    /// encodes the input bytes into a Base64 string.
    pub fn encode(&self, input: &[u8]) -> String {
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
    pub fn decode(&self, input: &str) -> String {
        let mut ret = Vec::new();
        let mut flag = 0;
        let mut prev: u8 = 0;

        for i in input.chars() {
            if i == '=' {
                break;
            }
            let ind = BASE64_REVERSE_MAP[i as usize];
            match flag {
                0 => {
                    prev = ind << 2;
                    flag += 1;
                }
                1 => {
                    let ind_1 = prev + (ind >> 4);
                    prev = (ind & 0b00001111) << 4;

                    ret.push(ind_1);
                    flag += 1;
                }
                2 => {
                    let ind_1 = prev + (ind >> 2);
                    prev = (ind & 0b00111111) << 6;

                    ret.push(ind_1);
                    flag += 1;
                }
                _ => {
                    ret.push(ind + prev);
                    flag = 0;
                }
            }
        }

        let ret_str = String::from_utf8_lossy(&ret).to_string();
        ret_str
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn script1() {
        let bb = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
        let mut res = Vec::new();
        for b in bb.chars() {
            let fmt_str = format!("\"{}\"", b);
            res.push(fmt_str);
        }
        let res_str = res.join(", ");
        println!("[{}]", res_str);
    }
    #[test]
    fn shift() {
        let x: u8 = 0b00001111;
        let y: u8 = x << 2;
        println!("{}", y);
    }
    #[test]
    fn base64_encode() {
        let test = "test";
        let base64 = Base64;
        let output = base64.encode(test.as_bytes());
        println!("{}", output);

        let output = base64.decode(&output);
        println!("{:?}", output);

        let test = "fasdfa";
        let base64 = Base64;
        let output = base64.encode(test.as_bytes());
        println!("{}", output);

        let test = "中文测试";
        let base64 = Base64;
        let output = base64.encode(test.as_bytes());
        println!("{}", output);
    }
}
