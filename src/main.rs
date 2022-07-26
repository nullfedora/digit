use std::{thread};

mod frontend;
mod backend;

fn main() {
    
    println!("Main Thread Started");

    //start rendering thread
    thread::spawn( || {
        frontend::main::main();
    });

    thread::sleep_ms(2);
}

#[cfg(test)]
mod test{

}