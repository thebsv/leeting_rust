pub fn split_words_by_separator(words: Vec<String>, separator: char) -> Vec<String> {
    let mut res = vec![];
    for word in words {
        if word.contains(separator) {
            let strs: Vec<String> = word
                .split(separator)
                .map(|c| c.to_string())
                .filter(|x| x.len() > 0)
                .collect();
            res.extend(strs);
        } else {
            res.push(word);
        }
    }
    res
}

pub fn main() {
    let words: Vec<String> = vec![
        String::from("one.two.three"),
        String::from("four.five"),
        String::from("six"),
    ];
    let separator: char = '.';
    println!(
        "split words: {:?}",
        split_words_by_separator(words, separator)
    );
}
