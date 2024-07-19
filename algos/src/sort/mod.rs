pub mod bubble;
pub mod bucket;
pub mod cocktail;
pub mod comb;
pub mod counting;
pub mod gnome;
pub mod heap;
pub mod insertion;
pub mod merge;

fn is_sorted<T: PartialOrd>(arr: &[T]) -> bool {
    if arr.is_empty() {
        return true;
    }
    for i in 0..(arr.len() - 1) {
        if arr[i] > arr[i + 1] {
            return false;
        }
    }
    true
}
