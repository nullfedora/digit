use wgpu;

///Handles the surface created with WGPU, and the device configuration.  
/// Also handles resizing the surface in case of a window resize.
pub struct WGPUState{
    surface: wgpu::Surface,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    size: (i32, i32)
}

impl WGPUState{
    //Create a new wgpustate from a glfw window.  Taken from https://sotrh.github.io/learn-wgpu/beginner/tutorial2-surface/#state-new
    pub async fn new(window: &glfw::Window) -> WGPUState{
        

        let size = window.get_size();

        let instance = wgpu::Instance::new(wgpu::Backends::all());
        let surface = unsafe { instance.create_surface(window) };
        let adapter = instance.request_adapter(
            &wgpu::RequestAdapterOptions{
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: Some(&surface),
                force_fallback_adapter: true //Tutorial uses false but that crashes for some reason
            }
        ).await.unwrap();

        let (device, queue) = adapter.request_device(
            &wgpu::DeviceDescriptor {
                features: wgpu::Features::empty(),
                limits: wgpu::Limits::default(),
                label: None
            },
            None
        ).await.unwrap();

        if(size.0 == 0 || size.1 == 0){
            panic!("Size of window cannot be zero in either dimension!");
        }

        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface.get_supported_formats(&adapter)[0],
            width: size.0 as u32,
            height: size.1 as u32,
            present_mode: wgpu::PresentMode::Fifo
        };
        surface.configure(&device, &config);

        WGPUState{
            surface,
            device,
            queue,
            config,
            size
        }
    }

    ///Called when the window resizes to update the surface to match the new window size.
    ///# panics
    /// Panics if the new dimensions are zero in either dimension.
    pub fn resize(&mut self, new_size: (i32, i32)) { //glfw::WindowEvent::Size has i32, i32
        if new_size.0 > 0 && new_size.1 > 0 {
            self.size = new_size;
            self.config.width = new_size.0 as u32;
            self.config.height = new_size.1 as u32;
            self.surface.configure(&self.device, &self.config)
        }else{
            panic!("Size of window cannot be zero in either dimension!");
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