use::std::io;

fn main() {
    loop
    {
        println!("Escribe n de la serie Fibonacci: ");
        let mut n = String::new();
        io::stdin().read_line(&mut n)
            .expect("Failed to read line");
        let n: u32 = match n.trim().parse(){
            Ok(num)=>num,
            Err(_)=>continue,
        };
        let golden_ratius: f32 = 1.618034;
        let x: f32 = 5.0;
        let fibonacci: f32 = (((golden_ratius).powf(n as f32))-(1.0-golden_ratius).powf(n as f32))/x.sqrt();
        println!("El número {} en la serie Fibonacci es: {}",n,fibonacci as u32);
    }
}
