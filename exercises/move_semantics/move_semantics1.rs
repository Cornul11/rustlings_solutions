// move_semantics1.rs
// Execute `rustlings hint move_semantics1` or use the `hint` watch subcommand for a hint.

fn main() {
    let vec0 = Vec::new();


    let vec0_copy = vec0;
    let mut vec1 = fill_vec(&vec0_copy);

    println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);

    vec1.push(88);

    println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);
}

fn fill_vec(vec: &Vec<i32>) -> Vec<i32> {
    let mut new_vec = vec.to_vec();

    new_vec.push(22);
    new_vec.push(44);
    new_vec.push(66);

    new_vec
}
