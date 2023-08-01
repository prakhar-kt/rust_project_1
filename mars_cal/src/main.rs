use std::io;

fn main() {
    let mut input = String::new();
    some_fn(&input);
    io::stdin().read_line(&mut input);
    let mars_weight: f32 = calculate_weight_on_mars(100.0);
    println!("Weight on Mars : {}", mars_weight);
    
}

fn calculate_weight_on_mars(weight: f32) -> f32 {
    (weight * 9.86) / 3.711 
}

fn some_fn(s: &mut String) {

    s.push_str("a")

}