use std::fmt;
use std::str::FromStr;

const NUM_WORDS: usize = 64; // 64 * 64 = 4096 bits

#[derive(Clone, Debug)]
pub struct BigUint4096 {
    data: [u64; NUM_WORDS],
}

impl Default for BigUint4096 {
    fn default() -> Self {
        Self::new()
    }
}

impl BigUint4096 {
    pub fn new() -> Self {
        BigUint4096 {
            data: [0; NUM_WORDS],
        }
    }

    pub fn from_hex_str(hex_str: &str) -> Result<Self, String> {
        let hex_str = if hex_str.len() % 2 == 0 {
            hex_str.to_string()
        } else {
            format!("0{}", hex_str) // Prepend a '0' to make it even length
        };

        let mut bytes = hex::decode(hex_str).map_err(|e| format!("Hex decode error: {}", e))?;
        let mut data = [0; NUM_WORDS];

        let data_len = data.len();
        // add leading '0' for furhter fast processing
        let padding = 8 - (bytes.len() % 8);
        if padding < 8 {
            bytes = [vec![0u8; padding], bytes].concat();
        }
        if bytes.len() * 8 > data_len * 64 {
            return Err("Input string too long".to_string());
        }

        for (i, chunk) in bytes.chunks(8).enumerate() {
            let mut word = [0u8; 8];
            word.copy_from_slice(chunk);
            data[data_len - 1 - i] = u64::from_be_bytes(word);
        }

        Ok(BigUint4096 { data })
    }

    pub fn to_hex_string(&self) -> String {
        let mut hex_string = String::new();

        for &word in &self.data {
            if word == 0 {
                continue;
            }
            let bytes = word.to_be_bytes();

            hex_string.push_str(&hex::encode(bytes));
        }
        // Remove leading zeros
        hex_string.trim_start_matches('0').to_string()
    }

    // Addition
    pub fn add(&self, other: &Self) -> Self {
        let mut result = BigUint4096::new();
        let mut carry = 0;

        for i in (0..NUM_WORDS).rev() {
            let sum = self.data[i].wrapping_add(other.data[i]).wrapping_add(carry);
            result.data[i] = sum & u64::MAX; // Mask to handle overflow
            carry = if sum == u64::MAX { 1 } else { 0 }; // Calculate carry
        }

        result
    }

    // Subtraction
    pub fn sub(&self, other: &Self) -> Self {
        let mut result = BigUint4096::new();
        let mut borrow = 0;

        for i in (0..NUM_WORDS).rev() {
            let diff = self.data[i]
                .wrapping_sub(other.data[i])
                .wrapping_sub(borrow);
            result.data[i] = diff;
            borrow = (diff >> 63) & 1; // Check if the subtraction caused underflow
        }

        result
    }
}

impl FromStr for BigUint4096 {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        BigUint4096::from_hex_str(s)
    }
}

impl fmt::Display for BigUint4096 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_hex_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_hex_str_and_to_hex_string() {
        let hex_str = "123456789abcdef0";
        let big_uint = BigUint4096::from_hex_str(hex_str).unwrap();
        assert_eq!(big_uint.to_hex_string(), hex_str);
    }

    #[test]
    fn test_display_trait() {
        let hex_str = "123456789abcdef0";
        let big_uint = BigUint4096::from_hex_str(hex_str).unwrap();
        assert_eq!(format!("{}", big_uint), hex_str);
    }

    #[test]
    fn test_addition() {
        let num1 = BigUint4096::from_hex_str("1234").unwrap();
        let num2 = BigUint4096::from_hex_str("5678").unwrap();
        let result = num1.add(&num2);
        assert_eq!(result.to_hex_string(), "68ac");
    }

    #[test]
    fn test_subtraction() {
        let num1 = BigUint4096::from_hex_str("5678").unwrap();
        let num2 = BigUint4096::from_hex_str("1234").unwrap();
        let result = num1.sub(&num2);
        assert_eq!(result.to_hex_string(), "4444");
    }
}
