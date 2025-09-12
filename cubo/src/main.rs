use cgmath::prelude::*;
use wgpu::util::DeviceExt;
use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window::{Window, WindowBuilder},
};
use image::{ImageBuffer, Rgba};

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
struct Vertex {
    position: [f32; 3],
    tex_coords: [f32; 2],
}

impl Vertex {
    fn desc<'a>() -> wgpu::VertexBufferLayout<'a> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float32x3,
                },
                wgpu::VertexAttribute {
                    offset: std::mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float32x2,
                },
            ],
        }
    }
}

// Vértices del cubo con coordenadas de textura para atlas de texturas
const VERTICES: &[Vertex] = &[
    // Front face (Cara frontal) - Primera sección del atlas
    Vertex { position: [-1.0, -1.0,  1.0], tex_coords: [0.0, 1.0] },
    Vertex { position: [ 1.0, -1.0,  1.0], tex_coords: [0.25, 1.0] },
    Vertex { position: [ 1.0,  1.0,  1.0], tex_coords: [0.25, 0.5] },
    Vertex { position: [-1.0,  1.0,  1.0], tex_coords: [0.0, 0.5] },
    
    // Back face (Cara trasera) - Segunda sección del atlas
    Vertex { position: [-1.0, -1.0, -1.0], tex_coords: [0.25, 1.0] },
    Vertex { position: [-1.0,  1.0, -1.0], tex_coords: [0.25, 0.5] },
    Vertex { position: [ 1.0,  1.0, -1.0], tex_coords: [0.5, 0.5] },
    Vertex { position: [ 1.0, -1.0, -1.0], tex_coords: [0.5, 1.0] },
    
    // Top face (Cara superior) - Tercera sección del atlas
    Vertex { position: [-1.0,  1.0, -1.0], tex_coords: [0.5, 1.0] },
    Vertex { position: [-1.0,  1.0,  1.0], tex_coords: [0.5, 0.5] },
    Vertex { position: [ 1.0,  1.0,  1.0], tex_coords: [0.75, 0.5] },
    Vertex { position: [ 1.0,  1.0, -1.0], tex_coords: [0.75, 1.0] },
    
    // Bottom face (Cara inferior) - Cuarta sección del atlas
    Vertex { position: [-1.0, -1.0, -1.0], tex_coords: [0.75, 1.0] },
    Vertex { position: [ 1.0, -1.0, -1.0], tex_coords: [1.0, 1.0] },
    Vertex { position: [ 1.0, -1.0,  1.0], tex_coords: [1.0, 0.5] },
    Vertex { position: [-1.0, -1.0,  1.0], tex_coords: [0.75, 0.5] },
    
    // Right face (Cara derecha) - Primera sección inferior
    Vertex { position: [ 1.0, -1.0, -1.0], tex_coords: [0.0, 0.5] },
    Vertex { position: [ 1.0,  1.0, -1.0], tex_coords: [0.0, 0.0] },
    Vertex { position: [ 1.0,  1.0,  1.0], tex_coords: [0.25, 0.0] },
    Vertex { position: [ 1.0, -1.0,  1.0], tex_coords: [0.25, 0.5] },
    
    // Left face (Cara izquierda) - Segunda sección inferior
    Vertex { position: [-1.0, -1.0, -1.0], tex_coords: [0.25, 0.5] },
    Vertex { position: [-1.0, -1.0,  1.0], tex_coords: [0.5, 0.5] },
    Vertex { position: [-1.0,  1.0,  1.0], tex_coords: [0.5, 0.0] },
    Vertex { position: [-1.0,  1.0, -1.0], tex_coords: [0.25, 0.0] },
];

const INDICES: &[u16] = &[
    0, 1, 2,  2, 3, 0,    // Front face
    4, 5, 6,  6, 7, 4,    // Back face
    8, 9, 10, 10, 11, 8,  // Top face
    12, 13, 14, 14, 15, 12, // Bottom face
    16, 17, 18, 18, 19, 16, // Right face
    20, 21, 22, 22, 23, 20, // Left face
];

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
struct Uniforms {
    view_proj: [[f32; 4]; 4],
}

impl Uniforms {
    fn new() -> Self {
        use cgmath::SquareMatrix;
        Self {
            view_proj: cgmath::Matrix4::identity().into(),
        }
    }

