fn is_palindrome(s: &str) -> bool {
    let s = s.to_lowercase();
    let l = s.len() / 2;
    let chars = s.chars().filter(|c| c.is_alphanumeric());
    chars.clone().take(l).eq(chars.rev().take(l))
}

fn main() {
    println!("Valid Palindrome Checker");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_with_special_chars() {
        assert!(is_palindrome("H\"an ah!"));
    }

    #[test]
    fn test_non_palindrome() {
        assert!(!is_palindrome("Hugo"));
    }

    #[test]
    fn test_palindrome_with_spaces() {
        assert!(is_palindrome("step on no pets!"));
    }

    #[test]
    fn test_case_sensitive_palindrome() {
        let input = "Step on no pets!";
        assert!(is_palindrome(input));
    }
}
