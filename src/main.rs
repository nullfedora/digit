use std::{thread};
use std::time::Duration;
use std::sync::{Arc, Mutex};

use intermediary::message_queue::{MessageQueue, FrontendMessage, BackendMessage};

mod frontend;
mod backend;
mod intermediary;

fn main() {
    
    println!("Main Thread Started");


    let frontend_message_queue: Arc<Mutex<MessageQueue<FrontendMessage>>> = Arc::new(Mutex::new(MessageQueue::new()));
    let backend_message_queue: Arc<Mutex<MessageQueue<BackendMessage>>> = Arc::new(Mutex::new(MessageQueue::new()));

    //create copies of message queues for the render thread
    let render_frontend_message_queue = frontend_message_queue.clone();
    let render_backend_message_queue = backend_message_queue.clone();

    //start rendering thread
    thread::spawn(move || {
        frontend::main::main(render_frontend_message_queue, render_backend_message_queue);
    });

    thread::sleep(Duration::new(1, 0));
}

#[cfg(test)]
mod test{

}