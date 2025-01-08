#![allow(unused)]

fn main() {
    // Array - fixed length, known at compile time
    let arr: [u32; 3] = [1, 2, 3];
    print!("arr[1] = {} ", arr[1]);

    let mut arr: [u32; 3] = [1, 2, 3];
    arr[1] = 9;
    print!("arr[1] now is = {} ", arr[1]);

    let arr: [u32; 10] = [0; 10];
    print!("{:?}", arr);

    // Slices
    let nums: [i32; 10] = [-1, 1, -2, 2, -3, 3, -4, 4, -5, 5];

    // First 3 elements
    let s = &nums[0..3]; // not included the third element so you get: 0, 1, 2
    print!("First 3 elements are: {:?}", s);

    let s = &nums[3..7];
    print!("middle 4 elements: {:?}", s);

    let s = &nums[7..10];
    print!("last 4 elements : {:?}", s);

    let s = &nums[..];
    print!("all elemts {:?}", s);
}
