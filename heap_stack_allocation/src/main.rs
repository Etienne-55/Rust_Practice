fn main() {

    let a = 10;

    let b = Box::new(10);

    println!("Value stored on the stack: {}", a);

    println!("Value stored on the heap: {}", *b);
}
