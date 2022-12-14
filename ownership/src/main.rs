fn main() {
    println!(
        "&String={}Bytes &str={}Bytes",
        std::mem::size_of::<&String>(),
        std::mem::size_of::<&str>(),
    );
    {
        let s = String::from("hello world");
        // if you still need the original string pass the slice to another variable
        let word = first_word(&s);
        // if you want to slice a string shadow it with its slice if you no longer need the rest of it
        let s = first_word(&s);
        println!("{} and s is: {}", word, s);
    };
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        };
    }
    &s[..]
}
fn _ffirst_word(s: &String) -> usize {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        };
    }
    s.len()
}
