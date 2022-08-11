use crate::frontend::wgpu_state::WGPUState;
use super::render_state::RenderState;



///Render the things designated to be rendered in render_state to the surface in wgpu_state.
///adapted from tuturial https://sotrh.github.io/learn-wgpu/beginner/tutorial2-surface/#render
pub fn render(wgpu_state: &WGPUState, render_state: &RenderState){
    
    let output = wgpu_state.get_surface().get_current_texture().unwrap();

    let view = output.texture.create_view(&wgpu::TextureViewDescriptor::default());

    let mut encoder = wgpu_state.get_device().create_command_encoder(&wgpu::CommandEncoderDescriptor{
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

        render_pass.set_pipeline(&wgpu_state.get_render_pipeline());
        render_pass.set_vertex_buffer(0, wgpu_state.get_vertex_buffer().slice(..));
        render_pass.set_index_buffer(wgpu_state.get_index_buffer().slice(..), wgpu::IndexFormat::Uint16);
        render_pass.draw_indexed(0..wgpu_state.get_num_indices(), 0, 0..1);

    }

    wgpu_state.get_queue().submit(std::iter::once(encoder.finish()));
    output.present();
}