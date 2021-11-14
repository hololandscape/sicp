/// The gnome sort is a sorting algorithm which is similar to insertion sort in that it works with ont item at a 
/// a time but gets the item to the proper place by a series of swaps, similar to a bubble sort. It is conceptually simple, requiring no nested loops.
/// The average running time is O(n^2) but tends towards O(n) if the list is initially almost sorted.
/// 
/// Worst case O(n^2)
/// Best case O(n)
/// Average case O(n^2)
use std::cmp;

pub fn gnome_sort<T>(arr: &[T])-> Vec<T>
where
    T: cmp::PartialEq + cmp::PartialOrd+Clone,
{
    // Copy self to a new Vec
    let mut arr=arr.to_vec();
    let mut i:usize=1;
    let mut j:usize=2;

    while i< arr.len(){
        if arr[i-1]<arr[i]{
            i=j;
            j=i+1;
        }else{
            arr.swap(i-1,i);
            i-=1;
            if i==0{
                i=j;
                i+=1;
            }
        }
    }
    arr
}

#[cfg(test)]
mod test_gnome{
    use super::*;

    #[test]
    fn basic() {
        let res=gnome_sort(&vec![6,5,-8,3,2,3]);
        assert_eq!(res, vec![-8,2,3,3,5,6]);
    }

    #[test]
    fn already_sorted() {
        let res=gnome_sort(&vec!["a","b","c"]);
        assert_eq!(res, vec!["a","b","c"]);
    }

    #[test]
    fn odd_number_of_elements() {
        let res=gnome_sort(&vec!["d", "a", "c", "e", "b"]);
        assert_eq!(res, vec!["a","b","c","d","e"]);
    }

    #[test]
    fn empty() {
        let res=gnome_sort(&Vec::<u8>::new());
        assert_eq!(res,vec![]);
    }
}
