

use crate::intermediary::message_queue::{MessageQueue, FrontendMessage, BackendMessage};
use std::thread;
use std::time::Duration;
use std::sync::{Arc, Mutex};

use glfw::{Action, Context, Key};

use crate::frontend::wgpu_state;
use crate::frontend::rendering::render_state;

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

    let mut render_state = render_state::RenderState::new();

    //for debugging purposes
    let mut rendering_total_time = Duration::from_secs(0);
    let mut times_rendered: u32 = 0;

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

        let mut messages_for_backend: Vec<FrontendMessage> = Vec::new();

        //process events from glfw
        glfw.poll_events();
        for (_, event) in glfw::flush_messages(&events){
            match event{
                glfw::WindowEvent::Size(x, y) => {
                    let result = wgpu_state.resize((x, y));
                    match result {
                        Ok(..) => {//resize did not fail
                        },
                        Err(message) => {
                            messages_for_backend.push(FrontendMessage::DebugMessage(Box::new(message.to_string())));
                        }
                    }
                },
                glfw::WindowEvent::Close => {
                    should_quit = true;
                    println!("Closing!")
                },
                _ => {}
            }
        }

        if should_quit {
            messages_for_backend.push(FrontendMessage::UserQuit);

            //print out the average time rendering took
            println!("Rendering took an average of {:?}", rendering_total_time.checked_div(times_rendered));

            window.set_should_close(true);
        }

        //send messages to backend
        {
            let mut outgoing_queue = frontend_message_queue.lock().unwrap();
            for message in messages_for_backend{
                outgoing_queue.add_message(message.clone());
            }
        }

        //render the screen
        let now = std::time::Instant::now();
        crate::frontend::rendering::render::render(&wgpu_state, &render_state);
        rendering_total_time += now.elapsed();
        times_rendered += 1;


        //sleep for a bit
        thread::sleep(Duration::from_millis(50));
    }
}