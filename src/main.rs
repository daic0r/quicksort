

fn main() {
    let mut v = vec![6, 99, 12, 12, 333, 1, 1024, 34];
    quicksort::lib::quicksort(&mut v);
    println!("{:?}", &v);
}
