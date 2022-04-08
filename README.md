string_info
===========


PURPOSE
-------
The purpose of this program is to play around with Rust, and learn how to decode UTF-8 encoded characters into unicode codepoints.


USAGE
-------------

Run the program with some regular and fancy UTF-8 encoded characters
```bash
cargo run héllo there 😀 ☺️
```

The above input will produce the following output
```bash
Word: Chars(['h', 'é', 'l', 'l', 'o'])
Char { scalar: 'h', codepoint: "U+0068", binary: ["1101000"], decimal: [104] }
Char { scalar: 'é', codepoint: "U+00E9", binary: ["11000011", "10101001"], decimal: [195, 169] }
Char { scalar: 'l', codepoint: "U+006C", binary: ["1101100"], decimal: [108] }
Char { scalar: 'l', codepoint: "U+006C", binary: ["1101100"], decimal: [108] }
Char { scalar: 'o', codepoint: "U+006F", binary: ["1101111"], decimal: [111] }

Word: Chars(['t', 'h', 'e', 'r', 'e'])
Char { scalar: 't', codepoint: "U+0074", binary: ["1110100"], decimal: [116] }
Char { scalar: 'h', codepoint: "U+0068", binary: ["1101000"], decimal: [104] }
Char { scalar: 'e', codepoint: "U+0065", binary: ["1100101"], decimal: [101] }
Char { scalar: 'r', codepoint: "U+0072", binary: ["1110010"], decimal: [114] }
Char { scalar: 'e', codepoint: "U+0065", binary: ["1100101"], decimal: [101] }

Word: Chars(['😀'])
Char { scalar: '😀', codepoint: "U+1F600", binary: ["11110000", "10011111", "10011000", "10000000"], decimal: [240, 159, 152, 128] }

Word: Chars(['☺', '\u{fe0f}'])
Char { scalar: '☺', codepoint: "U+263A", binary: ["11100010", "10011000", "10111010"], decimal: [226, 152, 186] }
Char { scalar: '\u{fe0f}', codepoint: "U+FE0F", binary: ["11101111", "10111000", "10001111"], decimal: [239, 184, 143] }
```

NOTES
-------------
This program assumes that the input is UTF-8 encoded and doesn't try to understand other UTF encodings