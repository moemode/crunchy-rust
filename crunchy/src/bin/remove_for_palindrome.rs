fn is_palindrome(s: &str) -> bool {
    find_first_mismatch(s).is_none()
}

fn is_palindrome_after_removal(s: &str) -> bool {
    let s: String = s
        .to_lowercase()
        .chars()
        .filter(|c| c.is_alphanumeric())
        .collect();
    find_first_mismatch(&s).map_or_else(
        || true,
        |(l, r)| {
            find_first_mismatch(&s[l..r]).is_none()
                || find_first_mismatch(&s[l + 1..r + 1]).is_none()
        },
    )
}

fn find_first_mismatch(s: &str) -> Option<(usize, usize)> {
    let chars = s.chars();
    let left_mismatch = chars
        .clone()
        .take(s.len() / 2)
        .zip(chars.rev())
        .position(|(a, b)| a != b);
    left_mismatch.map(|l| (l, s.len() - 1 - l))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_mismatch() {
        assert_eq!(find_first_mismatch("abcba"), None);
        assert_eq!(find_first_mismatch("abcde"), Some((0, 4)));
        assert_eq!(find_first_mismatch("raceacar"), Some((3, 4)));
        assert_eq!(find_first_mismatch("abaaza"), Some((1, 4)))
    }

    #[test]
    fn test_special_chars_and_case() {
        assert!(
            is_palindrome_after_removal("abaaA!"),
            "Should be true after removing 'b' and ignoring special chars and case"
        );
    }

    #[test]
    fn test_multiple_chars_different() {
        assert!(
            !is_palindrome_after_removal("abaaza"),
            "Cannot make palindrome by removing single character"
        );
    }
}
