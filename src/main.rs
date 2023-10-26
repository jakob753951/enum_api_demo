mod demo1;
mod demo2;

fn main() {
    println!("Trying the current solution");
    demo1::main();
    
    println!();
    println!("Trying the Rust way");
    demo2::main();
}
