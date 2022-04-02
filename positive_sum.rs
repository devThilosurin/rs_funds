fn positive_sum(slice: &[i32]) -> i32 {
    if slice.is_empty() {
        return 0;
    }

    slice
        .iter()
        .map(|n| *n)
        .filter(|n| n.is_positive())
        .reduce(|acc, curr| acc + curr)
        .unwrap_or_default()
}

fn main() {
    println!(" -> {:?}", positive_sum(&[1, 2, 3, 4, 5]));
    println!(" -> {:?}", positive_sum(&[1, -2, 3, 4, 5]));
    println!(" -> {:?}", positive_sum(&[-1, 2, 3, 4, -5]));
    println!(" -> {:?}", positive_sum(&[]));
    println!(" -> {:?}", positive_sum(&[-1, -2, -3, -4, -5]));

    let number_matcher = 47;
    match number_matcher {
        x if x > 0 => println!("positive number"),
        x if 0 > x => println!("negative number"),
        0 => println!("zero"),
        _ => panic!(),
    }
}

fn solution(arr: &[i32]) -> i32 {
    arr.iter().filter(|x| x.is_positive()).sum()
}
