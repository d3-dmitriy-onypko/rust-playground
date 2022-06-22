use std::io;

fn main() {
    loop {
        println!("Enter value");

        let number = match read_from_input() {
            Ok(k) => k,
            Err(_) => {
                println!("wrong value");
                continue;
            }
        };

        loop {
            println!("Please enter variant");
            println!("Convert to fahrenheit: 1");
            println!("Conver to celsius: 2");
            
            let variant = match read_from_input() {
                Ok(k) => k,
                Err(_) => {
                    println!("wrong value");
                    continue;
                }
            };

            match variant {
                1 => {
                    println!("result is {}F", number as f32 * 9.0/5.0 + 32.0);
                    continue;
                }
                2 => {
                    println!("result is {}C", (number as f32 - 32.0) * 5.0/9.0);
                    continue;
                }
                _ => {continue}
            }
        }
    }
}

fn read_from_input() -> Result<i32, &'static str> {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).expect("failed to read");
    match buffer.trim_end().parse() {
        Ok(k) => return Ok(k),
        Err(_) => return Err("Invalid value"),
    };
}
