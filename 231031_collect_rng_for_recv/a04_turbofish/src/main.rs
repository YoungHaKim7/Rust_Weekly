fn main() {
    let v = (1..5).collect::<Vec<i32>>();

    let results = [Ok(1), Err("nope"), Ok(3), Err("bad")];

    let result: Result<Vec<_>, &str> = results.iter().cloned().collect();

    // gives us the first error
    println!("{result:?}");

    let results = [Ok(1), Ok(3)];

    let result: Result<Vec<_>, &str> = results.iter().cloned().collect();

    // gives us the list of answers
    println!("{result:?}");

    let mut i = 0;
    while i < v.len() {
        println!("{:?}", &v[i..i + 1]);
        i += 1;
    }

    for _ in v.iter() {
        println!("{v:?}");
    }
}
