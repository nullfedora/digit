

use sdl2;
use sdl2::event::Event;
//::image::{self, InitFlag};
use crate::intermediary::message_queue::{MessageQueue, FrontendMessage, BackendMessage};
use std::thread;
use std::time::Duration;
use std::sync::{Arc, Mutex};

/// This is the "main function" for the rendering thread.  This is called once from main and everything else rendering related happens here.
/// the message queues are used to communicate between threads
pub fn main(frontend_message_queue: Arc<Mutex<MessageQueue<FrontendMessage>>>, backend_message_queue: Arc<Mutex<MessageQueue<BackendMessage>>>){
    println!("Rendering Thread Started");

    //initialize sdl2 window

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    //let _image_context = image::init(InitFlag::PNG | InitFlag::JPG).unwrap();

    let window = video_subsystem
        .window("Engine", 1000, 1000)
        .position_centered()
        .build()
        .expect("Failed to initialize window");

    let mut canvas = window.into_canvas().build().expect("Could not make canvas");

    
    //initialize sdl2 event pump and begin main loop
    let mut event_pump = sdl_context.event_pump().unwrap();

    loop{

        //process messages from main thread
        let mut incoming_queue = backend_message_queue.lock().unwrap();
   
        while !incoming_queue.is_empty() {
            let message = incoming_queue.get_message();

            println!("Frontend recieved message!");

        }

        drop(incoming_queue);


        let mut should_quit: bool = false;

        //process events from sdl
        for event in event_pump.poll_iter(){
            match event{
                Event::Quit{ .. } => {
                    should_quit = true;
                },
                _ => {}
            }
        }

        if should_quit {
            let mut outgoing_queue = frontend_message_queue.lock().unwrap();

            outgoing_queue.add_message(FrontendMessage::UserQuit);
        }

        //sleep for a bit
        thread::sleep(Duration::from_millis(50));
    }
}