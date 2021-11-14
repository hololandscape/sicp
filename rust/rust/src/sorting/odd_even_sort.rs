/// odd even sort algorithm
/// developed originally for use on parallel processors with local interconnections
/// It is a comparison sorted related to bubble sort, with which
/// it shares many characteristics
/// 
/// Worst O(n^2)
/// Best O(n)
/// Average O(n^2)
pub fn odd_even_sort<T>(arr: &mut [T])
where
    T: Ord
{
    let len =arr.len();
    if len==0{
        return;
    }

    let mut sorted=false;
    while !sorted{
        sorted=true;

        for i in (1..len-1).step_by(2){
            if arr[i]>arr[i+1]{
                arr.swap(i, i+1);
                sorted=false;
            }
        }

        for i in (0..len -1).step_by(2){
            if arr[i] > arr[i+1]{
                arr.swap(i, i+1);
                sorted=false;
            }
        }
    }
}


#[cfg(test)]
mod test_odd_even_sort{
    use super::*;

    #[test]
    fn basic() {
        let mut arr=vec![3,5,1,2,4,6];
        odd_even_sort(&mut arr);
        assert_eq!(arr, vec![1,2,3,4,5,6]);
    }

    #[test]
    fn empty() {
        let mut arr=Vec::<i32>::new();
        odd_even_sort(&mut arr);
        assert_eq!(arr, vec![]);
    }

    #[test]
    fn one_element() {
        let mut arr=vec![1];
        odd_even_sort(&mut arr);
        assert_eq!(arr, vec![1]);
    }

    #[test]
    fn pre_sorted() {
        let mut arr=vec![0,1,2,3,4,5,6,7,8,9];
        odd_even_sort(&mut arr);
        assert_eq!(arr,vec![0,1,2,3,4,5,6,7,8,9]);
    }
}
