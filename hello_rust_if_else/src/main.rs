fn main() {
    for n in 1..10 {
        println!("Hello, world! {}", n);
        if n%2 == 0 {
            println!("    even number");
        }
        else {
            println!("    odd number");
        }
    }
}
