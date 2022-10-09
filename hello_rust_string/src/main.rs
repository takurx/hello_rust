fn main() {
    let str_hello_world = "Hello, world!";
    let str_even = "even number";
    let str_odd = "odd number";
    for n in 1..5 {
        println!("{} {}", str_hello_world, n);
        if n%2 == 0 {
            println!("    {}", str_even);
        }
        else {
            println!("    {}", str_odd);
        }
    }  
}
