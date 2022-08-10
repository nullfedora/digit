use crate::frontend::wgpu_state::WGPUState;
use super::render_state::RenderState;



///Render the things designated to be rendered in render_state to the surface in wgpu_state.
///adapted from tuturial https://sotrh.github.io/learn-wgpu/beginner/tutorial2-surface/#render
pub fn render(wgpu_state: &WGPUState, render_state: &RenderState){
    
    let output = wgpu_state.surface.get_current_texture().unwrap();

    let view = output.texture.create_view(&wgpu::TextureViewDescriptor::default());

    let mut encoder = wgpu_state.device.create_command_encoder(&wgpu::CommandEncoderDescriptor{
        label: Some("Render Encoder")
    });

    {
        let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("Render Pass"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment{
                view: &view,
                resolve_target: None,
                ops: wgpu::Operations{
                    load: wgpu::LoadOp::Clear(wgpu::Color{
                        r: 0.1,
                        g: 0.2,
                        b: 0.3,
                        a: 1.0
                    }),
                    store: true
                }
            })],
            depth_stencil_attachment: None
        });

        render_pass.set_pipeline(&wgpu_state.render_pipeline);
        render_pass.draw(0..3, 0..1);

    }

    wgpu_state.queue.submit(std::iter::once(encoder.finish()));
    output.present();
}