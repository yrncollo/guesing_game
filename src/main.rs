use rand::Rng;
mod game_logic;
fn main() {

    let secret_number = rand::thread_rng().gen_range(1..=101);

    println!("The secret_number is {}",secret_number);

    game_logic::run(secret_number);

}
