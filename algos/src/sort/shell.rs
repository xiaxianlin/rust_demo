fn insertion<T: Ord + Copy>(values: &mut Vec<T>, start: usize, gap: usize) {
    for i in ((start + gap)..values.len()).step_by(gap) {
        let val_current = values[i];
        let mut pos = i;
        while pos >= gap && values[pos - gap] > val_current {
            values[pos] = values[pos - gap];
            pos -= gap;
        }
        values[pos] = val_current;
    }
}

pub fn shell_sort<T: Ord + Copy>(values: &mut Vec<T>) {
    let mut count_sub_list = values.len() / 2;
    while count_sub_list > 0 {
        for pos_start in 0..count_sub_list {
            insertion(values, pos_start, count_sub_list);
        }
        count_sub_list /= 2;
    }
}

#[cfg(test)]
mod test {
    use super::shell_sort;

    #[test]
    fn basic() {
        let mut vec = vec![3, 5, 6, 3, 1, 4];
        shell_sort(&mut vec);
        for i in 0..vec.len() - 1 {
            assert!(vec[i] <= vec[i + 1]);
        }
    }

    #[test]
    fn empty() {
        let mut vec: Vec<i32> = vec![];
        shell_sort(&mut vec);
        assert_eq!(vec, vec![]);
    }

    #[test]
    fn reverse() {
        let mut vec = vec![6, 5, 4, 3, 2, 1];
        shell_sort(&mut vec);
        for i in 0..vec.len() - 1 {
            assert!(vec[i] <= vec[i + 1]);
        }
    }

    #[test]
    fn already_sorted() {
        let mut vec = vec![1, 2, 3, 4, 5, 6];
        shell_sort(&mut vec);
        for i in 0..vec.len() - 1 {
            assert!(vec[i] <= vec[i + 1]);
        }
    }
}
