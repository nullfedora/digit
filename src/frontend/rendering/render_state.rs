use crate::frontend::rendering::mesh::{Vertex, VERTICES};


pub struct RenderState{
    pub vertices: Vec<Vertex>,
    pub indices: Vec<u16>,
    pub num_indices: u32
}

///Stores vertex and index information which can later be sent to the gpu using [[WGPUState]]
impl RenderState{
    pub fn new() -> RenderState{
        RenderState {  
            vertices: Vec::new(),
            indices: Vec::new(),
            num_indices: 0
        }
    }

    pub fn add_mesh(&mut self, vertices: &[Vertex], indices: &[u16]){
        //indices are for the current mesh and we probably already have some so they need to be adjusted
        let index_offset = self.vertices.len() as u16;

        //add vertices
        for v in vertices{
            self.vertices.push(*v);
        }

        //add indices, offset by the calculated factor
        for i in indices{
            self.indices.push(*i + index_offset);
        }

        self.num_indices += indices.len() as u32;
    }

    ///Remove all vertices and indices from this RenderState.
    pub fn clear(&mut self){
        self.vertices.clear();
        self.indices.clear();
        self.num_indices = 0;
    }

    ///Pads the index buffer so that it is a multiple of 4, but does NOT increase the num_indices count.  
    ///May cause unexpected behaviour if called multiple times before clearing.
    pub fn pad_index_buffer(&mut self){
        while self.indices.len() % 4 != 0{
            self.indices.push(0);
        }
    }
}

#[cfg(test)]
mod test{
    use super::*;

    #[test]
    fn add_mesh_vertices(){
        let mut render_state = RenderState::new();

        render_state.add_mesh(
            &[
                Vertex { position: [0.5, -0.5, 0.0], color: [1.0, 0.0, 0.0]},
                Vertex { position: [0.0, 0.5, 0.0], color: [0.0, 1.0, 0.0]},
                Vertex { position: [-0.5, -0.5, 0.0], color: [0.0, 0.0, 1.0]}
            ],
            &[
                0, 1, 2,
            ] 
        );

        assert!(Vertex::bitwise_equal(render_state.vertices[0], Vertex { position: [0.5, -0.5, 0.0], color: [1.0, 0.0, 0.0]}));
        assert!(Vertex::bitwise_equal(render_state.vertices[1], Vertex { position: [0.0, 0.5, 0.0], color: [0.0, 1.0, 0.0]}));
        assert!(Vertex::bitwise_equal(render_state.vertices[2], Vertex { position: [-0.5, -0.5, 0.0], color: [0.0, 0.0, 1.0]}));
    }

    #[test]
    fn add_mesh_indices(){
        let mut render_state = RenderState::new();

        render_state.add_mesh(
            &[
                Vertex { position: [0.5, -0.5, 0.0], color: [1.0, 0.0, 0.0]},
                Vertex { position: [0.0, 0.5, 0.0], color: [0.0, 1.0, 0.0]},
                Vertex { position: [-0.5, -0.5, 0.0], color: [0.0, 0.0, 1.0]}
            ],
            &[
                0, 1, 2,
            ] 
        );

        assert_eq!(render_state.indices, vec![0, 1, 2]);
    }

    #[test]
    fn add_mesh_num_vertices(){
        let mut render_state = RenderState::new();

        render_state.add_mesh(
            &[
                Vertex { position: [0.5, -0.5, 0.0], color: [1.0, 0.0, 0.0]},
                Vertex { position: [0.0, 0.5, 0.0], color: [0.0, 1.0, 0.0]},
                Vertex { position: [-0.5, -0.5, 0.0], color: [0.0, 0.0, 1.0]}
            ],
            &[
                0, 1, 2,
            ] 
        );

        assert_eq!(render_state.num_indices, 3);
    }

    #[test]
    fn clear_mesh(){
        let mut render_state = RenderState::new();

        render_state.add_mesh(
            &[
                Vertex { position: [0.5, -0.5, 0.0], color: [1.0, 0.0, 0.0]},
                Vertex { position: [0.0, 0.5, 0.0], color: [0.0, 1.0, 0.0]},
                Vertex { position: [-0.5, -0.5, 0.0], color: [0.0, 0.0, 1.0]}
            ],
            &[
                0, 1, 2,
            ] 
        );

        render_state.clear();

        assert_eq!(render_state.num_indices, 0);
        assert_eq!(render_state.vertices.len(), 0);
    }

    #[test]
    fn pad_indices(){
        let mut render_state = RenderState::new();

        render_state.add_mesh(
            &[
                Vertex { position: [0.5, -0.5, 0.0], color: [1.0, 0.0, 0.0]},
                Vertex { position: [0.0, 0.5, 0.0], color: [0.0, 1.0, 0.0]},
                Vertex { position: [-0.5, -0.5, 0.0], color: [0.0, 0.0, 1.0]}
            ],
            &[
                0, 1, 2,
            ] 
        );

        render_state.pad_index_buffer();

        assert_eq!(render_state.indices.len(), 4);

    }

}