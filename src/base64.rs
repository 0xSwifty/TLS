//base64.rs

pub struct Base64;

impl Base64 {

    // base64 alphabet
    const ALPHABET: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

    // base64 padding character
    const PADDING: u8 = b'=';

    // converts to base64
    pub fn to_base64(raw: &str) -> String {
        let raw = raw.as_bytes();

        let output = raw
            .chunks(3)
            .map(|chunk| Bits::to_six_bits(chunk))
            .flat_map(|bits| Self::encode(bits));

        String::from_iter(output)
    }

    pub fn from_base64(encoded: String) -> String {
        let bytes = encoded.as_bytes();

        let filtered = bytes
            .iter()
            .filter(|s| *s != &Self::PADDING)
            .map(|s| Self::get_index(*s) as u8)
            .collect::<Vec<u8>>();

        let bits = filtered.chunks(4).flat_map(|s| Bits::to_eight_bits(s)).map(|s| s as char).collect::<Vec<char>>();

        String::from_iter(bits)
    }

    fn encode(bits: Vec<u8>) -> Vec<char> {
        let mut out = vec![Self::PADDING as char; 4];

        for i in 0..bits.len() {
            let char = Self::get_char(bits[i]);
            out[i] = char;
        }
        out
    }

    fn get_char(index: u8) -> char {

        // function expects index fFom size 0 to 63
        assert!(index < 64, "Not found on alphabet");

        let index = index as usize;
        let table = Self::ALPHABET;

        table.chars().nth(index).expect("Index from 0 to 63 is expected")
    }

    fn get_index(byte: u8) -> usize {

        let table = Self::ALPHABET;

        // returns the index of the byte within base64 alphabet
        table.find(|index| index == byte as char).expect("Index from 0 to 63 is expected")
    }
}

struct Bits;

impl Bits {

    // converts a u8 into a vector of bits
    fn to_binary(decimal: u8) -> Vec<u8> {
        let mut binary = Vec::new();
        for i in (0..8).rev() {
            let bit = (decimal >> i) & 1;
            binary.push(bit);
        }
        binary
    }

    // turns bytes from 6 bits into 8 bits
    fn to_eight_bits(bytes: &[u8]) -> Vec<u8> {
        let out = match bytes.len() {
            2 => vec![
                (bytes[0] & 0b00111111) << 2 | (bytes[1] >> 4),
                (bytes[1] & 0b00001111) << 4,
            ],
            3 => vec![
                (bytes[0] & 0b00111111) << 2 | (bytes[1] >> 4),
                (bytes[1] & 0b00001111) << 4 | (bytes[2] >> 2),
                (bytes[2] & 0b00000011) << 6,
            ],
            4 => vec![
                (bytes[0] & 0b00111111) << 2 | (bytes[1] >> 4),
                (bytes[1] & 0b00001111) << 4 | (bytes[2] >> 2),
                (bytes[2] & 0b00000011) << 6 | (bytes[3] & 0b00111111),
            ],
            _ => unreachable!(),
        };
        out.into_iter().filter(|&x| x > 0).collect()
    }

    // turns bytes from 8 bits into 6 bits
    fn to_six_bits(bytes: &[u8]) -> Vec<u8> {
        match bytes.len() {
            1 => vec![
                bytes[0] >> 2,
                (bytes[0] & 0b00000011) << 4,
            ],
            2 => vec![
                bytes[0] >> 2,
                (bytes[0] & 0b00000011) << 4 | bytes[1] >> 4,
                (bytes[1] & 0b00001111) << 2,
            ],
            3 => vec![
                bytes[0] >> 2,
                (bytes[0] & 0b00000011) << 4 | bytes[1] >> 4,
                (bytes[1] & 0b00001111) << 2 | bytes[2] >> 6,
                bytes[2] & 0b00111111,
            ],
            _ => unreachable!(),
        }
    }
}
