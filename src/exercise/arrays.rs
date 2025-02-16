pub fn arrays() {
    let some_arr: [u32; 10] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    // access the element:
    println!("{:?}", some_arr[0]);

    // fill the array:
    let filled_arr: [u32; 10] = [0; 10];

    // Slices:
    let some_slice: &[u32] = &filled_arr[0..5];

    println!("{:?}", some_slice);
}
