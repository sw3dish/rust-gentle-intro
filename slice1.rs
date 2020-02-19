fn main() {
    let mut arr = [1, 2, 3, 4];
    let slice1 = &arr[1..3];
    let slice2 = &arr[1..];

    println!("{:?}", slice1);
    println!("{:?}", slice2);
}
