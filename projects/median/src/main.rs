use ::std::collections::HashMap;

fn bubble_sort(mut v: Vec<i32>) -> Vec<i32> {
    let mut already_sorted = true;
    for i in 0..v.len() - 1 {
        for j in 0..v.len() - i - 1 {
            if v[j] > v[j + 1] {
                let temp = v[j + 1];
                v[j + 1] = v[j];
                v[j] = temp;
                already_sorted = false;
            }
        }
        if already_sorted {
            println!("Already sorted");
            break;
        }
    }
    v
}

fn get_median(v: &Vec<i32>) -> Option<f64> {
    if v.len() == 0 {
        None
    } else if v.len() % 2 == 0 {
        let left_middle = (v.len() - 1) / 2;
        Some((v[left_middle] + v[left_middle + 1]) as f64 / 2.0)
    } else {
        let middle = (v.len() - 1) / 2;
        Some(v[middle] as f64)
    }
}

fn get_mode(v: &Vec<i32>) -> Option<i32> {
    let mut num_count = HashMap::new();

    for num in v.iter() {
        let count = num_count.entry(num).or_insert(1);
        *count += 1;
    }

    let mut mode: Option<i32> = None;

    for (_key, value) in &num_count {
        match mode {
            Some(num) => {
                if value > &num {
                    mode = Some(*value);
                }
            }
            None => mode = Some(*value),
        }
    }

    mode
}

fn main() {
    let un_sorted_v: Vec<i32> = vec![5, 1, -5, 9, 2, 34, -1, 5, 99, -11];
    let sorted_v = bubble_sort(un_sorted_v.clone());
    println!("\nBubble sort:");
    println!("{:?}", un_sorted_v);
    println!("{:?}", sorted_v);

    println!("\n{:?}", sorted_v);
    let _ = bubble_sort(sorted_v.clone());

    println!("\nMedians:");
    let median = get_median(&sorted_v);
    match median {
        Some(num) => println!("{num}"),
        None => println!("The vector is empty, so there is no median"),
    }

    let median = get_median(&vec![]);
    match median {
        Some(num) => println!("{num}"),
        None => println!("The vector is empty, so there is no median"),
    }

    let median = get_median(&vec![1]);
    match median {
        Some(num) => println!("{num}"),
        None => println!("The vector is empty, so there is no median"),
    }

    let median = get_median(&vec![1, 2]);
    match median {
        Some(num) => println!("{num}"),
        None => println!("The vector is empty, so there is no median"),
    }

    println!("\nMode:");
    let mode = get_mode(&sorted_v);
    match mode {
        Some(num) => println!("{num}"),
        None => println!("The vector is empty, so there is no mode"),
    };
}
