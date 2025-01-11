/// Finds all unique triplets in the input array that sum to zero.
///
/// # Arguments
///
/// * `numbers` - A mutable vector of integers that will be sorted during processing
///
/// # Returns
///
/// A vector of tuples containing triplets (a, b, c) where:
/// * a + b + c = 0
/// * The triplets are sorted lexicographically in ascending order
/// * Each triplet is sorted in ascending order
/// * Duplicates are removed
///
/// # Examples
///
/// ```
/// let mut nums = vec![-1, 0, -2, 1, 3, 2];
/// let result = sum_of_three(&mut nums);
/// assert_eq!(result, vec![(-2, -1, 3), (-2, 0, 2), (-1, 0, 1)]);
///
/// let mut nums_with_duplicates = vec![-2, -1, 0, 0, 2, -1, 2, -2];
/// let result = sum_of_three(&mut nums_with_duplicates);
/// assert_eq!(result, vec![(-2, 0, 2), (-1, -1, 2)]);
/// ```
fn sum_of_three(numbers: &mut Vec<i64>) -> Vec<(i64, i64, i64)> {
    let mut zero_sums = vec![];
    numbers.sort_unstable();
    let mut last_i = None;
    for (idx, &i) in numbers.iter().enumerate() {
        // we have already found all triplets starting with i
        if last_i == Some(i) {
            continue;
        }
        // if the smallest number is greater than 0, the sum is larger than 0
        if i > 0 {
            break;
        }
        zero_sums.extend(find_triplets(-i, &numbers[(idx + 1)..]));
        last_i = Some(i);
    }
    zero_sums
}

/// Finds all unique pairs in the input array that sum to the goal sum.
///
/// # Arguments
///
/// * `goal_sum` - The target sum to find pairs for
/// * `numbers` - A slice of integers that MUST be sorted in ascending order
///
/// # Returns
///
/// A vector of tuples (-goal_sum, a, b) where -goal_sum + a + b = 0, with
/// (a, b) being the pairs found in the numbers slice that sum to goal_sum
fn find_triplets(goal_sum: i64, numbers: &[i64]) -> Vec<(i64, i64, i64)> {
    let mut triplets = vec![];
    let (mut l, mut r) = (0, numbers.len().saturating_sub(1));
    while l < r {
        let sum = numbers[l] + numbers[r];
        match sum.cmp(&goal_sum) {
            std::cmp::Ordering::Less => l += 1,
            std::cmp::Ordering::Equal => {
                triplets.push((-goal_sum, numbers[l], numbers[r]));
                while l < r && numbers[l] == numbers[l + 1] {
                    l += 1;
                }
                while l < r && numbers[r - 1] == numbers[r] {
                    r -= 1;
                }
                l += 1;
                r -= 1;
            }
            std::cmp::Ordering::Greater => r -= 1,
        }
    }
    triplets
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_single() {
        let mut numbers = vec![2, 2, -4];
        let expected = vec![(-4, 2, 2)];
        assert_eq!(sum_of_three(&mut numbers), expected);
    }

    #[test]
    fn test_no_single() {
        let mut numbers = vec![2, 2, -3];
        let expected = vec![];
        assert_eq!(sum_of_three(&mut numbers), expected);
    }

    #[test]
    fn test_unique() {
        let mut numbers = vec![-4, -4, -4, 2, 2, 2];
        let expected = vec![(-4, 2, 2)];
        assert_eq!(sum_of_three(&mut numbers), expected);
    }

    #[test]
    fn test_unique_same() {
        let mut numbers = vec![0, 0, 0, 0, 0, 0];
        let expected = vec![(0, 0, 0)];
        assert_eq!(sum_of_three(&mut numbers), expected);
    }

    #[test]
    fn test_multiple_triplets() {
        let mut numbers = vec![-1, 0, -2, 1, 3, 2];
        let mut result = sum_of_three(&mut numbers);
        let mut expected = vec![(-2, -1, 3), (-2, 0, 2), (-1, 0, 1)];
        assert_eq!(result, expected);
    }

    #[test]
    fn test_duplicates() {
        let mut numbers = vec![-2, -1, 0, 0, 2, -1, 2, -2];
        let mut result = sum_of_three(&mut numbers);
        let mut expected = vec![(-2, 0, 2), (-1, -1, 2)];
        assert_eq!(result, expected);
    }
}
