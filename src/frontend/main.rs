

use sdl2;//::image::{self, InitFlag};
use crate::intermediary::message_queue::{MessageQueue, FrontendMessage, BackendMessage};

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
}