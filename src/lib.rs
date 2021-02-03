use rand::{distributions::Uniform, Rng};

pub fn generate_random_numbers(count: i64) -> Vec<i64> {
    let mut rng = rand::thread_rng();
    let number_range = Uniform::new(-1000, 1000);

    (0..count + 1).map(|_| rng.sample(&number_range)).collect()
}

pub fn bubble_sort(arr: &Vec<i64>) -> Vec<i64> {
    let mut sorted = false;
    let mut sorted_arr = arr.clone();

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bubble_sort_sorts() {
        let random_nums = vec![697, 1, 86, 885, -460, -291, 836, 197, -180, 307, 779];
        assert_eq!(
            bubble_sort(&random_nums),
            [-460, -291, -180, 1, 86, 197, 307, 697, 779, 836, 885],
        );
    }
}
