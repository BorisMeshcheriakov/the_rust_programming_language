fn main() {
    let mut s1 = String::from("hello");

    s1.push_str(", world!");

    println!("{}", s1);

    let len = calculate_length(&s1);

    println!("The length of '{}' is {}", s1, len);

    let mut s = String::from("hello world");

    let word = first_word(&s);

    s.clear();

    println!("{}", word);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}
