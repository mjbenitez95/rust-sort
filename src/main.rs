use sort::generate_random_numbers;

fn main() {
    let vec = generate_random_numbers(10);
    let bubble_sorted_vec = sort::bubble_sort(&vec);

    println!("{:?} || {:?}", vec, bubble_sorted_vec);
}
