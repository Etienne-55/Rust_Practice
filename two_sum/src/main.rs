use std::collections::HashMap;


fn two_sum(nums: &[i32], target: i32) -> Option<(usize, usize)> {
    let mut map = HashMap::new();

    for (i, &num) in nums.iter().enumerate() {
        let compelent = target - num;
        if let Some(&index) = map.get(&compelent) {
            return Some((index, i));
        }
        map.insert(num, i);

    }

    None
}

fn main() {

    let nums = vec![4, 1, 2];
    let target = 3;

    match  two_sum(&nums, target) {
        Some((i, j)) => println!("Indices: {} and {}", i, j),
        None => println!("No two sum solution found"),
        
    }
}