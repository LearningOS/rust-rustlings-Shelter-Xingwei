// move_semantics1.rs
// Execute `rustlings hint move_semantics1` or use the `hint` watch subcommand for a hint.

fn main() {
    let mut vec0 = Vec::new();

    vec0.push(11);
    let mut vec1 = fill_vec(vec0.clone());
    vec0.push(99);
    println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);

    vec1.push(88);

    println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);
    println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec0);
}

fn fill_vec(vec: Vec<i32>) -> Vec<i32> {
    let mut vec: Vec<i32> = vec;

    vec.push(22);
    vec.push(44);
    vec.push(66);

    vec
}
