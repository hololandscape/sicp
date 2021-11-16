/// much less efficient on large lists then more advanced algorithms such as quicksort
/// heapsort, or merge sort
///
/// Worst O(n^2)
/// Best O(n)
/// Average O(n^2)
use std::cmp;

#[allow(dead_code)]
pub fn insertion_sort<T>(arr: &[T]) -> Vec<T>
where
    T: cmp::PartialEq + cmp::PartialOrd + Clone,
{
    let mut result: Vec<T> = Vec::with_capacity(arr.len());

    // Iterable over the elements to sort and put a clone of the element to insert in elem.__rust_force_expr!
    for elem in arr.iter().cloned() {
        // How many elements have already been insert?
        let n_inserted = result.len();

        // Loop over the inserted elements and one more index.
        // 0..n_inserted not includes n_inserted,
        // 0..=n_inserted is include n_inserted
        for i in 0..=n_inserted {
            // If at the end of result[i] is larger than current element, we have found the  right spot;
            if i == n_inserted || result[i] > elem {
                result.insert(i, elem);
                break;
            }
        }
    }
    result
}

#[cfg(test)]

mod test_is {

    use super::*;

    #[test]
    fn empty() {
        let res = insertion_sort(&Vec::<u8>::new());
        assert_eq!(res, vec![]);
    }

    #[test]
    fn one_element() {
        let res = insertion_sort(&vec!["a"]);
        assert_eq!(res, vec!["a"]);
    }

    #[test]
    fn already_sorted() {
        let res = insertion_sort(&vec!["a", "b", "c"]);
        assert_eq!(res, vec!["a", "b", "c"]);
    }

    #[test]
    fn basic() {
        let res = insertion_sort(&vec!["d", "a", "c", "b"]);
        assert_eq!(res, vec!["a", "b", "c", "d"]);
    }

    #[test]
    fn odd_number_of_elements() {
        let res = insertion_sort(&vec!["d", "a", "c", "e", "b"]);
        assert_eq!(res, vec!["a", "b", "c", "d", "e"]);
    }

    #[test]
    fn repeated_elements() {
        let res = insertion_sort(&vec![542, 542, 542, 542]);
        assert_eq!(res, vec![542, 542, 542, 542]);
    }
}
