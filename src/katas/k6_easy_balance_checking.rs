#![allow(unused)]

/*
    Kata (6kyu): Easy Balance Checking
    Url: https://www.codewars.com/kata/easy-balance-checking/train/rust

    You are given a (small) check book as a - sometimes - cluttered
    (by non-alphanumeric characters) string:

    "1000.00
    125 Market 125.45
    126 Hardware 34.95
    127 Video 7.45
    128 Book 14.32
    129 Gasoline 16.10"

    The first line shows the original balance. Each other line (when not blank)
    gives information: check number, category, check amount.

    First you have to clean the lines keeping only letters, digits,
    dots and spaces.

    Then return a report as a string (underscores show spaces -- don't put them
    in your solution. They are there so you can see them and how many of them
    you need to have):

    "Original_Balance:_1000.00
    125_Market_125.45_Balance_874.55
    126_Hardware_34.95_Balance_839.60
    127_Video_7.45_Balance_832.15
    128_Book_14.32_Balance_817.83
    129_Gasoline_16.10_Balance_801.73
    Total_expense__198.27
    Average_expense__39.65"

    On each line of the report you have to add the new balance and then
    in the last two lines the total expense and the average expense.
    So as not to have a too long result string we don't ask for a properly
    formatted result.

    Notes
        It may happen that one (or more) line(s) is (are) blank.
        Round to 2 decimals your results.
        The line separator of results may depend on the language \n or \r\n.
        See examples in the "Sample tests".
*/

use std::error::Error;
use std::num::ParseFloatError;

fn parse_first_line(line: &str) -> Result<f64, ParseFloatError> {
    let mut cleaned = String::new();

    for b in line.bytes() {
        if b.is_ascii_digit() || b == b'.' {
            cleaned.push(b as char);
        }
    }

    cleaned.parse()
}

type Line = (String, String, f64, f64);

fn parse_line(line: &str, balance: f64) -> Result<Line, Box<Error>> {
    let mut check_number = String::new();
    let mut category = String::new();

    let mut curr_word = String::new();

    for b in line.bytes() {
        if b.is_ascii_alphanumeric() || b == b'.' {
            curr_word.push(b as char);
        } else if b.is_ascii_whitespace() {
            if check_number.is_empty() {
                check_number = curr_word.clone();
            } else if category.is_empty() {
                category = curr_word.clone();
            }

            curr_word.clear();
        }
    }

    let mut check_amount: f64 = curr_word.parse()?;

    Ok((check_number, category, check_amount, balance - check_amount))
}

fn balance(book: &str) -> String {
    let lines: Vec<_> = book.lines()
        .filter(|line| !line.is_empty())
        .collect();

    let (first_line, rest_lines) = lines.split_at(1);

    let mut balance = parse_first_line(first_line[0]).unwrap();
    let mut total_expense = 0_f64;
    let mut expenses = Vec::new();

    let mut result = vec![format!("Original Balance: {:.2}", balance)];

    for &line in rest_lines {
        let (
            check_number,
            category,
            check_amount,
            balance_after,
        ) = parse_line(line, balance).unwrap();

        balance = balance_after;
        expenses.push(check_amount);
        total_expense += check_amount;

        let formatted_line = format!(
            "{} {} {:.2} Balance {:.2}",
            check_number,
            category,
            check_amount,
            balance_after,
        );
        result.push(formatted_line);
    }

    let total_expense = format!("Total expense  {:.2}", total_expense);
    let average_expense = format!(
        "Average expense  {:.2}",
        expenses.iter().sum::<f64>() / expenses.len() as f64,
    );

    result.push(total_expense);
    result.push(average_expense);

    result.join("\n")
}

fn dotest(book: &str, exp: &str) {
    assert_eq!(balance(book), exp);
}

#[test]
fn basic_tests() {
    let b1 = r#"
1000.00!=

125 Market !=:125.45
126 Hardware =34.95
127 Video! 7.45
128 Book :14.32
129 Gasoline ::16.10
"#;
    let b2 = r#"
1233.00
125 Hardware;! 24.8?;
123 Flowers 93.5
127 Meat 120.90
120 Picture 34.00
124 Gasoline 11.00
123 Photos;! 71.4?;
122 Picture 93.5
132 Tyres;! 19.00,?;
129 Stamps 13.6
129 Fruits{} 17.6
129 Market;! 128.00?;
121 Gasoline;! 13.6?;
"#;

    let b1sol="Original Balance: 1000.00\n125 Market 125.45 Balance 874.55\n126 Hardware 34.95 Balance 839.60\n127 Video 7.45 Balance 832.15\n128 Book 14.32 Balance 817.83\n129 Gasoline 16.10 Balance 801.73\nTotal expense  198.27\nAverage expense  39.65";
    let b2sol="Original Balance: 1233.00\n125 Hardware 24.80 Balance 1208.20\n123 Flowers 93.50 Balance 1114.70\n127 Meat 120.90 Balance 993.80\n120 Picture 34.00 Balance 959.80\n124 Gasoline 11.00 Balance 948.80\n123 Photos 71.40 Balance 877.40\n122 Picture 93.50 Balance 783.90\n132 Tyres 19.00 Balance 764.90\n129 Stamps 13.60 Balance 751.30\n129 Fruits 17.60 Balance 733.70\n129 Market 128.00 Balance 605.70\n121 Gasoline 13.60 Balance 592.10\nTotal expense  640.90\nAverage expense  53.41";

    dotest(b1, b1sol);
    dotest(b2, b2sol);
}
