use std::fmt::Debug;
use std::str::FromStr;

pub fn parse_csv<I, R>(input: I) -> Vec<R>
where
    I: AsRef<str>,
    R: FromStr,
    <R as FromStr>::Err: Debug,
{
    input
        .as_ref()
        .split(',')
        .map(|v| v.trim())
        .filter(|&v| !v.is_empty())
        .map(|v| v.parse())
        .collect::<Result<_, _>>()
        .unwrap()
}

pub fn parse_line_delimited<I, R>(input: I) -> Vec<R>
where
    I: AsRef<str>,
    R: FromStr,
    <R as FromStr>::Err: Debug,
{
    input
        .as_ref()
        .lines()
        .map(|l| l.trim())
        .filter(|&l| !l.is_empty())
        .map(|l| l.parse())
        .collect::<Result<_, _>>()
        .unwrap()
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_line_delimited() {
        let lines = "1\n2\r\n3";
        let parsed = parse_line_delimited(lines);
        assert_eq!(&[1u32, 2, 3], parsed.as_slice());
    }
}