    fn update_view_proj(&mut self, view: cgmath::Matrix4<f32>, proj: cgmath::Matrix4<f32>) {
        self.view_proj = (proj * view).into();
    }
}

struct Camera {
    eye: cgmath::Point3<f32>,
    target: cgmath::Point3<f32>,
    up: cgmath::Vector3<f32>,
    aspect: f32,
    fovy: f32,
    znear: f32,
    zfar: f32,
}

impl Camera {
    fn build_view_projection_matrix(&self) -> cgmath::Matrix4<f32> {
        let view = cgmath::Matrix4::look_at_rh(self.eye, self.target, self.up);
        let proj = cgmath::perspective(cgmath::Deg(self.fovy), self.aspect, self.znear, self.zfar);
        proj * view
    }
}

struct State {
    surface: wgpu::Surface,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    size: winit::dpi::PhysicalSize<u32>,
    render_pipeline: wgpu::RenderPipeline,
    vertex_buffer: wgpu::Buffer,
    index_buffer: wgpu::Buffer,
    num_indices: u32,
    diffuse_bind_group: wgpu::BindGroup,
    camera: Camera,
    uniforms: Uniforms,
    uniform_buffer: wgpu::Buffer,
    uniform_bind_group: wgpu::BindGroup,
    window: Window,
}

impl State {
    async fn new(window: Window) -> State {
        let size = window.inner_size();

        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::all(),
            dx12_shader_compiler: Default::default(),
        });

        let surface = unsafe { instance.create_surface(&window) }.unwrap();

        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await
            .unwrap();

        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    features: wgpu::Features::empty(),
                    limits: wgpu::Limits::default(),
                    label: None,
                },
                None,
            )
            .await
            .unwrap();

        let surface_caps = surface.get_capabilities(&adapter);
        let surface_format = surface_caps
            .formats
            .iter()
            .copied()
            .find(|f| f.is_srgb())
            .unwrap_or(surface_caps.formats[0]);

        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.width,
            height: size.height,
            present_mode: surface_caps.present_modes[0],
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![],
        };
        surface.configure(&device, &config);

        // Crear textura (intentar cargar desde archivo o generar una por defecto)
        let diffuse_texture = create_or_load_texture(&device, &queue).await;

        let texture_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                entries: &[
                    wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Texture {
                            multisampled: false,
                            view_dimension: wgpu::TextureViewDimension::D2,
                            sample_type: wgpu::TextureSampleType::Float { filterable: true },
                        },
                        count: None,
                    },
                    wgpu::BindGroupLayoutEntry {
                        binding: 1,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                        count: None,
                    },
                ],
                label: Some("texture_bind_group_layout"),
            });

        let diffuse_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &texture_bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&diffuse_texture.view),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(&diffuse_texture.sampler),
                },
            ],
            label: Some("diffuse_bind_group"),
        });

        // Cámara
        let camera = Camera {
            eye: (0.0, 1.0, 2.0).into(),
            target: (0.0, 0.0, 0.0).into(),
            up: cgmath::Vector3::unit_y(),
            aspect: config.width as f32 / config.height as f32,
            fovy: 45.0,
            znear: 0.1,
            zfar: 100.0,
        };

        let mut uniforms = Uniforms::new();
        uniforms.update_view_proj(
            cgmath::Matrix4::look_at_rh(camera.eye, camera.target, camera.up),
            cgmath::perspective(cgmath::Deg(camera.fovy), camera.aspect, camera.znear, camera.zfar),
        );

        let uniform_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Uniform Buffer"),
            contents: bytemuck::cast_slice(&[uniforms]),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        let uniform_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                entries: &[wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                }],
                label: Some("uniform_bind_group_layout"),
            });

        let uniform_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &uniform_bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: uniform_buffer.as_entire_binding(),
            }],
            label: Some("uniform_bind_group"),
        });

        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("shader.wgsl").into()),
        });

        let render_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Render Pipeline Layout"),
                bind_group_layouts: &[&texture_bind_group_layout, &uniform_bind_group_layout],
                push_constant_ranges: &[],
            });

        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_main",
                buffers: &[Vertex::desc()],
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: "fs_main",
                targets: &[Some(wgpu::ColorTargetState {
                    format: config.format,
                    blend: Some(wgpu::BlendState::REPLACE),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: Some(wgpu::Face::Back),
                polygon_mode: wgpu::PolygonMode::Fill,
                unclipped_depth: false,
                conservative: false,
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            multiview: None,
        });

        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Vertex Buffer"),
            contents: bytemuck::cast_slice(VERTICES),
            usage: wgpu::BufferUsages::VERTEX,
        });

        let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Index Buffer"),
            contents: bytemuck::cast_slice(INDICES),
            usage: wgpu::BufferUsages::INDEX,
        });

        let num_indices = INDICES.len() as u32;

        Self {
            window,
            surface,
            device,
            queue,
            config,
            size,
            render_pipeline,
            vertex_buffer,
            index_buffer,
            num_indices,
            diffuse_bind_group,
            camera,
            uniforms,
            uniform_buffer,
            uniform_bind_group,
        }
    }

    pub fn window(&self) -> &Window {
        &self.window
    }

    fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.size = new_size;
            self.config.width = new_size.width;
            self.config.height = new_size.height;
            self.surface.configure(&self.device, &self.config);
            
            self.camera.aspect = self.config.width as f32 / self.config.height as f32;
        }
    }

    fn update(&mut self) {
        // Rotación del cubo
        let time = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs_f32();
        
        let rotation_y = cgmath::Matrix4::from_angle_y(cgmath::Rad(time * 0.5));
        let rotation_x = cgmath::Matrix4::from_angle_x(cgmath::Rad(time * 0.3));
        let rotation = rotation_y * rotation_x;
        
        let view = cgmath::Matrix4::look_at_rh(self.camera.eye, self.camera.target, self.camera.up);
        let proj = cgmath::perspective(
            cgmath::Deg(self.camera.fovy), 
            self.camera.aspect, 
            self.camera.znear, 
            self.camera.zfar
        );
        
        self.uniforms.view_proj = (proj * view * rotation).into();
        self.queue.write_buffer(&self.uniform_buffer, 0, bytemuck::cast_slice(&[self.uniforms]));
    }

    fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        let output = self.surface.get_current_texture()?;
        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });

        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.1,
                            g: 0.2,
                            b: 0.3,
                            a: 1.0,
                        }),
                        store: true,
                    },
                })],
                depth_stencil_attachment: None,
            });

            render_pass.set_pipeline(&self.render_pipeline);
            render_pass.set_bind_group(0, &self.diffuse_bind_group, &[]);
            render_pass.set_bind_group(1, &self.uniform_bind_group, &[]);
            render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
            render_pass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
            render_pass.draw_indexed(0..self.num_indices, 0, 0..1);
        }

        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();

        Ok(())
    }
}

