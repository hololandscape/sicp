pub mod data_structures;
pub mod searching;
pub mod sorting;

#[cfg(test)]
mod test_lib {

    use super::*;

    #[test]
    fn quick_sort() {
        let mut ve1 = vec![6, 5, 4, 3, 2, 1];
        sorting::quick_sort(&mut ve1);
        for i in 0..ve1.len() - 1 {
            assert!(ve1[i] <= ve1[i + 1]);
        }

        //pre-sorted
        let mut ve2 = vec![1, 2, 3, 4, 5, 6];
        sorting::quick_sort(&mut ve2);
        for i in 0..ve2.len() - 1 {
            assert!(ve2[i] <= ve2[i + 1]);
        }
    }
}
