fn bubble_sort(arr: &mut [i32]) {
    let mut n = arr.len();
    while n > 0 {
        let mut last_modified_index = 0;
        for i in 1..n {
            if arr[i - 1] > arr[i] {
                arr.swap(i - 1, i);
                last_modified_index = i;
            }
        }
        n = last_modified_index;
    }
}

fn main() {
    let mut arr = [64, 34, 25, 12, 22, 11, 90];
    println!("Unsorted array: {:?}", arr);
    
    bubble_sort(&mut arr);
    
    println!("Sorted array: {:?}", arr);
}
