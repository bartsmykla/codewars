// Preloaded:
//
// struct MorseDecoder {
//     morse_code: HashMap<String, String>,
// }
//
// MorseDecoder::new() populates the morse_code map, e.g. ".-" -> "A".

use std::collections::HashMap;

struct MorseDecoder {
    morse_code: HashMap<String, String>
}

impl MorseDecoder {
    fn new() -> Self {
        Self {
            morse_code: [
                (".-".to_string(), "A".to_string()),
                ("-...".to_string(), "B".to_string()),
                ("-.-.".to_string(), "C".to_string()),
                ("-..".to_string(), "D".to_string()),
                (".".to_string(), "E".to_string()),
                ("..-.".to_string(), "F".to_string()),
                ("--.".to_string(), "G".to_string()),
                ("....".to_string(), "H".to_string()),
                ("..".to_string(), "I".to_string()),
                (".---".to_string(), "J".to_string()),
                ("-.-".to_string(), "K".to_string()),
                (".-..".to_string(), "L".to_string()),
                ("--".to_string(), "M".to_string()),
                ("-.".to_string(), "N".to_string()),
                ("---".to_string(), "O".to_string()),
                (".--.".to_string(), "P".to_string()),
                ("--.-".to_string(), "Q".to_string()),
                (".-.".to_string(), "R".to_string()),
                ("...".to_string(), "S".to_string()),
                ("-".to_string(), "T".to_string()),
                ("..-".to_string(), "U".to_string()),
                ("...-".to_string(), "V".to_string()),
                (".--".to_string(), "W".to_string()),
                ("-..-".to_string(), "X".to_string()),
                ("-.--".to_string(), "Y".to_string()),
                ("--..".to_string(), "Z".to_string()),
                (".----".to_string(), "1".to_string()),
                ("..---".to_string(), "2".to_string()),
                ("...--".to_string(), "3".to_string()),
                ("....-".to_string(), "4".to_string()),
                (".....".to_string(), "5".to_string()),
                ("-....".to_string(), "6".to_string()),
                ("--...".to_string(), "7".to_string()),
                ("---..".to_string(), "8".to_string()),
                ("----.".to_string(), "9".to_string()),
                ("-----".to_string(), "0".to_string()),
                ("...---...".to_string(), "SOS".to_string()),
                (".-.-.-".to_string(), ".".to_string()),
                ("-.-.--".to_string(), "!".to_string()),
            ].iter().cloned().collect(),
        }
    }

    fn decode_morse(&self, encoded: &str) -> String {
        let trimmed = encoded.trim();
        let mut result = vec![];
        let mut unit = String::new();

        for (i, c) in trimmed.chars().enumerate() {
            if c == ' ' && !(&unit).is_empty() {
                result.push(unit.clone());

                unit.clear();
            } else if i == trimmed.len() - 1 {
                unit.push(c);

                result.push(unit.clone());
            } else {
                unit.push(c);
            }
        }

        result
            .iter()
            .map(|unit| {
                if unit == " " {
                    return " ".to_string();
                }

                self.morse_code.get(unit).unwrap().clone()
            })
            .collect()
    }
}

#[test]
fn test_hey_jude() {
    let decoder = MorseDecoder::new();
    assert_eq!(decoder.decode_morse(
        "       ...---... -.-.--   - .... .   --.- ..- .. -.-. -.-   -... .-. --- .-- -.   ..-. --- -..-   .--- ..- -- .--. ...   --- ...- . .-.   - .... .   .-.. .- --.. -.--   -.. --- --. .-.-.-    "),
           "SOS! THE QUICK BROWN FOX JUMPS OVER THE LAZY DOG."
    );
    assert_eq!(decoder.decode_morse(".... . -.--   .--- ..- -.. ."), "HEY JUDE");
}