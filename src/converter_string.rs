use std::str::FromStr;

pub fn converter_string() {
    let x = "321";
    let a: i32 = x.parse().unwrap();
    let b = x.parse::<i32>().unwrap();
    let c = i32::from_str(x).unwrap();

    println!("{a} {b} {c}");
}
