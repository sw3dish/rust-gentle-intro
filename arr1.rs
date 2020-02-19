fn main() {
    let arr = [10, 20, 30, 40];
    let first = arr[0];
    println!("First: {}", first);

    for i in 0..arr.len() {
        println!("arr[{}]: {}", i, arr[i]);
    }

    println!("Length: {}", arr.len());
}
