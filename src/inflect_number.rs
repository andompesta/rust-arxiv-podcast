use lazy_static::lazy_static;
use regex::Captures;
use regex::Regex;

const NEGATIVE: &str = "minus";
const BASE_NUMBERS: [&str; 20] = [
    "zero",
    "one",
    "two",
    "three",
    "four",
    "five",
    "six",
    "seven",
    "eight",
    "nine",
    "ten",
    "eleven",
    "twelve",
    "thirteen",
    "fourteen",
    "fifteen",
    "sixteen",
    "seventeen",
    "eighteen",
    "nineteen",
];

const TENS: [&str; 8] = [
    "twenty", "thirty", "forty", "fifty", "sixty", "seventy", "eighty", "ninety",
];

const HUNDRED: &str = "hundred";

const THOUSANDS: [&str; 6] = [
    "thousand",
    "million",
    "billion",
    "trillion",
    "quadrillion",
    "quintillion",
    // TODO: Overflow in current algorithm.
    //  We'll need to use a u128 to go higher.
    //  But that seems like overkill.
    //"sextillion",
    //"septillion",
    //"octillion",
    //"nonillion",
];

lazy_static! {
    static ref COMMA_NUMBER: Regex = Regex::new(r"([0-9][0-9,]+[0-9])").unwrap();
    static ref POUND: Regex = Regex::new(r"£([0-9,]*[0-9]+)").unwrap();
    static ref DOLLAR: Regex = Regex::new(r"\$([0-9,]*[0-9]+)").unwrap();
    static ref DECIMAL_NUMBER: Regex = Regex::new(r"([0-9]+.[0-9]+)").unwrap();
    static ref ORDINAL_NUMBER: Regex = Regex::new(r"[0-9]+(st|nd|rd)").unwrap();
    static ref ORDINAL_NUMBER_2: Regex = Regex::new(r"([0-9]+)(th)").unwrap();
    static ref NUMBER: Regex = Regex::new(r"[0-9]+").unwrap();
}

#[derive(Debug, Clone)]
pub enum InflectError {
    ConversionError,
    ParsingError,
}

pub fn normalize_number(text: &str) -> Result<String, InflectError> {
    let text = COMMA_NUMBER.replace_all(text, |caps: &Captures| caps[1].replace(",", ""));
    
    // expand pounds
    let text = POUND.replace_all(text.as_ref(), |caps: &Captures| format!("{} pounds", &caps[1]));

    // expand dollars
    let text = DOLLAR.replace_all(text.as_ref(), |caps: &Captures| {
        let parts: Vec<&str> = caps[1].split(".").collect();

        if parts.len() > 2 {
            return format!("{} dollars", &caps[1]);
        }

        let dollars = parts
            .first()
            .unwrap_or(&"0")
            .parse::<u32>()
            .expect("error parsing dollar component, not a number");

        let cents = parts
            .get(1)
            .unwrap_or(&"0")
            .parse::<u32>()
            .expect("error parsing cents component, not a number");

        if dollars > 0 && cents > 0 {
            return format!("{} dollars {} cents ", dollars, cents);
        } else if dollars > 0 && cents == 0 {
            return format!("{} dollars", dollars);
        } else if dollars == 0 && cents > 0 {
            return format!("{} cents", cents);
        } else {
            return "zero dollars".to_string();
        }
    });

    // expand decimals number
    let text = DECIMAL_NUMBER.replace_all(text.as_ref(), |caps: &Captures| caps[1].replace(".", " point "));

    // expand decimal number
    let text = ORDINAL_NUMBER.replace_all(text.as_ref(), |caps: &Captures| {
        let inflect_number = match &caps[0] {
            "1st" => "first",
            "2nd" => "second",
            "3rd" => "theerd",
            _ => "",
        };
        return inflect_number.to_string().clone();
    });

    let text = ORDINAL_NUMBER_2.replace_all(&text.as_ref(), |caps: &Captures| {
        let mut number = convert_number(caps[1].parse::<i64>().unwrap());
        if number == "five" {
            number = "fif".to_string();
        }
        format!("{}th", number)
    });

    let text = NUMBER.replace_all(text.as_ref(), |caps: &Captures| {
        convert_number(caps[0].parse::<i64>().unwrap())
    });

    return Ok(text.as_ref().to_string());
}

