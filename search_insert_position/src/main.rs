fn print_target(nums: &[i32], target: i32) -> Option<usize> {
    for (i, &num) in nums.iter().enumerate() {
        if num == target {
            return Some(i);
        }
    }
    None
}

fn main() {

    let nums = vec![1,2,3,4,5];
    let target = 4;
    match print_target(&nums, target) {
        Some(index) => println!("Target found at index: {}", index),
        None => println!("Target not found"),
    }
}
