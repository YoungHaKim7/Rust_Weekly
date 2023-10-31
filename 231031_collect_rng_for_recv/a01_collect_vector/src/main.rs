fn main() {
    let v = (1..5).collect::<Vec<i32>>();
    for _ in v.iter() {
        println!("{v:?}");
    }
    let mut i = 0;
    while i < v.len() {
        println!("{:?}", &v[i..i + 1]);
        i += 1;
    }
}
