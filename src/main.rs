use std::{thread};

mod frontend;
mod backend;

fn main() {
    
    println!("Main Thread Started");

    //start rendering thread
    thread::spawn( || {
        frontend::main::main();
    });

}

#[cfg(test)]
mod test{

}