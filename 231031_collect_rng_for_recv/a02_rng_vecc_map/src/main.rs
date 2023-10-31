use rand::Rng;

fn main() {
    let v: i32 = (1..5)
        .collect::<Vec<i32>>()
        .iter()
        .map(|&x: &i32| x.pow(2))
        .sum();

    let my_vec: Vec<i32> = (1..10)
        .collect::<Vec<i32>>()
        .iter()
        .map(|&x: &i32| x.pow(2))
        .collect();

    let vals: Vec<u64> = (0..10).map(|v| v + 1000).collect();

    let mut rng = rand::thread_rng();
    let rng_vals: Vec<u64> = (0..10).map(|_| rng.gen_range(0..10)).collect();

    println!("{v}");
    println!("{my_vec:?}");
    println!("{vals:?}");
    println!("{rng_vals:?}");
}
