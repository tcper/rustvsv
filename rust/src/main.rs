use std::fs;
use std::io::{Write, BufReader, BufRead, Error};
#[macro_use]
extern crate serde_json;

fn bubble_sort<T: PartialOrd>(arr: &mut [T]) {
    for i in 0..arr.len() {
        for j in 0..arr.len() - 1 - i {
            // println!("{}", arr[j]);
            if arr[j] > arr[j + 1] {
                arr.swap(j, j + 1);
            }
        }
    }
}

fn main() -> Result<(), Error> {
    let path = "../data.json";
    println!("In file {}", path);
    let contents = fs::read_to_string(path)
        .expect("Should have been able to read the file");
    // println!("With text:\n{contents}");
    let v: Vec<Vec<f64>> = serde_json::from_str(&contents).unwrap();
    // println!("{:?}",v);
    for mut x in v {
        println!("{:?}", x);
        // let mut ve1 = vec![5, -3, 0, 3, 1, 2, -1, 8, 7];
        println!("排序前: {:?}",x);
        bubble_sort(&mut x);
        println!("排序后：{:?}",x);
    }
    Ok(())
}
