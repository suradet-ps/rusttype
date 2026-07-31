fn double_even_numbers(numbers: &[i32]) -> Vec<i32> {
    numbers
        .iter()
        .filter(|&&n| n % 2 == 0)
        .map(|&n| n * 2)
        .collect()
}

fn sum_of_squares(numbers: &[i32]) -> i32 {
    numbers.iter().map(|&n| n * n).sum()
}

fn find_first_negative(numbers: &[i32]) -> Option<&i32> {
    numbers.iter().find(|&&n| n < 0)
}

fn zip_with_index(words: &[&str]) -> Vec<(usize, &str)> {
    words.iter().enumerate().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn double_even() {
        let input = vec![1, 2, 3, 4, 5, 6];
        let result = double_even_numbers(&input);
        assert_eq!(result, vec![4, 8, 12]);
    }

    #[test]
    fn sum_squares() {
        assert_eq!(sum_of_squares(&[1, 2, 3]), 14);
    }

    #[test]
    fn first_negative_found() {
        let nums = [1, 2, -3, 4];
        assert_eq!(find_first_negative(&nums), Some(&-3));
    }

    #[test]
    fn first_negative_none() {
        let nums = [1, 2, 3];
        assert!(find_first_negative(&nums).is_none());
    }

    #[test]
    fn zip_index() {
        let words = vec!["a", "b", "c"];
        let result = zip_with_index(&words);
        assert_eq!(result, vec![(0, "a"), (1, "b"), (2, "c")]);
    }
}
