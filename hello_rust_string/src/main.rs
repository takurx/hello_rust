fn main() {
    let str_hello = "Hello";
    let str_world = "world!";
    let str_hello_world = String::from(str_hello) + ", " + str_world;
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
    for i in str_hello.as_str().chars() {
        println!("{}", i);
    }
}