pub fn convert_number(num: i64) -> String {
    let mut words: Vec<String> = Vec::new();

    if num < 0 {
        words.push(NEGATIVE.to_string());
    }

    let num = num.abs() as u64;
    if num < 100 {
        convert_nn(num, &mut words).unwrap()
    } else if num < 1000 {
        convert_nnn(num, &mut words).unwrap()
    } else {
        convert_large(num, &mut words).unwrap()
    };

    words.join(" ")
}

fn convert_nn(number: u64, words: &mut Vec<String>) -> Result<(), InflectError> {
    if number < 20 {
        let word = BASE_NUMBERS[number as usize].to_string();
        words.push(word);
        return Ok(());
    }

    for (i, word) in TENS.iter().enumerate() {
        let dval = 10 * i + 20;
        if dval + 10 > number as usize {
            if (number as usize % 10) != 0 {
                let first = word.to_string();
                let second = BASE_NUMBERS[(number % 10) as usize].to_string();

                words.push(first);
                words.push(second);
                return Ok(());
            }
            words.push(word.to_string());
            return Ok(());
        }
    }

    return Err(InflectError::ConversionError); // Should not be reached.
}

fn convert_nnn(number: u64, words: &mut Vec<String>) -> Result<(), InflectError> {
    let rem = number / 100;
    if rem > 0 {
        words.push(BASE_NUMBERS[rem as usize].to_string());
        words.push(HUNDRED.to_string());
    }

    let md = number % 100;
    if md > 0 {
        return convert_nn(md, words);
    }
    return Ok(());
}

fn convert_large(number: u64, words: &mut Vec<String>) -> Result<(), InflectError> {
    if number < 1000 {
        return convert_nnn(number, words);
    }

    // Iterate backwards, largest magnitudes first.
    for (i, unit) in THOUSANDS.iter().rev().enumerate() {
        let i = THOUSANDS.len() - i; // shadow
        let mag = 1000u64.pow(i as u32);

        if number < mag {
            continue;
        }

        let quo = number / mag;
        let rem = number - (quo * mag);

        convert_nnn(quo, words).unwrap();

        words.push(unit.to_string());

        if rem > 0 {
            return convert_large(rem, words);
        }
        return Ok(());
    }

    return Err(InflectError::ConversionError); // Should not be reached.
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn generics_for_all_integer_types_work() {
        // Test that the generics traits from the 'num' crate do what we want.
        assert_eq!("eight", convert_number(8));
        assert_eq!("minus eight", convert_number(-8));
        assert_eq!("sixteen", convert_number(16));
        assert_eq!("minus sixteen", convert_number(-16));
        assert_eq!("thirty two", convert_number(32));
        assert_eq!("minus thirty two", convert_number(-32));
        assert_eq!("sixty four", convert_number(64));
        assert_eq!("minus sixty four", convert_number(-64));
    }

    #[test]
    fn positive_integers() {
        // Tens + Ones
        assert_eq!("twenty one", convert_number(21));
        assert_eq!("thirty five", convert_number(35));
        assert_eq!("forty four", convert_number(44));
        assert_eq!("fifty five", convert_number(55));
        assert_eq!("sixty six", convert_number(66));

        // Large with small prefix
        assert_eq!("nine thousand", convert_number(9_000));
        assert_eq!("ten million", convert_number(10_000_000));
        assert_eq!("sixty billion", convert_number(60_000_000_000));
        assert_eq!("four hundred forty four trillion", convert_number(444_000_000_000_000));
    }

    #[test]
    fn negative_integers() {
        // Misc negative numbers.
        assert_eq!("minus one", convert_number(-1));
        assert_eq!("minus nine thousand one", convert_number(-9_001));
        assert_eq!("minus four hundred forty four trillion", convert_number(-444_000_000_000_000));
    }

    #[test]
    fn normalize_number_test() {
        // dollars
        let text_with_normalized_nummber = normalize_number("I have $250 in my pocket.").unwrap();
        assert_eq!("I have two hundred fifty dollars in my pocket.", text_with_normalized_nummber.as_str());

        // decimal
        let text_with_normalized_nummber = normalize_number("I have increase my weight by 0.50 kg.").unwrap();
        assert_eq!("I have increase my weight by zero point fifty kg.", text_with_normalized_nummber.as_str());

        // ordinal
        let text_with_normalized_nummber = normalize_number("I finished 5th overall").unwrap();
        assert_eq!("I finished fifth overall", text_with_normalized_nummber.as_str());

        // ordinal
        let text_with_normalized_nummber = normalize_number("I finished 6th overall").unwrap();
        assert_eq!("I finished sixth overall", text_with_normalized_nummber.as_str());
    }
}