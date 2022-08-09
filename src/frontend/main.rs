

use crate::intermediary::message_queue::{MessageQueue, FrontendMessage, BackendMessage};
use std::thread;
use std::time::Duration;
use std::sync::{Arc, Mutex};

use glfw::{Action, Context, Key};

use crate::frontend::wgpu_state;

/// This is the "main function" for the rendering thread.  This is called once from main and everything else rendering related happens here.
/// the message queues are used to communicate between threads
pub fn main(frontend_message_queue: Arc<Mutex<MessageQueue<FrontendMessage>>>, backend_message_queue: Arc<Mutex<MessageQueue<BackendMessage>>>){
    println!("Rendering Thread Started");

    //initialize glfw window

    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();

    let (mut window, events) = glfw.create_window(640, 480, "Digit", glfw::WindowMode::Windowed)
        .expect("Failed to create window!");

    
    window.set_pos_polling(true);
    window.set_all_polling(true);
    window.set_size_polling(true);
    window.set_close_polling(true);
    window.set_refresh_polling(true);
    window.set_focus_polling(true);
    window.set_iconify_polling(true);
    window.set_framebuffer_size_polling(true);
    window.set_key_polling(true);
    window.set_char_polling(true);
    window.set_char_mods_polling(true);
    window.set_mouse_button_polling(true);
    window.set_cursor_pos_polling(true);
    window.set_cursor_enter_polling(true);
    window.set_scroll_polling(true);
    window.set_maximize_polling(true);
    window.set_content_scale_polling(true);

    window.make_current();


    let mut wgpu_state = pollster::block_on(wgpu_state::WGPUState::new(&window));

    loop{

        //process messages from main thread
        let mut incoming_queue = backend_message_queue.lock().unwrap();
   
        while !incoming_queue.is_empty() {
            let message = incoming_queue.get_message();

            println!("Frontend recieved message!");

        }

        drop(incoming_queue);


        let mut should_quit: bool = false;

        window.swap_buffers();

        //process events from glfw
        glfw.poll_events();
        for (_, event) in glfw::flush_messages(&events){
            println!("{:?}", event);
            match event{
                glfw::WindowEvent::Size(x, y) => {
                    wgpu_state.resize((x, y));
                },
                glfw::WindowEvent::Close => {
                    should_quit = true;
                    println!("Closing!")
                },
                _ => {}
            }
        }

        if should_quit {
            let mut outgoing_queue = frontend_message_queue.lock().unwrap();

            outgoing_queue.add_message(FrontendMessage::UserQuit);

            window.set_should_close(true);
        }

        //sleep for a bit
        thread::sleep(Duration::from_millis(50));
    }
}