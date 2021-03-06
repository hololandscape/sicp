/// Integer sorting algorithm, running time is linear in the number of items.
/// And only suitable for direct use in situations where variation in keys
/// is not significantly greater than the number rof items.
///
/// In place counting sort for collections of u32
/// O(n+maxval) in time, where maxval is the biggest value an input can possibly take O(maxval) in memory
/// u32 is chosen arbitrarly, a counting sort probably shouldn't be used on data that requires bigger types.
///
/// Worst O(n+k)
/// Base case O(n+k)
/// Average case O(n+k)
pub fn counting_sort(arr: &mut [u32], maxval: usize) {
    // Create the Vec with maxval+1 numbers of 0 use vec macro!
    let mut occurences: Vec<usize> = vec![0; maxval + 1];

    // Add the items to the Vec, value as key, the sequence as value.
    for &data in arr.iter() {
        occurences[data as usize] += 1;
    }

    //
    let mut i = 0;
    for (data, &number) in occurences.iter().enumerate() {
        for _ in 0..number {
            arr[i] = data as u32;
            i += 1;
        }
    }
}

use ::std::ops::AddAssign;
/// Generic implementation of a counting sort for all usigned types
#[allow(dead_code)]
pub fn generic_counting_sort<T>(arr: &mut [T], maxval: usize)
where
    T: Into<u64> + From<u8> + AddAssign + Copy,
{
    let mut occurences: Vec<usize> = vec![0; maxval + 1];

    for &data in arr.iter() {
        // What's the mean of into()?
        occurences[data.into() as usize] += 1;
    }

    // Current index in output array
    let mut i = 0;

    // current data point, necessary to be type-safe
    let mut data = T::from(0);

    // This will iterate from 0 to the largest data point in `arr`
    // `number` contains the occurances of the dat point `data`

    for &number in occurences.iter() {
        for _ in 0..number {
            arr[i] = data;
            i += 1;
        }

        data += T::from(1);
    }
}

#[cfg(test)]
mod test_counting_sort {
    use super::super::is_sorted;
    use super::*;

    #[test]
    fn counting_sort_descending() {
        let mut ve1 = vec![6, 5, 4, 3, 2, 1];
        counting_sort(&mut ve1, 6);
        assert!(is_sorted(&ve1));
    }

    #[test]
    fn counting_sort_pre_sorted() {
        let mut ve2 = vec![1, 2, 3, 4, 5, 6];
        counting_sort(&mut ve2, 6);

        assert!(is_sorted(&ve2));
    }

    #[test]
    fn generic_counting_sort() {
        let mut ve1: Vec<u8> = vec![100, 30, 60, 10, 20, 120, 1];
        super::generic_counting_sort(&mut ve1, 120);

        assert!(is_sorted(&ve1));
    }

    #[test]
    fn presorted_u64_counting_sort() {
        let mut ve2: Vec<u64> = vec![1, 2, 3, 4, 5, 6];
        super::generic_counting_sort(&mut ve2, 6);

        assert!(is_sorted(&ve2));
    }
}