struct Texture {
    texture: wgpu::Texture,
    view: wgpu::TextureView,
    sampler: wgpu::Sampler,
}

// Función para generar textura atlas con diferentes colores
fn create_texture_atlas() -> Vec<u8> {
    let width = 256;
    let height = 128;
    let section_width = width / 4; // 4 secciones horizontales
    let section_height = height / 2; // 2 secciones verticales
    
    let img = ImageBuffer::from_fn(width, height, |x, y| {
        let section_x = x / section_width;
        let section_y = y / section_height;
        
        // Determinar el color basado en la sección
        match (section_x, section_y) {
            (0, 0) => Rgba([255, 100, 100, 255]), // Rojo - Front face
            (1, 0) => Rgba([100, 255, 100, 255]), // Verde - Back face
            (2, 0) => Rgba([100, 100, 255, 255]), // Azul - Top face
            (3, 0) => Rgba([255, 255, 100, 255]), // Amarillo - Bottom face
            (0, 1) => Rgba([255, 100, 255, 255]), // Magenta - Right face
            (1, 1) => Rgba([100, 255, 255, 255]), // Cian - Left face
            _ => Rgba([255, 255, 255, 255]),       // Blanco por defecto
        }
    });
    
    let mut bytes = Vec::new();
    use std::io::Cursor;
    img.write_to(&mut Cursor::new(&mut bytes), image::ImageOutputFormat::Png)
        .expect("Error al escribir imagen");
    bytes
}

