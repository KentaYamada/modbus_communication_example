
pub fn bin_to_ascii(value: u8) -> Result<char, String> {
    if !value.is_ascii() {
        return Err(format!("Byte {} is not a valid ASCII character.", value));
    }

    // 0~9, A~FのASCIIコードかどうか判定
    if matches!(value, 0x30..=0x39 | 0x41..=0x46) {
        Ok(value as char)
    } else {
        Err(format!("Byte 0x{:X} is not within the valid range.", value))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_valid() {
        for b in 0x30..=0x39 {
            assert_eq!(bin_to_ascii((b & 0x0F) + b'0').unwrap(), b as char);

            let shifted_value = ((b >> 4) & 0x0F) + b'0';
            assert_eq!(bin_to_ascii(shifted_value).unwrap(), shifted_value as char);
        }
    }

    #[test]
    fn is_invalid() {
        for b in 0x3a..=0x40 {
            assert!(bin_to_ascii(b).is_err());
        }
    }
}
