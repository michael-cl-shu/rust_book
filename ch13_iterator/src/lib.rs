#[derive(PartialEq, Debug)]
struct Shoe {
    size: u32,
    style: String,
}

#[cfg(test)]
fn shoes_in_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    shoes.into_iter().filter(|s| s.size == shoe_size).collect()
}

fn list_increment(list: &Vec<u32>, inc: u32) -> Vec<u32> {
    return list.iter().map(|i| i + inc).collect();
}
mod tests {
    #[test]
    fn increment_test() {
        let list = vec![1, 2, 3];
        let result = crate::list_increment(&list, 3);
        assert_eq!(result, vec![4, 5, 6]);
    }

    #[test]
    fn iterator_demonstration() {
        let v1 = vec![1, 2, 3];
        let mut v2 = v1.iter();

        assert_eq!(v2.next(), Some(&1));
        assert_eq!(v2.next(), Some(&2));
        assert_eq!(v2.next(), Some(&3));
        assert_eq!(v2.next(), None);
    }

    #[test]
    fn iterator_sum() {
        let v1 = vec![1, 2, 3];

        let v1_iter = v1.iter();

        let total: i32 = v1_iter.sum();

        assert_eq!(total, 6);
    }

    #[test]
    fn filters_by_size() {
        let shoes = vec![
            crate::Shoe {
                size: 10,
                style: String::from("sneaker"),
            },
            crate::Shoe {
                size: 13,
                style: String::from("sandal"),
            },
            crate::Shoe {
                size: 10,
                style: String::from("boot"),
            },
        ];

        let in_my_size = crate::shoes_in_size(shoes, 10);

        assert_eq!(
            in_my_size,
            vec![
                crate::Shoe {
                    size: 10,
                    style: String::from("sneaker")
                },
                crate::Shoe {
                    size: 10,
                    style: String::from("boot")
                },
            ]
        );
    }
}
