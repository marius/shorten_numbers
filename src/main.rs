use std::cmp;
use std::io::{stdin, BufRead};
use structopt::StructOpt;
use onig::*;

#[derive(Debug, StructOpt)]
#[structopt(name = "short_numbers", about = "A filter to shorten numbers")]
struct Opt {
    /// Use SI suffixes to shorten the numbers
    #[structopt(short = "s", long = "suffix")]
    suffix: bool,
}

const SUFFIXES: [&str; 9] = ["", "k", "M", "G", "T", "P", "E", "Z", "Y"];

fn convert(number: &str) -> String {
    let mut number: f64 = number.parse().unwrap();

    /* Cap exponent to highest suffix (Y = 8). */
    let exp = cmp::min((number.log10() / 3.0) as u32, 8);

    /* Reduce number by 10^exp to match the suffix. */
    for _ in 0..exp {
        number /= 1000.0;
    }

    format!("{:.0}{}", number, SUFFIXES[exp as usize])
}

fn main() {
    let opt = Opt::from_args();

    if opt.suffix {
        let re = Regex::new(r"(\d+(\.\d+)?)").unwrap();
        for line in stdin().lock().lines() {
            let l = line.unwrap();
            println!("{}", re.replace_all(&l, |caps: &Captures| {
                convert(caps.at(1).unwrap())
            }));
        }
    } else {
        let re = Regex::new(r"(\d)(?=(\d\d\d)+(?!\d))").unwrap();
        for line in stdin().lock().lines() {
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
        assert_eq!(convert("10000000000000"), "10T");
        assert_eq!(convert("50000000000000000"), "50P");
        assert_eq!(convert("1000000000000000000"), "1E");
        assert_eq!(convert("7000000000000000000000"), "7Z");
        assert_eq!(convert("123400000000000000000000"), "123Z");
        assert_eq!(convert("99000000000000000000000000"), "99Y");
        assert_eq!(convert("9876543210000000000000000000"), "9877Y");
    }
}
