use std::env;
fn main() {
    let args : Vec<String> = env::args().collect();
    let program = args[0].clone();
    for i in &args {
        println!("{}",i);
    }
    println!("program {:?}",program);
}
