//! Base 64 encoder from https://github.com/JeninSutradhar/base64-Rust-Encoder-Decoder/blob/master/src/lib.rs

// This defines the 64 characters used in Base64 encoding.
const CHARSET: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

// This character is used for padding the Base64 encoded string
// when the input data is not a multiple of 3 bytes.
const PADDING: char = '=';

/// Combines two provided bytes into a u16 and collects 6 bits from it using an AND mask
///
/// Example:
///     Bytes: X and Y
///     (bits of those bytes will be signified using the names of their byte)
///     Offset: 4
///
/// 'combined' = 0bXXXXXXXXYYYYYYYY
/// AND mask:
///     0b1111110000000000 >> offset (4) = 0b0000111111000000
/// `combined` with mask applied:
///     0b0000XXYYYY000000
/// Shift the value right by (16 bit number) - (6 bit mask) - (4 offset) = 6:
/// 0b0000000000XXYYYY
/// And then turn it into a u8:
///     0b00XXYYYY (Return value)
///
/// Parameters:
/// - `from`: Takes a tuple of two bytes.
/// - `offset`: The offset value.
///
/// Combines the two bytes into a single 16-bit integer.
/// Masks and extracts 6 bits from the combined value based on the offset.
/// Returns: A single byte (u8) containing the 6 bits extracted.
fn collect_six_bits(from: (u8, u8), offset: u8) -> u8 {
    let combined: u16 = ((from.0 as u16) << 8) | (from.1 as u16);
    ((combined & (0b1111110000000000u16 >> offset)) >> (10 - offset)) as u8
}

/// Base64 encoding converts binary data into a textual representation
/// using 64 ASCII characters. Each Base64 character represents 6 bits
/// of the original binary data.
///
/// Parameters:
/// - `data`: A byte slice (`&[u8]`) of the data to be encoded.
///
/// Returns: A Base64 encoded string.
pub fn base64_encode(data: &[u8]) -> String {
    let mut encoded_string = String::new();
    let mut bits_encoded = 0usize;

    // Using modulo twice to prevent an underflow
    let padding_needed = ((6 - (data.len() * 8) % 6) / 2) % 3;
    loop {
        // Integer division
        let lower_byte_index_to_encode = bits_encoded / 8usize;
        if lower_byte_index_to_encode == data.len() {
            break;
        };

        let lower_byte_to_encode = data[lower_byte_index_to_encode];
        let upper_byte_to_encode = if (lower_byte_index_to_encode + 1) == data.len() {
            0u8
        } else {
            data[lower_byte_index_to_encode + 1]
        };

        let bytes_to_encode = (lower_byte_to_encode, upper_byte_to_encode);
        let offset: u8 = (bits_encoded % 8) as u8;
        encoded_string.push(CHARSET[collect_six_bits(bytes_to_encode, offset) as usize] as char);

        bits_encoded += 6;
    }

    for _ in 0..padding_needed {
        encoded_string.push(PADDING);
    }

    encoded_string
}
