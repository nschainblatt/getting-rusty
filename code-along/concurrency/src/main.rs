use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    let mut v = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let handle = thread::spawn(move || {
        for i in 1..10 {
            println!("This is from the spawned thread: {i}");
            println!("{v:?}");
            v.pop();
            thread::sleep(Duration::from_millis(1));
        }
    });

    handle.join().unwrap();

    for i in 1..10 {
        println!("This is from the main thread: {i}");
        thread::sleep(Duration::from_millis(1));
    }

    channels();
}

fn channels() {
    let (tx1, rx) = mpsc::channel();
    let tx2 = tx1.clone();

    thread::spawn(move || {
        let vals = vec![
            String::from("test1"),
            String::from("test2"),
            String::from("test3"),
            String::from("test4"),
        ];
        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_millis(1));
        }
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("test5"),
            String::from("test6"),
            String::from("test7"),
            String::from("test8"),
        ];
        for val in vals {
            tx2.send(val).unwrap();
            thread::sleep(Duration::from_millis(1));
        }
    });

    let receiv = rx.recv().unwrap();
    println!("Got: {receiv}");
    for received in &rx {
        println!("Got: {received}");
    }
}
