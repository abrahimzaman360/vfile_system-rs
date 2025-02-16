pub fn error() -> Result<i32, String> {
    let arr = [1, 2, 3];
    let x: Option<&i32> = arr.get(10);

    match x {
        Some(x) => Ok(*x),
        None => Err("Hello World!".to_string()),
    }
}
