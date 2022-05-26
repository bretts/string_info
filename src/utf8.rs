pub mod char_decoder {
    use crate::decoded;

    /// Decodes a UTF-8 char into a queryable struct
    pub fn decode(c: char) -> decoded::Char {
        let mut binary_formated_bytes: Vec<String> = vec![];
        let mut decimal_formated_bytes: Vec<u8> = vec![];

        c.to_string().as_bytes().into_iter().for_each(|byte| {
            binary_formated_bytes.push(format!("{:b}", byte));
            decimal_formated_bytes.push(byte.to_owned());
        });

        let decoded = decoded::Char {
            scalar: c,
            codepoint: utf8_bytes_to_unicode_code_point(c.to_string().as_bytes()),
            binary: binary_formated_bytes,
            decimal: decimal_formated_bytes,
        };

        decoded
    }

    /// Translates UTF-8 bytes representing a char into the corresponding unicode code point
    fn utf8_bytes_to_unicode_code_point(bytes: &[u8]) -> String {
        if bytes.len() == 1 {
            // When the UTF-8 encoded character is one byte, it works the same way as a 7 bit ASCII byte
            String::from(format!("U+{:04X}", bytes.get(0).unwrap()))
        } else {
            // When UTF-8 encoded characters use more than one byte, then it has the following format:
            //
            // a variable number of bytes upto a maximum of 4
            //
            // byte 1 (leading byte): The first 4 bits are structural indicating how many continuation bytes there are
            // byte 2 (continuation byte): The first 2 bits are structural
            // byte 3 (continuation byte): The first 2 bits are structural
            // byte 4 (continuation byte): The first 2 bits are structural
            //
            // There a total of 21 non structural bits that can be used to represent values

            let mut non_structural_bytes: Vec<String> = vec![];

            for (index, element) in bytes.iter().enumerate() {
                let byte_string = format!("{:b}", element);

                if index == 0 {
                    non_structural_bytes.push(byte_string.get(4..=7).unwrap().to_owned());
                } else {
                    non_structural_bytes.push(byte_string.get(2..=7).unwrap().to_owned());
                }
            }

            let non_structural_bytes = non_structural_bytes.join("");
            let non_structural_bytes_as_decimal: isize =
                isize::from_str_radix(&non_structural_bytes, 2).unwrap();

            String::from(format!("U+{:04X}", non_structural_bytes_as_decimal))
        }
    }


    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_utf8_bytes_to_unicode_code_point_works_with_single_byte_scalars() {
            // This is a single 7 bit byte
            let string = utf8_bytes_to_unicode_code_point('h'.to_string().as_bytes());
            assert_eq!("U+0068".to_string(), string);
        }

        #[test]
        fn test_utf8_bytes_to_unicode_code_point_works_with_multibyte_scalars() {
            // This scalar is a 2 byte sequence
            let string = utf8_bytes_to_unicode_code_point('é'.to_string().as_bytes());
            assert_eq!("U+00E9".to_string(), string);
        }

        #[test]
        fn test_utf8_bytes_to_unicode_code_point_works_with_emoji() {
            // This emoji is a 4 byte sequence
            let string = utf8_bytes_to_unicode_code_point('😀'.to_string().as_bytes());
            assert_eq!("U+1F600".to_string(), string);
        }

        #[test]
        fn test_utf8_bytes_to_unicode_code_point_works_with_emoji_that_uses_continuation_sequences() {
            // This code point is composed of two code points. The first 3 byte sequence is the base glyph for “White Smiling Face” and the second 3 byte sequence is the variation sequence
            let string = utf8_bytes_to_unicode_code_point("☺️".to_string().as_bytes());
            assert_eq!("U+98EAFE0F".to_string(), string);
        }
    }

}