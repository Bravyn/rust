use std::io;

fn main() {
loop{
    println!("Type in the bytes below:\n");
    let mut bytes = String::new();
    io::stdin().read_line(&mut bytes).unwrap();
    
    let buff: &[u8] = bytes.as_bytes();
    
    println!("\n{:?}\n", buff);

    let mut bytes = String::new();
    io::stdin().read_line(&mut bytes).unwrap();
}
}
