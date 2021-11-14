// bubble_sort for Rust

// What's the Ord?
   // from mathematics, a total or linear order is a partial order in which any two elements are comparable
   // if a<=b and b<=c then a<=c, if a<=b and b<=a then a=b
// The mean after function name?
// methods using generics (trait bound), the bubble_sort is generic over some type T which implements the Ord trait
pub fn bubble_sort<T: Ord>(arr: &mut [T]){
    for i in 0..arr.len(){
        for j in 0..arr.len()-1-i{
            if arr[j] > arr[j+1]{
                arr.swap(j, j+1)
            }
        }
    }
}

// What's the mean of cfg?
// annotation on the tests module tell Rust to compile and run the test code only when you run `cargo test`, not when `cargo build`
// unit tests
#[cfg(test)]

// `mod` keyword use to defined `mod` can hold definitions for other items. such as structs, enums, constants, traits and functions
mod test_bs{

    // What are the tools under super?
    // a relative path starts from the current module and uses `self`, `super` or an identifier in the current module
    // `use` keyword add a path into scope
    use super::*;

    // What's the name of this block?
    // define the test function
    #[test]
    fn descending() {
        let mut ve1=vec![6,5,4,3,2,1];
        bubble_sort(&mut ve1);
        for i in 0..ve1.len()-1{
            assert!(ve1[i]<=ve1[i+1]);
        }
    }

    #[test]
    fn ascending() {
        let mut ve2=vec![1,2,3,4,5,6];
        bubble_sort(&mut ve2);
        for i in 0..ve2.len()-1{
            assert!(ve2[i]<=ve2[i+1]);
        }
    }
}
