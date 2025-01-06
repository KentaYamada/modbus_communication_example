// Modbus多項式の既定値
const CRC_DEFAULT_VALUE: u16 = 0xA001;

pub fn calc_parity8(data: u8) -> u8 {
    let mut count: u8 = 0;
    let mut current: u8 = data;

    while current > 0 {
        count += (current & 0x01) as u8;
        current <<= 1;
    }

    if count % 2 == 0 { 0 } else { 1 }
}

pub fn calc_crc16(data: &[u8]) -> (u8, u8) {
    let mut crc: u16 = 0xFFFF;

    for &byte in data {
        crc ^= byte as u16;

        for _ in 0..8 {
            crc = if crc & 0x0001 != 0 { (crc >> 1) ^ CRC_DEFAULT_VALUE } else { crc >> 1 }
        }
    }

    // 8bitに分割して、下位・上位の順の配列を返す
    let high = ((crc >> 8) & 0xFF) as u8;
    let low = (crc & 0xFF) as u8;

    (low, high)
}

pub fn calc_lrc_ascii(data: &[u8]) -> (u8, String) {
    // データを全て加算し、キャリーを無視する
    let sum: u8 = data.iter().fold(0, |acc, &byte| acc.wrapping_add(byte));
    // 2の補数を計算する
    let lrc = (!sum).wrapping_add(1);
    // LRC値をASCII文字列形式（大文字16進数）に変換
    let lrc_ascii = format!("{:02X}", lrc);

    (lrc, lrc_ascii)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calc_parity8_worked() {
        for i in 0..=10 {
            assert_eq!(calc_parity8(i), i % 2);
        }
    }

    #[test]
    fn calc_crc16_worked() {
        let data = [0x01, 0x04, 0x00, 0x00, 0x00, 0x01];
        let (low, high) = calc_crc16(&data);
        assert_eq!(low, 0x31);
        assert_eq!(high, 0xCA);
    }

    #[test]
    fn calc_lrc_ascii_worked() {
        let data = [0x01, 0x03, 0x00,  0x80, 0x00, 0x01];
        let (lrc, lrc_ascii) = calc_lrc_ascii(&data);
        assert_eq!(lrc, 0x7B);
        assert_eq!(lrc_ascii, "7B");
    }
}
