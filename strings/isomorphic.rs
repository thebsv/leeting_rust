use std::{
    collections::{HashMap, HashSet},
    iter::zip,
};

pub fn is_isomorphic(s: String, t: String) -> bool {
    let mut hasha: HashMap<char, Vec<usize>> = HashMap::new();
    let mut hashb: HashMap<char, Vec<usize>> = HashMap::new();

    fn populate_map(mut hashr: HashMap<char, Vec<usize>>, x: String) -> HashMap<char, Vec<usize>> {
        for (index, c) in x.char_indices() {
            hashr.entry(c).or_insert(vec![index]).push(index);
        }

        hashr
    }

    hasha = populate_map(hasha, s);
    hashb = populate_map(hashb, t);

    let mut set1 = HashSet::new();
    let mut set2 = HashSet::new();

    for (x, y) in zip(hasha.values(), hashb.values()) {
        set1.insert(x);
        set2.insert(y);
    }

    let res = set1 == set2;

    // println!("map1: {:#?}", hasha);
    // println!("map2: {:#?}", hashb);
    // println!("res: {}", res);

    res
}

pub fn main() {
    let s = String::from("egg");
    let t = String::from("add");
    println!("is_iso: {}", is_isomorphic(s, t));
}
