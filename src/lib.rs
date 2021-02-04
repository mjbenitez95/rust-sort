use rand::{distributions::Uniform, Rng};
use std::cmp::Ord;

pub fn generate_random_numbers(count: i64) -> Vec<i64> {
    let mut rng = rand::thread_rng();
    let number_range = Uniform::new(-1000, 1000);

    (0..count + 1).map(|_| rng.sample(&number_range)).collect()
}

pub fn run<T: Ord + Clone + Copy>(vec: &Vec<T>) {
    let _standard_sorted_vec = standard_sort(&vec);
    let _bubble_sorted_vec = bubble_sort(&vec);
    let _insertion_sorted_vec = insertion_sort(&vec);
    let _selection_sorted_vec = selection_sort(&vec);
}

fn standard_sort<T: Ord + Clone>(arr: &Vec<T>) -> Vec<T> {
    let mut sorted_arr = arr.to_vec();
    sorted_arr.sort();
    sorted_arr
}

fn bubble_sort<T: Ord + Clone>(arr: &Vec<T>) -> Vec<T> {
    let mut sorted_arr = arr.to_vec();
    let mut sorted = false;

    while !sorted {
        sorted = true;
        for i in 1..sorted_arr.len() {
            if sorted_arr[i - 1] > sorted_arr[i] {
                sorted_arr.swap(i - 1, i);
                sorted = false;
            }
        }
    }

    sorted_arr
}

fn insertion_sort<T: Ord + Clone>(arr: &Vec<T>) -> Vec<T> {
    let mut sorted_arr = arr.to_vec();
    for i in 1..sorted_arr.len() {
        let mut j = i;
        while j > 0 && sorted_arr[j] < sorted_arr[j - 1] {
            sorted_arr.swap(j, j - 1);
            j = j - 1;
        }
    }
    sorted_arr
}

fn selection_sort<T: Ord + Clone + Copy>(arr: &Vec<T>) -> Vec<T> {
    let mut unsorted_arr = arr.to_vec();
    let mut sorted_arr = Vec::with_capacity(unsorted_arr.len());

    while !unsorted_arr.is_empty() {
        let mut min_index = 0;
        let mut min_value = unsorted_arr[0];
        for (index, value) in unsorted_arr.iter().enumerate() {
            if value < &min_value {
                min_index = index;
                min_value = *value;
            }
        }
        sorted_arr.push(min_value);
        unsorted_arr.swap_remove(min_index);
    }

    sorted_arr
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn standard_sort_sorts() {
        let random_nums = vec![697, 1, 86, 885, -460, -291, 836, 197, -180, 307, 779];
        assert_eq!(
            standard_sort(&random_nums),
            [-460, -291, -180, 1, 86, 197, 307, 697, 779, 836, 885],
        );
    }

    #[test]
    fn bubble_sort_sorts() {
        let random_nums = vec![697, 1, 86, 885, -460, -291, 836, 197, -180, 307, 779];
        assert_eq!(
            bubble_sort(&random_nums),
            [-460, -291, -180, 1, 86, 197, 307, 697, 779, 836, 885],
        );
    }

    #[test]
    fn insertion_sort_sorts() {
        let random_nums = vec![697, 1, 86, 885, -460, -291, 836, 197, -180, 307, 779];
        println!("{:?}", insertion_sort(&random_nums));
        assert_eq!(
            insertion_sort(&random_nums),
            [-460, -291, -180, 1, 86, 197, 307, 697, 779, 836, 885],
        );
    }

    #[test]
    fn selection_sort_sorts() {
        let random_nums = vec![697, 1, 86, 885, -460, -291, 836, 197, -180, 307, 779];
        println!("{:?}", selection_sort(&random_nums));
        assert_eq!(
            selection_sort(&random_nums),
            [-460, -291, -180, 1, 86, 197, 307, 697, 779, 836, 885],
        );
    }
}
