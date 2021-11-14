/// cocktail shaker sort
/// It's an extension of bubble sort by more quickly moving items to the beginning of the
/// list, it provides only marginal performance improvement.
/// 
/// Properties
/// * Worst case performance O(n^2)
/// * Best case performance O(n)
/// * Average case performance O(n^2)

pub fn cocktail_shaker_sort<T>(arr: &mut [T])
where
    T: Ord
{
    let len =arr.len();

    if len==0{
        return;
    }

    loop{
        let mut swapped=false;
        // What's the mean of clamp?
        // Restrict a value to a certain interval.

        // Why use clamp() function?
        for i in 0..(len-1).clamp(0, len){
            if arr[i]> arr[i+1]{
                arr.swap(i, i+1);
                swapped=true;
            }
        }

        if !swapped{
            break;
        }

        swapped=false;

        for i in (0..(len-1).clamp(0,len)).rev(){
            if arr[i]>arr[i+1]{
                arr.swap(i, i+1);
                swapped=true;
            }
        }

        if !swapped{
            break;
        }
    }
}

#[cfg(test)]
mod test_for_cocktail_shaker{
    use super::*;

    #[test]
    fn basic() {
        let mut arr=vec![5,2,1,3,4,6];
        cocktail_shaker_sort(&mut arr);
        assert_eq!(arr, vec![1,2,3,4,5,6]);
    }

    #[test]
    fn empty() {
        let mut arr=Vec::<i32>::new();
        cocktail_shaker_sort(&mut arr);
        assert_eq!(arr, vec![])
    }

    #[test]
    fn one_element() {
        let mut arr=vec![1];
        cocktail_shaker_sort(&mut arr);
        assert_eq!(arr, vec![1]);
    }

    #[test]
    fn pre_sorted() {
        let mut arr=vec![1,2,3,4,5,6];
        cocktail_shaker_sort(&mut arr);
        assert_eq!(arr,vec![1,2,3,4,5,6]);
    }
}