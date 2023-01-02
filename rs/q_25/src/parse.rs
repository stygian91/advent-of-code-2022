pub fn parse_digit(ch: char) -> Option<i8> {
    match ch {
        '=' => Some(-2),
        '-' => Some(-1),
        '0' => Some(0),
        '1' => Some(1),
        '2' => Some(2),
        _ => None,
    }
}

pub fn parse_number(input: &str) -> Option<isize> {
    let mut result = 0;

    for (i, ch) in input.chars().rev().enumerate() {
        let digit = parse_digit(ch)?;
        result += 5isize.pow(i as u32) * (digit as isize);
    }

    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_number_works() {
        assert_eq!(parse_number("1").unwrap(), 1);
        assert_eq!(parse_number("2").unwrap(), 2);
        assert_eq!(parse_number("1=").unwrap(), 3);
        assert_eq!(parse_number("1-").unwrap(), 4);
        assert_eq!(parse_number("10").unwrap(), 5);
        assert_eq!(parse_number("11").unwrap(), 6);
        assert_eq!(parse_number("12").unwrap(), 7);
        assert_eq!(parse_number("2=").unwrap(), 8);
        assert_eq!(parse_number("2-").unwrap(), 9);
        assert_eq!(parse_number("20").unwrap(), 10);
        assert_eq!(parse_number("1=0").unwrap(), 15);
        assert_eq!(parse_number("1-0").unwrap(), 20);
        assert_eq!(parse_number("1=11-2").unwrap(), 2022);
        assert_eq!(parse_number("1-0---0").unwrap(), 12345);
        assert_eq!(parse_number("1121-1110-1=0").unwrap(), 314159265);
    }

}
