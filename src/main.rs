mod my_big_number;

use my_big_number::sum;

fn main() {
    let a1 = "111";
    let b1 = "999";
    println!("{} + {} = {}", a1, b1, sum(a1, b1));

    let a2: &str = "9999999999999999999999999999999999999999";
    let b2 = "1111111111111111111111111111111111111111";
    println!("{} + {} = {}", a2, b2, sum(a2, b2));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_big_sum() {
        assert_eq!(sum("123", "456"), "579");

        assert_eq!(sum("99", "1"), "100");
    }

    #[test]
    fn test_mismatched_length_sum() {
        assert_eq!(sum("1000", "20"), "1020");
        assert_eq!(sum("20", "1000"), "1020");
    }

    #[test]
    fn test_edge_cases() {
        assert_eq!(sum("", "123"), "0");
        assert_eq!(sum("123", ""), "0");
        assert_eq!(sum("12", "34"), "46");
    }
}