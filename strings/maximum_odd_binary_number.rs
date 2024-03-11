pub fn maximum_odd_binary_number(s: String) -> String {
    let mut count_ones = 0;
    for c in s.chars() {
        if c == '1' {
            count_ones += 1;
        }
    }
    count_ones -= 1;

    let mut res = String::new();

    while count_ones > 0 {
        res.push('1');
        count_ones -= 1;
    }

    while (s.len() - res.len() - 1) > 0 {
        res.push('0');
    }

    res.push('1');

    res
}

fn main() {
    let s = String::from("010");
    println!("max_odd_bin: {}", maximum_odd_binary_number(s));
    let s1 = String::from("0101");
    println!("max_odd_bin: {}", maximum_odd_binary_number(s1));
}
