extern "C" {
    fn abs(input: i32) -> i32;
}

fn main() {
    foo();
}

fn foo() -> ! {
    panic!();
}
