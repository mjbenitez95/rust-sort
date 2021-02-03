use sort::generate_random_numbers;

fn main() {
    let vec = generate_random_numbers(1000);
    let _bubble_sorted_vec = sort::bubble_sort(&vec);
}
