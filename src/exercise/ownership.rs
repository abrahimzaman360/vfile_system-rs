pub fn user() {
    let dropping: String = String::from("Satan");
    let own_drop = &dropping;
    {
        let s2 = dropping.clone();
    }
    println!("{} {}", dropping, own_drop);

    let some_spam = "Spam".to_string();
    // let addr = borrow(some_spam);
    // println!("{addr}")
}
