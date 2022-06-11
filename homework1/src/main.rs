fn main() {
    let org_arr = [1, 2, 3, 5, 6, 8, 10, 11];
    println!("Original array: {:?}", org_arr);
    let sub_arr = [6, 8];

    let mut slice_idx = 0;
    while slice_idx < org_arr.len() - sub_arr.len() {
        let slice_arr = &org_arr[slice_idx..slice_idx + sub_arr.len()];
        if sub_arr == slice_arr {
            println!("Sub Array: {:?}, Bingo!", slice_arr)
        }
        slice_idx += 1;
    }
}
