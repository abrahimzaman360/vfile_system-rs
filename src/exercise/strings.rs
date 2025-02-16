pub fn strings(super_slice: &str) -> String {
    let some_beautiful_str = String::from(super_slice);

    some_beautiful_str
}

pub fn super_slice() {
    let new_slice: &str = "Some Slice";

    for each_word in new_slice.chars().into_iter() {
        println!("{each_word}")
    }
}
