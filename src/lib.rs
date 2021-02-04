use rand::{distributions::Uniform, Rng};
use std::cmp::Ord;

pub fn generate_random_numbers(count: i64) -> Vec<i64> {
    let mut rng = rand::thread_rng();
    let number_range = Uniform::new(-1000, 1000);

    (0..count + 1).map(|_| rng.sample(&number_range)).collect()
}

pub fn run<T: Ord + Clone + Copy>(vec: &Vec<T>) {
    let _standard_sorted_vec = run_sort(vec, standard_sort);
    let _bubble_sorted_vec = run_sort(vec, bubble_sort);
    let _insertion_sorted_vec = run_sort(vec, insertion_sort);
    let _selection_sorted_vec = run_sort(vec, selection_sort);
}

pub fn run_sort<T: Ord + Clone + Copy>(vec: &Vec<T>, sort: fn(&mut Vec<T>)) -> Vec<T> {
    let mut new_vec = vec.to_vec();
    sort(&mut new_vec);
    new_vec
}

fn standard_sort<T: Ord + Clone + Copy>(arr: &mut Vec<T>) {
    arr.sort();
}

fn bubble_sort<T: Ord + Clone + Copy>(arr: &mut Vec<T>) {
    let mut sorted = false;

    while !sorted {
        sorted = true;
        for i in 1..arr.len() {
            if arr[i - 1] > arr[i] {
                arr.swap(i - 1, i);
                sorted = false;
            }
        }
    }
}

fn insertion_sort<T: Ord + Clone + Copy>(arr: &mut Vec<T>) {
    for i in 1..arr.len() {
        let mut j = i;
        while j > 0 && arr[j] < arr[j - 1] {
            arr.swap(j, j - 1);
            j = j - 1;
        }
    }
}

fn selection_sort<T: Ord + Clone + Copy>(arr: &mut Vec<T>) {
    for i in 0..arr.len() {
        let mut small = i;
        for j in (i + 1)..arr.len() {
            if arr[j] < arr[small] {
                small = j;
            }
        }
        arr.swap(small, i);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_test_values() -> (Vec<i64>, Vec<i64>) {
        let random_nums = vec![697, 1, 86, 885, -460, -291, 836, 197, -180, 307, 779];
        let sorted_nums = vec![-460, -291, -180, 1, 86, 197, 307, 697, 779, 836, 885];
        (random_nums, sorted_nums)
    }

    #[test]
    fn standard_sort_sorts() {
        let (random_nums, sorted_nums) = get_test_values();
        assert_eq!(run_sort(&random_nums, standard_sort), sorted_nums);
    }

    #[test]
    fn bubble_sort_sorts() {
        let (random_nums, sorted_nums) = get_test_values();
        assert_eq!(run_sort(&random_nums, bubble_sort), sorted_nums);
    }

    #[test]
    fn insertion_sort_sorts() {
        let (random_nums, sorted_nums) = get_test_values();
        assert_eq!(run_sort(&random_nums, insertion_sort), sorted_nums);
    }

    #[test]
    fn selection_sort_sorts() {
        let (random_nums, sorted_nums) = get_test_values();
        assert_eq!(run_sort(&random_nums, selection_sort), sorted_nums);
    }
}
