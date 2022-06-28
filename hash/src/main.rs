use std::collections::HashMap;
fn main() {
    let numbers = [42, 1, 36, 34, 76, 378, 43, 1, 43, 54, 2, 3, 43];

    let avg = numbers.iter().sum::<i32>() as f64 / numbers.len() as f64;

    let mut sorted = numbers.clone();
    sorted.sort();
    let mut resultString = (&sorted[0]).to_string();
    sorted
        .iter()
        .skip(1)
        .for_each(|x| resultString = format!("{}, {}", resultString, x));
    print!("{}", resultString);

    let owner = sorted[0];
    print!("{}", owner);
    let mid = sorted.len() / 2;

    let median = sorted[mid];

    let mut times = HashMap::new();

    for ele in numbers {
        *times.entry(ele).or_insert(0) += 1;
    }

    let result = times
        .iter()
        .max_by_key(|&(_, count)| count)
        .map(|(k, _)| k)
        .expect("eeeee");

    println!("AVERAGE: {}", avg);
    println!("MEDIAN: {}", median);
    println!("max: {}", result)
}
