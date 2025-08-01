pub fn quick_sort<T: PartialOrd + Clone>(arr: &[T]) -> Vec<T> {
    let len = arr.len();

    if len <= 1 {
        return arr.to_vec();
    }

    let pivot = arr[len - 1].to_owned();

    let mut left = Vec::new();
    let mut right = Vec::new();

    for item in &arr[..len-1] {  // 排除最后一个元素（pivot）
        if *item < pivot {
            left.push(item.to_owned());
        } else {
            right.push(item.to_owned());
        }
    }

    let mut result = quick_sort(&left);
    result.push(pivot);
    result.extend(quick_sort(&right));

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn quick_sort_works() {
        let arr = [5, 3, 8, 4, 2];
        let sorted = quick_sort(&arr);

        assert_eq!(sorted, &[2, 3, 4, 5, 8]);
    }
}
