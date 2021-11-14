/// Stooge sort is a recursive sorting algorithm. It is notable for its
/// exceptionally bas time complexity of O(n^(log 3/log 1.5))= O(n^2.7095)
/// 
/// Properties
/// 
/// * Worst case performance O(n ^(log(3)/log(1.5)))

fn _stooge_sort<T>(arr: &mut [T], start: usize, end: usize)
where
    T: Ord
{
    if arr[start] >arr[end]{
        arr.swap(start, end);
    }

    if start+1>=end{
        return;
    }

    let k=(end -start+1)/3;

    _stooge_sort(arr, start, end-k);
    _stooge_sort(arr, start+k, end);
    _stooge_sort(arr, start, end-k);
}

pub fn stooge_sort<T>(arr: &mut [T])
where
    T: Ord
{
    let len =arr.len();
    if len==0{
        return;
    }

    _stooge_sort(arr, 0, len-1);
}

#[cfg(test)]
mod test_stooge_sort{
    use super::*;

    #[test]
    fn basic() {
        let mut vec=vec![3,5,6,3,1,4];
        stooge_sort(&mut vec);
        for i in 0..vec.len()-1{
            assert!(vec[i]<=vec[i+1]);
        }
    }

    #[test]
    fn empty() {
        let mut vec: Vec<i32>=vec![];
        stooge_sort(&mut vec);
        assert_eq!(vec, vec![]);
    }

    #[test]
    fn reverse() {
        let mut vec=vec![6,5,4,3,2,1];
        stooge_sort(&mut vec);
        for i in 0..vec.len()-1{
            assert!(vec[i]<=vec[i+1]);
        }
    }

    #[test]
    fn already_sorted() {
        let mut vec=vec![1,2,3,4,5,6];
        stooge_sort(&mut vec);
        for i in 0..vec.len()-1{
            assert!(vec[i]<=vec[i+1]);
        }
    }
}