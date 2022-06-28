use std::io;

fn main() {
    const CONSONANTS: [&str; 21] = ["b", "c", "d", "f", "g", "h", "j", "k", "l", "m", "n", "p", "q", "r", "s", "t", "v", "w", "x", "y", "z"];
    const VOWELS: [&str; 5] = ["a", "e", "i", "o", "u"];

    let mut words = Vec::new();
    loop {
        let mut input = String::new();
        println!("Enter a sentence: ");
        if let Err(error) = io::stdin().read_line(&mut input) {
            println!("Error: {}", error);
            continue;
        }

        let sentence_splitted:Vec<String> = input.trim().split_whitespace().map(String::from).collect();
        match sentence_splitted.len() {
            0 => {
                println!("You have entered an empty sentence.");
                continue;
            },
            1 => {
                println!("You have entered a single word.");
                continue;
            },
            _ => {
                words = sentence_splitted;
                break;
            },
        }
    }

    let mut piglatin_words = Vec::new();

    for word in words {
        let first_char = word.chars().nth(0).unwrap();
        if CONSONANTS.contains(&first_char.to_lowercase().to_string().as_str()) {
            let except_first_char = word.chars().skip(1).collect::<String>();
            piglatin_words.push(format!("{}-{}ay", except_first_char, first_char));
        }
        else {
            piglatin_words.push(format!("{}-ay", word));
        }
    }

    dbg!(piglatin_words);



// fn get_words_from_input() -> Vec<&'static str> {
//     loop {
//         let mut input = String::new();
//         println!("Enter a sentence: ");
//         if let Err(error) = io::stdin().read_line(&mut input) {
//             println!("Error: {}", error);
//             continue;
//         }

//         let sentence_splitted = input.trim().split_whitespace().collect::<Vec<&str>>();
//         match sentence_splitted.len() {
//             0 => {
//                 println!("You have entered an empty sentence.");
//                 continue;
//             },
//             1 => {
//                 println!("You have entered a single word.");
//                 continue;
//             },
//             _ => {
//                 return sentence_splitted
//             },
//         }
//     }
}
