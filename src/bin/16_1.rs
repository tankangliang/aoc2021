use aoc2021::input_data;

struct ByteStream {
    bytes: Vec<u8>,
    index: usize,
    bit_index: usize,

    version_numbers: usize
}

impl ByteStream {
    fn from_hex(hex_str: &str) -> Self {
        use std::str;

        let bytes = hex_str.as_bytes()
            .chunks(2)
            .map(str::from_utf8)
            .map(|s| s.unwrap())
            .map(|s| u8::from_str_radix(s, 16).unwrap())
            .collect::<Vec<u8>>();

        Self {
            bytes,
            index: 0,
            bit_index: 0,
            version_numbers: 0
        }
    }

    fn take_one(&mut self) -> u8 {
        let byte = self.bytes[self.index];
        let bit = (byte >> (7-self.bit_index)) & 1;
        if self.bit_index < 7 {
            self.bit_index += 1;
        } else {
            self.bit_index = 0;
            self.index += 1;
        }
        bit
    }

    fn take(&mut self, num_of_bits: usize) -> Vec<u8> {
        let mut bytes = Vec::new();
        let mut current_bits_taken = 0;
        let mut current_byte = 0;

        for _ in 0..num_of_bits {

            current_byte <<= 1;
            let bit = self.take_one();
            current_bits_taken += 1;
            current_byte |= bit;

            if current_bits_taken == 8 {
                bytes.push(current_byte);
                current_bits_taken = 0;
                current_byte = 0;
            }
        }

        if current_bits_taken != 0 {
            bytes.push(current_byte);
        }

        bytes
    }

    fn position(&self) -> usize {
        self.index * 8 + self.bit_index
    }

    fn parse_packet(&mut self) {
        let version = self.take(3)[0];
        self.version_numbers += version as usize;
        let type_id = self.take(3)[0];

        match type_id {
            0b100 => {self.parse_literal();},
            _ => self.parse_operator()
        };
    }

    fn parse_literal(&mut self) -> u64 {
        let mut bits_taken = 6;

        loop {
            let group = self.take(5)[0];
            bits_taken += 5;
            if group & 0b10000 == 0b00000 {
                break
            }
        }
        1
    }

    fn parse_operator(&mut self) {
        let length_type = self.take_one();

        match length_type {
            0 => self.parse_length_zero(),
            _ => self.parse_length_one()
        }
    }

    fn parse_length_zero(&mut self) {
        let mut length = {
            let bytes = self.take(15);
            let mut l = 0_usize;
            l += bytes[0] as usize;
            l <<= 7;
            l += bytes[1] as usize;
            l
        };

        let mut position = self.index * 8 + self.bit_index;

        while length > 0 {
            self.parse_packet();
            let new_position = self.index * 8 + self.bit_index;
            length -= new_position - position;
            position = new_position;
        }
    }

    fn parse_length_one(&mut self) {
        let length = {
            let bytes = self.take(11);
            let mut l = 0_usize;
            l += bytes[0] as usize;
            l <<= 3;
            l += bytes[1] as usize;
            l
        };
        for packet in 0..length {
            self.parse_packet()
        }
    }
}

fn main() {
    let content = input_data(0);
    let mut bs = ByteStream::from_hex(&content);

    bs.parse_packet();
    println!("{}", bs.version_numbers);
}
