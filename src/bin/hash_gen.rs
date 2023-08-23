use rand::Rng;

fn main() {
    let mut rng = rand::thread_rng();
    
    let mut board_hash: [[u64; 5]; 32] = [[0; 5]; 32];
    for i in 0..32 {
        for j in 0..5 {
            board_hash[i][j] = rng.gen();
        }
    }
    
    let white_turn_hash: u64 = rng.gen();
    let black_turn_hash: u64 = rng.gen();
    
    println!("{:?}", board_hash);

    println!("White Turn Hash: {}", white_turn_hash);
    println!("Black Turn Hash: {}", black_turn_hash);
}