pub fn bubble_sort<T: PartialOrd>(arr: &mut [T]) -> &[T] {
    let len = arr.len();

    for i in 0..len {
        for j in 0..len - 1 - i {
            if arr[j] > arr[j + 1] {
                arr.swap(j, j + 1);
            }
        }
    }

    arr
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bubble_sort_works() {
        let mut arr = [5, 3, 8, 4, 2];
        let sorted = bubble_sort(&mut arr);

        assert_eq!(sorted, &[2, 3, 4, 5, 8]);
    }
}
