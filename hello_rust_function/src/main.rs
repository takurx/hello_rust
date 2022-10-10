fn main() {
    let str_hello = "Hello";
    let str_world = "world!";
    let str_hello_world = String::from(str_hello) + ", " + str_world;
    let str_even = "even number";
    let str_odd = "odd number";

    for n in 1..5 {
        print_even_or_odd(n, &str_hello_world);
        //let even_or_odd = is_even_or_odd_by(n); 
        if is_even_or_odd_by(n) {
            println!("    {}", str_even);
        }
        else {
            println!("    {}", str_odd);
        }
    }

    for i in str_hello.chars() {
        println!("{}", i);
    }
}

fn print_even_or_odd(n: u32, str_hello_world: &String) -> (){
    println!("{} {}", str_hello_world, n);
}

fn is_even_or_odd_by(n: u32) -> bool{
    if n%2 == 0 {
        return true;
    }
    else {
        return false;
    }
    //return false;
}