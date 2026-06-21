pub fn sum(a: &str, b: &str) -> String {
    if a.is_empty() || b.is_empty() {
        return String::from("0");
    } else {
        let chars_a: Vec<char> = a.chars().collect();
        let chars_b: Vec<char> = b.chars().collect();

        let len_a = chars_a.len();
        let len_b = chars_b.len();

        let max_len = std::cmp::max(len_a, len_b) + 1;

        let mut result_chars = vec!['0'; max_len];

        let mut i = (len_a as i32) - 1;
        let mut j = (len_b as i32) - 1;
        let mut carry = 0;
        
        for k in (0..max_len).rev() {
            if i < 0 && j < 0 && carry == 0 {
                break;
            }
            
            let mut digit_a = 0;
            let mut digit_b = 0;
            
            if i >= 0 {
                digit_a = chars_a[i as usize].to_digit(10).unwrap_or(0);
                i -= 1;
            }
            
            if j >= 0 {
                digit_b = chars_b[j as usize].to_digit(10).unwrap_or(0);
                j -= 1;
            }
            let total = digit_a + digit_b + carry;
            carry = total / 10;
            let current_digit = total % 10;

            result_chars[k] = std::char::from_digit(current_digit, 10).unwrap_or('0');
        }

        let mut result_str: String = result_chars.into_iter().collect();

        if result_str.starts_with('0') && result_str.len() > 1 {
            result_str.remove(0);
        }

        return result_str;

    }
}