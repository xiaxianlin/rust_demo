use std::{cmp, slice::Windows};

fn knapsack_table(w: &usize, weights: &[usize], values: &[usize]) -> Vec<Vec<usize>> {
    let n = weights.len();

    let mut m = vec![vec![0; w + 1]; n + 1];
    for i in 0..=n {
        for j in 0..=*w {
            if i == 0 || j == 0 {
                m[i][j] = 0;
            } else if weights[i - 1] <= j {
                m[i][j] = cmp::max(values[i - 1] + m[i - 1][j - weights[i - 1]], m[i - 1][j]);
            } else {
                m[i][j] = m[i - 1][j];
            }
        }
    }

    m
}

fn knapsack_items(weights: &[usize], m: &[Vec<usize>], i: usize, j: usize) -> Vec<usize> {
    if i == 0 {
        return vec![];
    }
    if m[i][j] > m[i - 1][j] {
        let mut knap = knapsack_items(weights, m, i - 1, j - weights[i - 1]);
        knap.push(i);
        knap
    } else {
        knapsack_items(weights, m, i - 1, j)
    }
}

pub fn knapsack(w: usize, weights: Vec<usize>, values: Vec<usize>) -> (usize, usize, Vec<usize>) {
    assert_eq!(weights.len(), values.len(), "Number of items in the list of weights doesn't match the number of items in the list of values!");

    let n = weights.len();
    let m = knapsack_table(&w, &weights, &values);
    let items = knapsack_items(&weights, &m, n, w);
    let mut total_weight = 0;

    for i in items.iter() {
        total_weight += weights[i - 1];
    }

    (m[n][w], total_weight, items)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p02() {
        assert_eq!(
            (51, 26, vec![2, 3, 4]),
            knapsack(26, vec![12, 7, 11, 8, 9], vec![24, 13, 23, 15, 16])
        );
    }

    #[test]
    fn test_p04() {
        assert_eq!(
            (150, 190, vec![1, 2, 5]),
            knapsack(
                190,
                vec![56, 59, 80, 64, 75, 17],
                vec![50, 50, 64, 46, 50, 5]
            )
        );
    }

    #[test]
    fn test_p01() {
        assert_eq!(
            (309, 165, vec![1, 2, 3, 4, 6]),
            knapsack(
                165,
                vec![23, 31, 29, 44, 53, 38, 63, 85, 89, 82],
                vec![92, 57, 49, 68, 60, 43, 67, 84, 87, 72]
            )
        );
    }

    #[test]
    fn test_p06() {
        assert_eq!(
            (1735, 169, vec![2, 4, 7]),
            knapsack(
                170,
                vec![41, 50, 49, 59, 55, 57, 60],
                vec![442, 525, 511, 593, 546, 564, 617]
            )
        );
    }

    #[test]
    fn test_p07() {
        assert_eq!(
            (1458, 749, vec![1, 3, 5, 7, 8, 9, 14, 15]),
            knapsack(
                750,
                vec![70, 73, 77, 80, 82, 87, 90, 94, 98, 106, 110, 113, 115, 118, 120],
                vec![135, 139, 149, 150, 156, 163, 173, 184, 192, 201, 210, 214, 221, 229, 240]
            )
        );
    }
}
