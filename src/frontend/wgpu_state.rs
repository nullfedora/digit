use wgpu;

///Handles the surface created with WGPU, and the device configuration.  
/// Also handles resizing the surface in case of a window resize.
pub struct WGPUState{
    pub surface: wgpu::Surface,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub config: wgpu::SurfaceConfiguration,
    pub size: (i32, i32),
    pub render_pipeline: wgpu::RenderPipeline
}

impl WGPUState{
    //Create a new wgpustate from a glfw window.  Taken from https://sotrh.github.io/learn-wgpu/beginner/tutorial2-surface/#state-new
    pub async fn new(window: &glfw::Window) -> WGPUState{
        
        //Get size of window
        let size = window.get_size();

        //Create surface from window
        let instance = wgpu::Instance::new(wgpu::Backends::all());
        let surface = unsafe { instance.create_surface(window) };

        //Create adapter from surface
        let adapter = instance.request_adapter(
            &wgpu::RequestAdapterOptions{
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: Some(&surface),
                force_fallback_adapter: true //Tutorial uses false but that crashes for some reason
            }
        ).await.unwrap();

        //Get device and queue from adapter
        let (device, queue) = adapter.request_device(
            &wgpu::DeviceDescriptor {
                features: wgpu::Features::empty(),
                limits: wgpu::Limits::default(),
                label: None
            },
            None
        ).await.unwrap();

        //Confirm the size is valid
        if(size.0 == 0 || size.1 == 0){
            panic!("Size of window cannot be zero in either dimension!");
        }

        //Create config
        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface.get_supported_formats(&adapter)[0],
            width: size.0 as u32,
            height: size.1 as u32,
            present_mode: wgpu::PresentMode::Fifo
        };
        surface.configure(&device, &config);

        //create shader for render pipeline
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor{
            label: Some("Shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("rendering/shaders/shader.wgsl").into())
        });

        //create pipeline layout for render pipeline
        let render_pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor{
            label: Some("Render Pipeline Layout"),
            bind_group_layouts: &[],
            push_constant_ranges: &[]
        });

        //create render pipeline
        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor{
            label: Some("Render Pipeline"),
            layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState{
                module: &shader,
                entry_point: "vs_main",
                buffers: &[]
            },
            fragment: Some(wgpu::FragmentState{
                module: &shader,
                entry_point: "fs_main",
                targets: &[Some(wgpu::ColorTargetState{
                    format: config.format,
                    blend: Some(wgpu::BlendState::REPLACE),
                    write_mask: wgpu::ColorWrites::ALL,
                })]
            }),
            primitive: wgpu::PrimitiveState{
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: Some(wgpu::Face::Back),
                polygon_mode: wgpu::PolygonMode::Fill,
                unclipped_depth: false,
                conservative: false
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState{
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false
            },
            multiview: None
        });

        WGPUState{
            surface,
            device,
            queue,
            config,
            size,
            render_pipeline
        }
    }

    ///Called when the window resizes to update the surface to match the new window size.
    pub fn resize(&mut self, new_size: (i32, i32)) -> Result<(), &str> { //glfw::WindowEvent::Size has i32, i32
        if new_size.0 > 0 && new_size.1 > 0 {
            self.size = new_size;
            self.config.width = new_size.0 as u32;
            self.config.height = new_size.1 as u32;
            self.surface.configure(&self.device, &self.config);
            Ok(())
        }else{
            Err("Failed to resize surface because the size cannot be zero in any dimension!")
        }
    }

    /* 
    fn input(&mut self, event: &WindowEvent) -> bool {
        todo!()
    }

    fn update(&mut self){
        todo!()
    }

    fn render(&mut self){
        todo!()
    }
    */
}

#[cfg(test)]
mod test{
    use super::*;
    #[test]
    fn error_on_resize_with_dimension_zero(){
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

        glfw::Context::make_current(&mut window);


    let mut wgpu_state = pollster::block_on(WGPUState::new(&window));

    let result = wgpu_state.resize((0, 20));

    match result{
        Ok(..) => {panic!("Should have returned error!")},
        Err(..) => {}
    }

    }
}