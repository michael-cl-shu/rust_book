#[cfg(test)]
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
}
