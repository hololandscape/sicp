/// a method for finding a target value within a list.
/// It sequentially checks each element of the list for the target value until a match is found or util all
/// the elements have been searched
///
/// Worst O(n)
/// Best O(1)
/// Average O(n)
use std::cmp::PartialEq;

pub fn linear_search<T>(item: &T, arr: &[T]) -> Option<usize>
where
    T: PartialEq,
{
    for (i, data) in arr.iter().enumerate() {
        if item == data {
            return Some(i);
        }
    }
    None
}

#[cfg(test)]
mod test_binary_search {
    use super::*;

    #[test]
    fn search_strings() {
        let index = linear_search(&"a", &vec!["a", "b", "c", "d", "google", "zoo"]);
        assert_eq!(index, Some(0));
    }

    #[test]
    fn search_ints() {
        let index = linear_search(&4, &vec![1, 2, 3, 4]);
        assert_eq!(index, Some(3));

        let index = linear_search(&3, &vec![1, 2, 3, 4]);
        assert_eq!(index, Some(2));

        let index = linear_search(&1, &vec![1, 2, 3, 4]);
        assert_eq!(index, Some(0));
    }

    #[test]
    fn no_found() {
        let index = linear_search(&5, &vec![1, 2, 3, 4]);
        assert_eq!(index, None);
    }

    #[test]
    fn empty() {
        let index = linear_search(&1, &vec![]);
        assert_eq!(index, None);
    }
}
