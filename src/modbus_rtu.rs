use crate::error_detection::calc_crc16;


/// Modbus RTU send message frame
#[derive(Debug)]
pub struct RTUSendMessage {
    /// スレーブアドレス
    slave_address: u8,

    /// ファンクションコード
    function_code: u8,

    /// 開始アドレス(上位)
    upper_start_address: u8,

    /// 開始アドレス(下位)
    lower_start_address: u8,

    /// レジスタ数(上位)
    upper_number_of_registers: u8,

    /// レジスタ数(下位)
    lower_number_of_registers: u8,
}

impl RTUSendMessage {
    pub fn new(
        slave_address: u8,
        function_code: u8,
        upper_start_address: u8,
        lower_start_address: u8,
        upper_number_of_registers: u8,
        lower_number_of_registers: u8) -> Self {
        Self {
            slave_address,
            function_code,
            upper_start_address,
            lower_start_address,
            upper_number_of_registers,
            lower_number_of_registers
        }
    }

    /// 送信電文生成
    pub fn build_send_message(&self) -> Vec<u8> {
        let mut message: Vec<u8> = vec![
            self.slave_address,
            self.function_code,
            self.upper_start_address,
            self.lower_start_address,
            self.upper_number_of_registers,
            self.lower_number_of_registers,
        ];

        // CRCコード付加
        let (low, high) = calc_crc16(&message);
        message.push(low);
        message.push(high);

        message
    }

    /// 電文ダンプ
    pub fn dump_message(&self, message: &[u8]) -> String {
        let dump_message: String = message.iter().map(|byte| format!("0x{:02X}", byte)).collect::<Vec<_>>().join(", ");

        dump_message
    }

    /// 受信電文解析
    pub fn receive_message(&self, message: &[u8], size: usize) -> bool {
        if message.len() != size {
            // todo: throw message size error
            return false;
        }

        let message_len = message.len() - 2; // CRCコード(2bytes)除いたメッセージサイズ
        let buffers = message[0..message_len].to_vec();

        let (low, high) = calc_crc16(&buffers);
        let recieve_crc_low = message[message.len()-2];
        let recieve_crc_high = message[message.len()-1];

        if low != recieve_crc_low || high != recieve_crc_high {
            // todo: throw CRC check error
            return false;
        }

        return true;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn build_message_worked() {
        (1..=10).collect::<Vec<u8>>().iter().for_each(|&slave| {
            let query = [slave, 0x03, 0x00, 0x6B, 0x00, 0x03];
            let (low, high) = calc_crc16(&query);

            let rtu = RTUSendMessage::new(slave, 0x03, 0x00, 0x6B, 0x00, 0x03);
            let message = rtu.build_send_message();

            assert_eq!(&message[0..6], query);
            assert_eq!(message[6], low);
            assert_eq!(message[7], high);
        });
    }

    #[test]
    fn dump_message_worked() {
        (1..=10).collect::<Vec<u8>>().iter().for_each(|&slave| {
            let mut query = [slave, 0x03, 0x00, 0x6B, 0x00, 0x03].to_vec();
            let (low, high) = calc_crc16(&query);

            query.push(low);
            query.push(high);

            let expected: String = query.iter().map(|byte| format!("0x{:02X}", byte)).collect::<Vec<_>>().join(", ");

            let rtu = RTUSendMessage::new(slave, 0x03, 0x00, 0x6B, 0x00, 0x03);
            let dump_message = rtu.dump_message(&query);

            assert_eq!(dump_message, expected);
        });
    }

    #[test]
    fn recieve_message_worked() {
        let mut message = [0x06, 0x03, 0x06, 0x03, 0xE8, 0x01, 0xF4, 0x00, 0x0A].to_vec();
        let (low, high) = calc_crc16(&message);
        message.push(low);
        message.push(high);

        let rtu = RTUSendMessage::new(0x01, 0x03, 0x00, 0x6B, 0x00, 0x03);
        let is_valid = rtu.receive_message(&message, message.len());
        assert_eq!(is_valid, true);
    }
}
