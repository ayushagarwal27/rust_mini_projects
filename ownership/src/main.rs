fn main() {
    let word = "Hello World!";
    let first_w = first_word(&word);
    // word.clear();
    println!("{}", first_w)

    let a = [1,2,3,4];
    let slice = &[1..3];
    // assert_eq!(slice, &[2, 3]);
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &ch) in bytes.iter().enumerate() {
        if ch == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}
