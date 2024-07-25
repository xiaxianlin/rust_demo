fn partition<T: PartialOrd>(arr: &mut [T], lo: usize, hi: usize) -> usize {
    let pivot = lo;
    let (mut left, mut right) = (lo, hi);
    while left < right {
        while left < right && arr[right] >= arr[pivot] {
            right -= 1;
        }

        while left < right && arr[left] <= arr[pivot] {
            left += 1;
        }

        if left != right {
            arr.swap(left, right);
        }
    }
    arr.swap(pivot, left);
    left
}

fn partition_random<T: PartialOrd>(arr: &mut [T], lo: usize, hi: usize) -> usize {
    use rand::Rng;
    let mut rng = rand::thread_rng();
    let pivot = rng.gen_range(lo..=hi);
    arr.swap(lo, pivot);

    let pivot = lo;
    let (mut left, mut right) = (lo, hi);
    while left < right {
        while left < right && arr[right] >= arr[pivot] {
            right -= 1;
        }

        while left < right && arr[left] <= arr[pivot] {
            left += 1;
        }

        if left != right {
            arr.swap(left, right);
        }
    }
    arr.swap(pivot, left);
    left
}

fn quick_sort_range<T: PartialOrd>(arr: &mut [T], lo: usize, hi: usize) {
    if lo < hi {
        let pos = partition(arr, lo, hi);
        // let pos = partition_random(arr, lo, hi);
        if pos != 0 {
            quick_sort_range(arr, lo, pos - 1);
        }
        quick_sort_range(arr, pos + 1, hi);
    }
}

pub fn quick_sort<T: PartialOrd>(arr: &mut [T]) {
    if arr.len() > 1 {
        quick_sort_range(arr, 0, arr.len() - 1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_vec() {
        let mut empty_vec: Vec<String> = vec![];
        quick_sort(&mut empty_vec);
        assert_eq!(empty_vec, Vec::<String>::new());
    }

    #[test]
    fn test_number_vec() {
        let mut vec = vec![7, 49, 73, 58, 30, 72, 44, 78, 23, 9];
        quick_sort(&mut vec);
        assert_eq!(vec, vec![7, 9, 23, 30, 44, 49, 58, 72, 73, 78]);
    }

    #[test]
    fn test_string_vec() {
        let mut vec = vec![
            String::from("Bob"),
            String::from("David"),
            String::from("Carol"),
            String::from("Alice"),
        ];
        quick_sort(&mut vec);
        assert_eq!(
            vec,
            vec![
                String::from("Alice"),
                String::from("Bob"),
                String::from("Carol"),
                String::from("David"),
            ]
        );
    }
}
