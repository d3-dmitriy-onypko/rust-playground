#[derive(PartialEq, Debug)]
struct Shoe {
    size: u32,
    style: String,
}

fn shoes_in_size(shoes: &Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    let shoes = shoes.iter().filter(|s| s.size == shoe_size).map(|f| Shoe {
        size: f.size,
        style: f.style.clone(),
    });
    let shoes = shoes.collect();
    shoes
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn filters_by_size() {
        let shoes = vec![
            Shoe {
                size: 10,
                style: String::from("sneaker"),
            },
            Shoe {
                size: 13,
                style: String::from("sandal"),
            },
            Shoe {
                size: 10,
                style: String::from("boot"),
            },
        ];

        let in_my_size = shoes_in_size(&shoes, 10);
        let in_my_size2 = shoes_in_size(&shoes, 10);

        assert_eq!(
            in_my_size,
            vec![
                Shoe {
                    size: 10,
                    style: String::from("sneaker")
                },
                Shoe {
                    size: 10,
                    style: String::from("boot")
                },
            ]
        );
    }
}
