const BASE64_MAP: [&str; 64] = [
    "A", "B", "C", "D", "E", "F", "G", "H", "I", "J", "K", "L", "M", "N", "O", "P", "Q", "R", "S",
    "T", "U", "V", "W", "X", "Y", "Z", "a", "b", "c", "d", "e", "f", "g", "h", "i", "j", "k", "l",
    "m", "n", "o", "p", "q", "r", "s", "t", "u", "v", "w", "x", "y", "z", "0", "1", "2", "3", "4",
    "5", "6", "7", "8", "9", "+", "/",
];

pub struct Base64;

impl Base64 {
    fn encode(&self, input: &[u8]) -> String {
        let mut ret = String::new();
        let mut flag = 0;

        let mut prev: u8 = 0;
        for i in input {
            match flag {
                0 => {
                    // 1111 1111 => 11 111100 & 0000 0011
                    // 6 & 2
                    let mask: u8 = 0b00000011;
                    let ind = i >> 2;
                    prev = (i & mask) << 4;

                    let c = BASE64_MAP[ind as usize];
                    ret += c;
                    flag += 1;
                }
                1 => {
                    // 1111 1111 => 1111 0000 & 0000 1111
                    // 4 & 4
                    let mask: u8 = 0b00001111;
                    let ind = prev + i >> 4;
                    prev = (i & mask) << 2;

                    let c = BASE64_MAP[ind as usize];
                    ret += c;
                    flag += 1;
                }
                2 => {
                    // 1111 1111 => 1100 0000 & 0011 1111
                    // 2 & 6
                    let mask: u8 = 0b00111111;
                    let ind = prev + (i >> 6);
                    let ind_1 = i & mask;
                    prev = 0;

                    let c = BASE64_MAP[ind as usize];
                    ret += c;
                    let c = BASE64_MAP[ind_1 as usize];
                    ret += c;
                    flag = 0;
                }
                _ => (),
            }
        }

        match flag {
            1 => {
                let c = BASE64_MAP[prev as usize];
                ret += c;
                ret += "==";
            }
            2 => {
                let c = BASE64_MAP[prev as usize];
                ret += c;
                ret += "=";
            }
            _ => (),
        }

        ret
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
        // let test = "test";
        // let base64 = Base64;
        // let output = base64.encode(test.as_bytes());
        // println!("{}", output);

        let test = "fasdfa";
        let base64 = Base64;
        let output = base64.encode(test.as_bytes());
        println!("{}", output);

        // let test = "中文测试";
        // let base64 = Base64;
        // let output = base64.encode(test.as_bytes());
        // println!("{}", output);
        // 5Lit5LaH5NWL6K+V
        // 5Lit5paH5rWL6K+V
    }
}
