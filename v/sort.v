import os
import time
import json

fn main() {
    sw := time.new_stopwatch()
    str := os.read_file('../data.json')!
    mut arr := json.decode([][]f64, str)!
    // println(arr)
    for i in 0..arr.len {
        bubble_sort(mut arr[i])
        // println(arr[i])
    }
    println('消耗时间 : ${sw.elapsed().milliseconds()}ms')
}


// Bubble Sort
[direct_array_access]
fn bubble_sort(mut array []f64) {
	for i in 0..array.len {
		for j in 0..array.len - 1 - i {
			if array[j] > array[j + 1] {
				array[j], array[j + 1] = array[j + 1], array[j]
			}
		}
	}
	// println('${array[i]}')
}
