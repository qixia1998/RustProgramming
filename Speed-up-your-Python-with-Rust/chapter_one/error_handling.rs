fn main() {
    let mut str_vector: Vec<&str> = vec!["one", "two", "three"];
    println!("{}", str_vector.len());
    str_vector.push("four");
    for i in str_vector.iter() {
        println!("{}", i);
    }
}