fn practice(nums: Vec<usize>, index: usize) -> usize {
    // let val = nums.get(index);
    // match val {
    //     Some(x) => return x * 5,
    //     None => return index * 5,
    // }
    return nums.get(index).unwrap_or(&index) * 5;
}

fn main() {
    let nums: Vec<usize> = vec![1, 2, 3, 4];
    let nums2: Vec<usize> = vec![1, 2, 3, 4];

    println!("{:?}", practice(nums, 2));
    println!("{:?}", practice(nums2, 6));
}
