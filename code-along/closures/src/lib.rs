mod shapes;
use std::thread;
use shapes::rectangle;

pub fn immutable_borrow_closure() {
    let list = vec![1, 2, 3, 4, 5];
    println!("Before borrowing list as a immutable reference: {:?}", list);

    let closure = || println!("Borrowing list as a immutable reference: {:?}", list);

    closure();
    println!("After borrowing the borrow: {:?}\n", list);
}

pub fn mutable_borrow_closure() {
    let mut list = vec![1, 2, 3, 4, 5];
    const VALUE: i32 = 6;
    println!("Before borrowing list as a mutable reference: {:?}", list);

    let mut closure = || {
        list.push(VALUE);
        println!(
            "Pushing {VALUE} onto the mutable reference of list: {:?}",
            list
        );
    };

    closure();
    println!("After modifying list: {:?}\n", list);
}

pub fn moving_ownership_to_another_thread_closure() {
    let mut list = vec![1, 2, 3, 4, 5];
    println!("Before moving list ownership to a new thread: {:?}", list);

    list = thread::spawn(move || {
        const VALUE: i32 = 6;
        list.push(VALUE);
        println!("Moved list into a new thread and added {VALUE}: {:?}", list);
        list
    })
    .join()
    .unwrap();

    println!("Returned list ownership to main thread: {:?}\n", list);
}

pub fn sort_rect_by_width() {
    let r1 = rectangle::Rectangle::build(10, 20);
    let r2 = rectangle::Rectangle::build(20, 10);
    let r3 = rectangle::Rectangle::build(5, 10);
    let r4 = rectangle::Rectangle::build(75, 105);
    let mut list = [r1, r2, r3, r4];
    let mut operation_count = 0;
    println!("List before sorting: {:#?}", list);
    list.sort_by_key(|r| {
        operation_count += 1;
        r.get_width()
    });
    println!("List after sorting: {:#?}\n", list);
    println!("Number of sorting operations: {operation_count}");
}
