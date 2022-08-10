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

    //create copies of message queues for the render thread.  these need to be copied because they're moved 
    let render_frontend_message_queue = frontend_message_queue.clone();
    let render_backend_message_queue = backend_message_queue.clone();

    //start rendering thread
    thread::spawn(move || {
        frontend::main::main(render_frontend_message_queue, render_backend_message_queue);
    });

    backend_message_queue.lock().unwrap().add_message(BackendMessage::TestMessage);

    'running: loop{

        //process messages from frontend
        let mut incoming_queue = frontend_message_queue.lock().unwrap();
   
        while !incoming_queue.is_empty() {
            let message = incoming_queue.get_message();

            match message{
                FrontendMessage::UserQuit => {
                    break 'running;
                },
                FrontendMessage::DebugMessage(message) => {
                    println!("[DEBUG] [FRONTEND]: {:?}", message)
                }
                _ => {}
            }

        }

        drop(incoming_queue);    
        
        thread::sleep(Duration::from_millis(10));
    }
}

#[cfg(test)]
mod test{

}