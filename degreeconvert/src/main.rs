use std::io;

fn main() {
    loop
    {
        println!("Introduce los grados en Farenheit:");
        let mut farenheit = String::new();
        io::stdin().read_line(&mut farenheit)
            .expect("Failed to read line");
        let farenheit: f32 = match farenheit.trim().parse(){
            Ok(num)=>num,
            Err(_)=>continue,
        };
        let celsius: f32 = (farenheit-32.0)/1.8;
        println!("La cantidad en Celisus es: {} °C", celsius);
    }
}
