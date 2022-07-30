mod data_structure;
mod hashtable;
mod tree;

fn main() {
    println!("Hello, world!");
    let a = hashtable::q1::Solution1::two_sum(vec![1, 2, 3], 59);
    println!("{:?}", a);
}
