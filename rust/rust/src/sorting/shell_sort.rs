/// Shell sort is a generalization of insertion sort that allows the exchange of items
/// that are far apart. The idea is to arrange the list of elements so that, starting
/// anywhere, considering every nth element gives a sorted list.
///
/// Worst O(nlog2 2n)
/// Best O(n log n)
/// Average case depends on gap sequence
pub fn shell_sort<T>(values: &mut Vec<T>)
where
    T: Ord+ Copy
{
    // shell sort works by swiping the value at g given gap and decreasing the gap to 1
    // What's mean of the gap?
    fn insertion<T>(values: &mut Vec<T>, start: usize, gap: usize)
    where
        T: Ord+Copy
    {
        for i in ((start+gap)..values.len()).step_by(gap){
            let val_current =values[i];
            let mut pos=i;
            // make swaps
            while pos>=gap && values[pos-gap]>val_current{
                values[pos]=values[pos-gap];
                pos-=gap;
            }
            values[pos]=val_current;
        }
    }

    let mut count_sublist=values.len() /2; // make gap as long as half of the array
    while count_sublist>0{
        for pos_start in 0..count_sublist{
            insertion(values, pos_start, count_sublist);
        }
        count_sublist/=2; // make gap as half of previous
    }
}

#[cfg(test)]
mod test_shell_sort{
    use super::shell_sort;

    #[test]
    fn basic() {
        let mut vec=vec![3,5,6,3,1,4];
        shell_sort(&mut vec);
        for i in 0..vec.len() -1{
            assert!(vec[i]<=vec[i+1]);
        }
    }

    #[test]
    fn empty() {
        let mut vec: Vec<i32>=vec![];
        shell_sort(&mut vec);
        assert_eq!(vec, vec![]);
    }


    #[test]
    fn reverse() {
        let mut vec=vec![6,5,4,3,2,1];
        shell_sort(&mut vec);
        for i in 0..vec.len()-1{
            assert!(vec[i]<=vec[i+1]);
        }
    }

    #[test]
    fn already_sorted() {
        let mut vec=vec![1,2,3,4,5,6];
        shell_sort(&mut vec);
        for i in 0..vec.len()-1{
            assert!(vec[i]<=vec[i+1]);
        }
    }
}
