fn main() { 
    let v1 = vec![1, 2];
    
    // iter produces an iterator of immutable references
    let mut v1_iter = v1.iter();
    
    for val in v1_iter.as_ref() {
        println!("{val}");
    }
    assert_eq!(v1_iter.next(), Some(&1));
    assert_eq!(v1_iter.next(), Some(&2));
    assert_eq!(v1_iter.next(), None);

    let v2 = vec![1, 2];
    
    // into_iter produces an iterator of owned values
    let mut _v2_iter = v2.into_iter();
    

    let mut v3 = vec![1, 2];

    // iter_mut produces an iterator of immutable references
    let _v3_iter = v3.iter_mut();

    let v4 = vec![1, 2];
    let v4_iter = v4.iter();
    let total: i32 = v4_iter.sum();
    println!("Sum of \n{v4:#?}\n{total}");

    let v5 = vec![1, 2];
    let v5_iter = v5.iter();
    let v5_plus_1: Vec<u32> = v5_iter.map(|x| x + 1).collect();
    assert_eq!(v5_plus_1, vec![2, 3]);

    let v6 = vec![1, 2, 3, 4, 5, 6, 7, 8];

    // Must be owned in order to place owned values into the vector 
    let v6_iter = v6.into_iter();
    let _even: Vec<i32> = v6_iter.filter(|x| x % 2 == 0).collect(); 
}
