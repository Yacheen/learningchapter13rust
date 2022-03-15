#[derive(PartialEq, Debug)]
pub struct Shoe {
    size: u32,
    style: String
}
pub struct Counter {
    count: u32,
}

impl Counter {
    fn new() -> Counter {
        Counter {count: 0}
    }
}

pub fn shoes_in_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    shoes.into_iter().filter(|s| s.size == shoe_size).collect()
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn filters_by_size() {
        let shoes = vec![
            Shoe {
                size: 6,
                style: String::from("adidas")
            },
            Shoe {
                size: 9,
                style: String::from("Rebok")

            },
            Shoe {
                size: 69,
                style: String::from("FashionScape")
            }
        ];

        let in_my_size = shoes_in_size(shoes, 69);
            println!("{:#?}", in_my_size);
        assert_eq!(
            in_my_size,
            vec![
                Shoe {
                    size: 69,
                    style: String::from("FashionScape")
                }
            ]
        );
    }
}