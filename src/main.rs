use std::env;
use std::io::{self, BufRead};
use onig::*;

const SUFFIXES: [&str; 9] = ["", "k", "M", "G", "T", "P", "E", "Z", "Y"];

fn convert(number: &str) -> String {
    let num: f64 = number.parse().unwrap();
    let mut exp = (num.log10() / 3.0) as u32;
    /* Cap exponent to highest suffix. */
    if exp > 8 {
        exp = 8;
    }
    let short = num / 1_000u64.pow(exp) as f64;
    format!("{:.0}{}", short, SUFFIXES[exp as usize])
}

fn main() {
    let mut suffix_mode = false;
    if let Some(arg) = env::args().nth(1) {
        if arg == "-s" {
            suffix_mode = true;
        }
    }

    if suffix_mode {
        let re = Regex::new(r"(\d+(\.\d+)?)").unwrap();
        for line in io::stdin().lock().lines() {
            let l = line.unwrap();
            println!("{}", re.replace_all(&l, |caps: &Captures| {
                convert(caps.at(1).unwrap())
            }));
        }
    } else {
        let re = Regex::new(r"(\d)(?=(\d\d\d)+(?!\d))").unwrap();
        for line in io::stdin().lock().lines() {
            let l = line.unwrap();
            println!("{}", re.replace_all(&l, |caps: &Captures| {
                format!("{}_", caps.at(1).unwrap())
            }));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_convert() {
        assert_eq!(convert("2"), "2");
        assert_eq!(convert("12"), "12");
        assert_eq!(convert("123"), "123");
        assert_eq!(convert("1234"), "1k");
        assert_eq!(convert("5432"), "5k");
        assert_eq!(convert("12345"), "12k");
        assert_eq!(convert("54321"), "54k");
        assert_eq!(convert("1234000"), "1M");
        assert_eq!(convert("7654321"), "8M");
        assert_eq!(convert("87654321"), "88M");
        assert_eq!(convert("987654321"), "988M");
        assert_eq!(convert("987000000"), "987M");
        assert_eq!(convert("1000000000"), "1G");
        assert_eq!(convert("9000000000"), "9G");
        assert_eq!(convert("9876543210"), "10G");
        assert_eq!(convert("10476543210"), "10G");
    }
}
