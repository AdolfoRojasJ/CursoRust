fn main() {
    let s1 = String::from("hello");
    let s2 = s1;
    let s3 = s2.clone();
    println!("{}, world!, {}", s2,s3);
}
