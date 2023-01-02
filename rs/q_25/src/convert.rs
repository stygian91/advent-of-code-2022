fn digit_to_snafu(digit: isize) -> char {
    match digit {
        -2 => '=',
        -1 => '-',
        0 => '0',
        1 => '1',
        2 => '2',
        _ => panic!("Invalid digit"),
    }
}

pub fn number_to_snafu(num: isize) -> String {
    let mut res = String::new();
    let mut quotient = num;

    if quotient == 0 {
        return "0".to_owned();
    }

    while quotient > 0 {
        let remainder = ((quotient + 2) % 5) - 2;
        quotient = (quotient + 2) / 5;
        res = format!("{}{}", digit_to_snafu(remainder), res);
    }

    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn to_snafu_works() {
        assert_eq!(number_to_snafu(0), "0");
        assert_eq!(number_to_snafu(1), "1");
        assert_eq!(number_to_snafu(2), "2");
        assert_eq!(number_to_snafu(5), "10");
        assert_eq!(number_to_snafu(6), "11");
        assert_eq!(number_to_snafu(7), "12");
        assert_eq!(number_to_snafu(10), "20");
        assert_eq!(number_to_snafu(11), "21");
        assert_eq!(number_to_snafu(12), "22");
        assert_eq!(number_to_snafu(50), "200");
        assert_eq!(number_to_snafu(8), "2=");
        assert_eq!(number_to_snafu(9), "2-");
        assert_eq!(number_to_snafu(2022), "1=11-2");
    }
}