// Función para crear textura desde bytes
fn load_texture_from_bytes(
    device: &wgpu::Device,
    queue: &wgpu::Queue,
    bytes: &[u8],
    label: &str,
) -> Result<Texture, Box<dyn std::error::Error>> {
    let img = image::load_from_memory(bytes)?;
    let rgba = img.to_rgba8();
    let dimensions = img.dimensions();

    let texture_size = wgpu::Extent3d {
        width: dimensions.0,
        height: dimensions.1,
        depth_or_array_layers: 1,
    };

    let texture = device.create_texture(&wgpu::TextureDescriptor {
        label: Some(label),
        size: texture_size,
        mip_level_count: 1,
        sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        format: wgpu::TextureFormat::Rgba8UnormSrgb,
        usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
        view_formats: &[],
    });

    queue.write_texture(
        wgpu::ImageCopyTexture {
            aspect: wgpu::TextureAspect::All,
            texture: &texture,
            mip_level: 0,
            origin: wgpu::Origin3d::ZERO,
        },
        &rgba,
        wgpu::ImageDataLayout {
            offset: 0,
            bytes_per_row: Some(4 * dimensions.0),
            rows_per_image: Some(dimensions.1),
        },
        texture_size,
    );

    let view = texture.create_view(&wgpu::TextureViewDescriptor::default());
    let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
        address_mode_u: wgpu::AddressMode::ClampToEdge,
        address_mode_v: wgpu::AddressMode::ClampToEdge,
        address_mode_w: wgpu::AddressMode::ClampToEdge,
        mag_filter: wgpu::FilterMode::Linear,
        min_filter: wgpu::FilterMode::Nearest,
        mipmap_filter: wgpu::FilterMode::Nearest,
        ..Default::default()
    });

    Ok(Texture {
        texture,
        view,
        sampler,
    })
}

// Función principal para crear o cargar textura
async fn create_or_load_texture(
    device: &wgpu::Device,
    queue: &wgpu::Queue,
) -> Texture {
    // Intentar cargar textura desde archivo
    let texture_bytes = match std::fs::read("assets/texture.png") {
        Ok(bytes) => {
            println!("✅ Textura cargada desde assets/texture.png");
            bytes
        },
        Err(_) => {
            println!("⚠️  No se encontró assets/texture.png, generando textura por defecto...");
            // Crear directorio assets si no existe
            std::fs::create_dir_all("assets").unwrap_or_else(|_| {});
            
            // Generar textura y guardarla
            let generated_bytes = create_texture_atlas();
            std::fs::write("assets/texture.png", &generated_bytes)
                .unwrap_or_else(|_| println!("No se pudo guardar la textura generada"));
            
            println!("✅ Textura atlas generada y guardada en assets/texture.png");
            generated_bytes
        }
    };

    load_texture_from_bytes(device, queue, &texture_bytes, "texture.png")
        .expect("Error al cargar textura")
}

fn main() {
    env_logger::init();
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("Cubo con Textura Atlas - Josero31")
        .with_inner_size(winit::dpi::LogicalSize::new(800, 600))
        .build(&event_loop)
        .unwrap();

    let mut state = pollster::block_on(State::new(window));

    event_loop.run(move |event, _, control_flow| {
        match event {
            Event::WindowEvent {
                ref event,
                window_id,
            } if window_id == state.window().id() => match event {
                WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                WindowEvent::Resized(physical_size) => {
                    state.resize(*physical_size);
                }
                WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                    state.resize(**new_inner_size);
                }
                WindowEvent::KeyboardInput {
                    input: KeyboardInput {
                        state: ElementState::Pressed,
                        virtual_keycode: Some(VirtualKeyCode::Escape),
                        ..
                    },
                    ..
                } => *control_flow = ControlFlow::Exit,
                _ => {}
            },
            Event::RedrawRequested(window_id) if window_id == state.window().id() => {
                state.update();
                match state.render() {
                    Ok(_) => {}
                    Err(wgpu::SurfaceError::Lost) => state.resize(state.size),
                    Err(wgpu::SurfaceError::OutOfMemory) => *control_flow = ControlFlow::Exit,
                    Err(e) => eprintln!("{:?}", e),
                }
            }
            Event::MainEventsCleared => {
                state.window().request_redraw();
            }
            _ => {}
        }
    });
}