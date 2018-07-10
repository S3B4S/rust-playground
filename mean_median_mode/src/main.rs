/**
 * Chapter 8, Common Collections
 * 
 * Given a list of integers, use a vector and return 
 * - the mean (the average value), 
 * - median (when sorted, the value in the middle position), 
 * - and mode (the value that occurs most often; a hash map will be helpful here) of the list.
 */

use std::collections::HashMap;

fn main() {
    let vec = vec![4,50,3,42,32,6,9,6,20,22,12];
    println!("Vector: {:?}", vec);
    println!("-> Mean: {}", mean(&vec));        // 206
    println!("-> Median: {}", median(&vec));    // 12
    println!("-> Mode: {}", mode(&vec));        // 6
}

/**
 * Returns average value of given vector
 */
fn mean(vec: &Vec<i32>) -> i32 {
    let mut total = 0;
    for int in vec.iter() {
        total += int;
    }
    total
}

/**
 * Finds value on middle position after given vector has been sorted. 
 */
fn median(vec: &Vec<i32>) -> i32 {
    let mut copy = vec.to_vec();
    copy.sort();
    let halved = vec.len() / 2;
    copy[halved]
}

/**
 * Find (arbitary) value that occurs most often in the given vector. 
 */
fn mode(vec: &Vec<i32>) -> i32 {
    let mut map = HashMap::new();

    for int in vec.iter() {
        let count = map.entry(int).or_insert(0);
        *count += 1;
    }

    let mut most_present: (&i32, i32) = (&0, 0);
    for (key, value) in map {
        if value > most_present.1 {
            most_present.0 = key;
            most_present.1 = value;
        }
    }
    *most_present.0
}