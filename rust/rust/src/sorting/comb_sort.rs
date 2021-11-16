///
/// Comb sort is a relatively simple sorting algorithm and improves on bubble sort in the same way that shell sort
/// improves on insertion sort
///
/// The basic idea of comb sort is that the gap (distance from two compared elements) can be much more than 1. And
/// the inner loop of bubble sort, which does actual `swap`, is modified such that the gap between swapped elements
/// goes down in steps of a `shrink factor k [n/k, n/k^2, ..., 1]`. And the gap is divided by the shrink factor in
/// every loop, and the process repeats until the gap is 1.
///
/// And comb sort continues using a gap of 1 until the list is fully sorted.
///
/// `k=1.3` has been suggested as an idea value(great effect on the efficiency of comb sort)
///
/// Worst case performance  O(n^2)
/// Best case performance  O(n log n)
/// Average case performance O(n^2/2^p)
pub fn comb_sort<T>(arr: &mut [T])
where
    T: Ord,
{
    let mut gap = arr.len();
    let shrink = 1.3;
    let mut sorted = false;

    while !sorted {
        gap = (gap as f32 / shrink).floor() as usize;
        if gap <= 1 {
            gap = 1;
            sorted = true;
        }
        for i in 0..arr.len() - gap {
            let j = i + gap;
            if arr[i] > arr[j] {
                arr.swap(i, j);
                sorted = false;
            }
        }
    }
}

#[cfg(test)]
mod test_comb_sort {
    use super::*;

    #[test]
    fn descending() {
        //descending
        let mut ve1 = vec![6, 5, 4, 3, 2, 1];
        comb_sort(&mut ve1);
        for i in 0..ve1.len() - 1 {
            assert!(ve1[i] <= ve1[i + 1]);
        }
    }

    #[test]
    fn ascending() {
        //pre-sorted
        let mut ve2 = vec![1, 2, 3, 4, 5, 6];
        comb_sort(&mut ve2);
        for i in 0..ve2.len() - 1 {
            assert!(ve2[i] <= ve2[i + 1]);
        }
    }
}
