fn main() {
    let _s1 = String::from("hello");
    let _s2 = &_s1;

    println!("{}, world!", _s1);
    let tmp = String::from("String second word");
    println!("{}",first_word(&tmp));
    println!("{}",first_word(&tmp));
    let t = tmp;
    println!("{}", second_word(&t));
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

fn second_word(s: &String) -> &str {

    let bytes = &s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[i + 1..s.len()];
        }
    }
    &s[..]
}