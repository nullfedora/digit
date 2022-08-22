

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    pub position: [f32; 3],
    pub color: [f32; 3]
}

pub const VERTICES: &[Vertex] = &[
    Vertex { position: [0.5, -0.5, 0.0], color: [1.0, 0.0, 0.0]},
    Vertex { position: [0.0, 0.5, 0.0], color: [0.0, 1.0, 0.0]},
    Vertex { position: [-0.5, -0.5, 0.0], color: [0.0, 0.0, 1.0]}
];

pub const INDICES: &[u16] = &[
    0, 1, 2,
];


impl Vertex{
    pub fn desc<'a>() -> wgpu::VertexBufferLayout<'a> {
        wgpu::VertexBufferLayout{
            array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float32x3
                },
                wgpu::VertexAttribute{
                    offset: std::mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float32x3
                }
            ]
        }
    }

    ///Check if two vertices are bitwise equal to eachother.
    pub fn bitwise_equal(a: Vertex, b: Vertex) -> bool{
        for i in 0..3 {
            if a.color[i] != b.color[i]{
                return false
            }
            if a.position[i] != b.position[i]{
                return false
            }
        };
        true
    }
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn bitwise_equal(){
        let a: Vertex = Vertex { position: [0.5, -0.5, 0.0], color: [1.0, 0.0, 0.5]};
        let b: Vertex = Vertex { position: [0.5, -0.5, 0.0], color: [1.0, 0.0, 0.5]};

        assert!(Vertex::bitwise_equal(a, b));
    }

    #[test]
    fn bitwise_not_equal(){
        let a: Vertex = Vertex { position: [0.5, -0.5, 0.0], color: [1.0, 0.0, 0.5]};
        let b: Vertex = Vertex { position: [0.0, -0.5, 0.0], color: [0.0, 1.0, 0.5]};

        assert!(!Vertex::bitwise_equal(a, b));
    }
}