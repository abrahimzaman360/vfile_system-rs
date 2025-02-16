pub fn arrays() {
    let some_arr: [u32; 10] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    // access the element:
    println!("{:?}", some_arr[0]);

    let mut some_arr: [u32; 10] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    // change an element:
    some_arr[2] = 10;

    println!("{:?}", some_arr);

    // fill the array:
    let filled_arr: [u32; 10] = [0; 10];

    // Slices:
    let some_slice: &[u32] = &filled_arr[0..(filled_arr.len() / 2)];
    println!("{:?}", some_slice);

    let some_slice: &[u32] = &filled_arr[0..];
    println!("{:?}", some_slice);

    let some_slice: &[u32] = &filled_arr[..];
    println!("{:?}", some_slice);
}
