use std::collections::HashMap;

pub fn count_pairs(coordinates: Vec<Vec<i32>>, k: i32) -> i32 {
    /*
    x1 XOR x2 = j    ->  x1 XOR j = x2
    y1 XOR y2 = k-j  ->  y1 XOR (k-j) = y2

    So, we can hashmap.get() (x1 xor j), (y1 xor (k-j)) which is the same as get() (x2, y2); where the (x1^x2 + y1^y2) pair adds up to k.
    Once that is done, we can add the result to the output, and then insert the current coordinate (x1, y1) into the map as well.
    */

    let mut hashm: HashMap<Vec<i32>, i32> = HashMap::new();
    let mut output: i32 = 0;

    for c in coordinates {
        for x in 0..=k {
            output += hashm.get(&vec![(c[0] ^ x), (c[1] ^ (k - x))]).unwrap_or(&0);
        }
        *hashm.entry(vec![c[0], c[1]]).or_insert(0) += 1;
    }

    output
}

pub fn main() {
    let coordinates: Vec<Vec<i32>> = vec![vec![1, 2], vec![4, 2], vec![1, 3], vec![5, 2]];
    let k = 5;

    println!("answer: {}", count_pairs(coordinates, k));
}
