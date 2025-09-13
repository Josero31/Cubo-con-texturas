#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct SimpleVertex {
    pub position: [f32; 3],
}

impl SimpleVertex {
    pub fn desc<'a>() -> wgpu::VertexBufferLayout<'a> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<SimpleVertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float32x3,
                },
            ],
        }
    }
}

// Triángulo simple en el centro de la pantalla
pub const SIMPLE_VERTICES: &[SimpleVertex] = &[
    SimpleVertex { position: [ 0.0,  0.5, 0.0] }, // Arriba
    SimpleVertex { position: [-0.5, -0.5, 0.0] }, // Abajo izquierda
    SimpleVertex { position: [ 0.5, -0.5, 0.0] }, // Abajo derecha
];

pub const SIMPLE_INDICES: &[u16] = &[
    0, 1, 2,  // Un solo triángulo
];