/// Worst O(log n)
/// Best O(1)
/// Average O(log n)
use std::cmp::Ordering;

// enum Option(None, and Some)
pub fn binary_search<T: Ord>(item: &T, arr: &[T]) -> Option<usize> {
    let mut left = 0;
    let mut right = arr.len();

    while left < right {
        let mid = left + (right - left) / 2;
        // cmp method returns an [`Ordering`] between `self` and `other`.
        match item.cmp(&arr[mid]) {
            // An `Ordering` is the result of a comparison between two values.
            Ordering::Less => right = mid,
            Ordering::Equal => return Some(mid),
            Ordering::Greater => left = mid + 1,
        }
    }
    // The item can not be find
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        let index = binary_search(&"a", &vec![]);
        assert_eq!(index, None);
    }

    #[test]
    fn one_item() {
        let index = binary_search(&"a", &vec!["a"]);
        assert_eq!(index, Some(0));
    }

    #[test]
    fn search_string() {
        let index = binary_search(&"a", &vec!["a", "b", "c", "d", "google", "zoo"]);
        assert_eq!(index, Some(0));
    }

    #[test]
    fn search_ints() {
        let index = binary_search(&4, &vec![1, 2, 3, 4]);
        assert_eq!(index, Some(3));

        let index = binary_search(&3, &vec![1, 2, 3, 4]);
        assert_eq!(index, Some(2));

        let index = binary_search(&2, &vec![1, 2, 3, 4]);
        assert_eq!(index, Some(1));

        let index = binary_search(&1, &vec![1, 2, 3, 4]);
        assert_eq!(index, Some(0));
    }

    #[test]
    fn not_found() {
        let index = binary_search(&5, &vec![1, 2, 3, 4]);
        assert_eq!(index, None);
    }
}
