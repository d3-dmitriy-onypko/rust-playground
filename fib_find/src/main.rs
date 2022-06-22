fn main() {
    println!("Fib find 10: {}", fib_find(10))
}

fn fib_find(n: u32) -> u32 {
    if n <= 1 {
        return n;
    }
       
    fib_find(n - 1) + fib_find(n - 2)
}
