// use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, HtmlImageElement};
// use yew::{function_component, html, use_effect_with_deps, use_node_ref, use_state, Html};
use yew::*;
//
// use winit::{
//     event::*,
//     event_loop::{ControlFlow, EventLoop},
//     platform::web::WindowBuilderExtWebSys,
//     window::{Window, WindowBuilder},
// };
//
// fn compute_work_group_count((image_dims): (u32, u32), (workgroup_dims): (u32, u32)) -> (u32, u32) {
//     let x = (image_dims.0 + workgroup_dims.0 - 1) / workgroup_dims.0;
//     let y = (image_dims.1 + workgroup_dims.1 - 1) / workgroup_dims.1;
//
//     (x, y)
// }
//
// fn padded_bytes_per_row(width: u32) -> usize {
//     let bytes_per_row = width as usize * 4;
//     let padding = (256 - bytes_per_row % 256) % 256;
//     bytes_per_row + padding
// }
//
// struct State {
//     surface: wgpu::Surface,
//     device: wgpu::Device,
//     queue: wgpu::Queue,
//     config: wgpu::SurfaceConfiguration,
//     size: winit::dpi::PhysicalSize<u32>,
// }
//
// impl State {
//     async fn new(window: &Window, image: HtmlImageElement, canvas: HtmlCanvasElement) -> Self {
//         let size = window.inner_size();
//
//         let instance = wgpu::Instance::new(wgpu::Backends::all());
//         let surface = unsafe { instance.create_surface(window) };
//
//         let options = wgpu::RequestAdapterOptions {
//             power_preference: wgpu::PowerPreference::default(),
//             compatible_surface: Some(&surface),
//             force_fallback_adapter: false,
//         };
//         let adapter = instance.request_adapter(&options).await.unwrap();
//
//         let descriptor = wgpu::DeviceDescriptor {
//             features: wgpu::Features::empty(),
//             limits: wgpu::Limits::downlevel_webgl2_defaults(),
//             label: None,
//         };
//         let (device, queue) = adapter.request_device(&descriptor, None).await.unwrap();
//
//         let context = canvas
//             .get_context("2d")
//             .unwrap()
//             .unwrap()
//             .dyn_into::<CanvasRenderingContext2d>()
//             .unwrap();
//
//         let (width, height) = (image.width(), image.height());
//         context
//             .draw_image_with_html_image_element_and_dw_and_dh(
//                 &image,
//                 0.0,
//                 0.0,
//                 width as f64,
//                 height as f64,
//             )
//             .unwrap();
//
//         let diffuse_rgba =
//             context.get_image_data(0.0, 0.0, width as f64, height as f64).unwrap().data();
//         let texture_size = wgpu::Extent3d { width, height, depth_or_array_layers: 1 };
//
//         let diffuse_texture = device.create_texture(&wgpu::TextureDescriptor {
//             size: texture_size,
//             mip_level_count: 1,
//             sample_count: 1,
//             dimension: wgpu::TextureDimension::D2,
//             format: wgpu::TextureFormat::Rgba8UnormSrgb,
//             usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
//             label: Some("diffuse_texture"),
//         });
//
//         queue.write_texture(
//             wgpu::ImageCopyTexture {
//                 texture: &diffuse_texture,
//                 mip_level: 0,
//                 origin: wgpu::Origin3d::ZERO,
//                 aspect: wgpu::TextureAspect::All,
//             },
//             &diffuse_rgba,
//             wgpu::ImageDataLayout {
//                 offset: 0,
//                 bytes_per_row: std::num::NonZeroU32::new(4 * width),
//                 rows_per_image: std::num::NonZeroU32::new(height),
//             },
//             texture_size,
//         );
//
//         let diffuse_texture_view =
//             diffuse_texture.create_view(&wgpu::TextureViewDescriptor::default());
//
//         // let shader = device.create_shader_module(&wgpu::ShaderModuleDescriptor {
//         //     label: Some("Grayscale shader"),
//         //     source: wgpu::ShaderSource::Wgsl(include_str!("shaders/grayscale.wgsl").into()),
//         // });
//
//         let pipeline = device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
//             label: Some("Grayscale pipeline"),
//             layout: None,
//             module: &shader,
//             entry_point: "grayscale_main",
//         });
//
//         let entry1 = wgpu::BindGroupEntry {
//             binding: 0,
//             resource: wgpu::BindingResource::TextureView(
//                 &input_texture.create_view(&wgpu::TextureViewDescriptor::default()),
//             ),
//         };
//         let entry2 = wgpu::BindGroupEntry {
//             binding: 1,
//             resource: wgpu::BindingResource::TextureView(
//                 &output_texture.create_view(&wgpu::TextureViewDescriptor::default()),
//             ),
//         };
//
//         let texture_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
//             label: Some("Texture bind group"),
//             layout: &pipeline.get_bind_group_layout(0),
//             entries: &[entry1, entry2],
//         });
//
//         let mut encoder =
//             device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
//
//         {
//             let (dispatch_with, dispatch_height) =
//                 compute_work_group_count((texture_size.width, texture_size.height), (16, 16));
//             let mut compute_pass = encoder
//                 .begin_compute_pass(&wgpu::ComputePassDescriptor { label: Some("Grayscale pass") });
//
//             compute_pass.set_pipeline(&pipeline);
//             compute_pass.set_bind_group(0, &texture_bind_group, &[]);
//             compute_pass.dispatch(dispatch_with, dispatch_height, 1);
//         }
//
//         let padded_bytes_per_row = padded_bytes_per_row(width);
//         let unpadded_bytes_per_row = width as usize * 4;
//
//         let output_buffer_size =
//             padded_bytes_per_row as u64 * height as u64 * std::mem::size_of::<u8>() as u64;
//         let output_buffer = device.create_buffer(&wgpu::BufferDescriptor {
//             label: None,
//             size: output_buffer_size,
//             usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ,
//             mapped_at_creation: false,
//         });
//
//         encoder.copy_texture_to_buffer(
//             wgpu::ImageCopyTexture {
//                 aspect: wgpu::TextureAspect::All,
//                 texture: &output_texture,
//                 mip_level: 0,
//                 origin: wgpu::Origin3d::ZERO,
//             },
//             wgpu::ImageCopyBuffer {
//                 buffer: &output_buffer,
//                 layout: wgpu::ImageDataLayout {
//                     offset: 0,
//                     bytes_per_row: std::num::NonZeroU32::new(padded_bytes_per_row as u32),
//                     rows_per_image: std::num::NonZeroU32::new(height),
//                 },
//             },
//             texture_size,
//         );
//
//         queue.submit(Some(encoder.finish()));
//         let buffer_slice = output_buffer.slice(..);
//         buffer_slice.map_async(wgpu::MapMode::Read, || log::info!("done"));
//
//         device.poll(wgpu::Maintain::Wait);
//
//         let config = wgpu::SurfaceConfiguration {
//             usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
//             format: surface.get_supported_formats(&adapter)[0],
//             width: size.width,
//             height: size.height,
//             present_mode: wgpu::PresentMode::Fifo,
//         };
//         surface.configure(&device, &config);
//         Self { surface, device, queue, config, size }
//     }
//
//     fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
//         if new_size.width > 0 && new_size.height > 0 {
//             self.size = new_size;
//             self.config.width = new_size.width;
//             self.config.height = new_size.height;
//             self.surface.configure(&self.device, &self.config);
//         }
//     }
//
//     fn input(&mut self, event: &WindowEvent) -> bool {
//         false
//     }
//
//     fn update(&mut self) {}
//
//     fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
//         let output = self.surface.get_current_texture()?;
//         let view = output.texture.create_view(&wgpu::TextureViewDescriptor::default());
//
//         let mut encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
//             label: Some("Render Encoder"),
//         });
//
//         let color = wgpu::Color { r: 0.1, g: 0.2, b: 0.3, a: 1.0 };
//         let attachment = wgpu::RenderPassColorAttachment {
//             view: &view,
//             resolve_target: None,
//             ops: wgpu::Operations { load: wgpu::LoadOp::Clear(color), store: true },
//         };
//
//         {
//             let _render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
//                 label: Some("Render Pass"),
//                 color_attachments: &[Some(attachment)],
//                 depth_stencil_attachment: None,
//             });
//         }
//
//         // submit will accept anything that implements IntoIter
//         self.queue.submit(std::iter::once(encoder.finish()));
//         output.present();
//
//         Ok(())
//     }
// }
//
// pub async fn run(
//     event_loop: EventLoop<()>,
//     window: Window,
//     image: HtmlImageElement,
//     canvas: HtmlCanvasElement,
// ) {
//     let mut state = State::new(&window, image, canvas).await;
//
//     event_loop.run(move |event, _, control_flow| {
//         match event {
//             Event::WindowEvent { ref event, window_id } if window_id == window.id() => {
//                 if !state.input(event) {
//                     match event {
//                         WindowEvent::CloseRequested
//                         | WindowEvent::KeyboardInput {
//                             input:
//                                 KeyboardInput {
//                                     state: ElementState::Pressed,
//                                     virtual_keycode: Some(VirtualKeyCode::Escape),
//                                     ..
//                                 },
//                             ..
//                         } => *control_flow = ControlFlow::Exit,
//                         WindowEvent::Resized(physical_size) => {
//                             state.resize(*physical_size);
//                         }
//                         WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
//                             state.resize(**new_inner_size);
//                         }
//                         _ => {}
//                     }
//                 }
//             }
//             Event::RedrawRequested(window_id) if window_id == window.id() => {
//                 state.update();
//                 match state.render() {
//                     Ok(_) => {}
//                     // Reconfigure the surface if lost
//                     Err(wgpu::SurfaceError::Lost) => state.resize(state.size),
//                     // The system is out of memory, we should probably quit
//                     Err(wgpu::SurfaceError::OutOfMemory) => *control_flow = ControlFlow::Exit,
//                     // All other errors (Outdated, Timeout) should be resolved by the next frame
//                     Err(e) => eprintln!("{:?}", e),
//                 }
//             }
//             Event::MainEventsCleared => {
//                 // RedrawRequested will only trigger once, unless we manually
//                 // request it.
//                 window.request_redraw();
//             }
//             _ => {}
//         }
//     });
// }

#[function_component(Test)]
pub fn test() -> Html {
    // let surface = use_node_ref();
    // let image = use_node_ref();
    // let canvas = use_node_ref();
    //
    // let loaded = use_state(|| false);
    // let onload = ywt::callback!(loaded; move |_| loaded.set(true));
    //
    // let url = "https://images.unsplash.com/photo-1514888286974-6c03e2ca1dba?ixid=MnwzNDk1MTF8MHwxfHNlYXJjaHwxfHxjYXR8ZW58MHx8fHwxNjU4Nzk0MDMz&ixlib=rb-1.2.1";
    //
    // use_effect_with_deps(
    //     |(surface, image, canvas, loaded)| {
    //         log::error!("haaaa {}", loaded);
    //         if *loaded {
    //             let surface = surface.cast::<HtmlCanvasElement>();
    //             let canvas = canvas.cast::<HtmlCanvasElement>().unwrap();
    //             log::info!("{image:?}");
    //             let image = image.cast::<HtmlImageElement>().unwrap();
    //
    //             let event_loop = EventLoop::new();
    //
    //             let window = WindowBuilder::new()
    //                 .with_canvas(surface)
    //                 .with_title("A fantastic window!")
    //                 .build(&event_loop)
    //                 .unwrap();
    //
    //             ywt::spawn!(async move {
    //                 run(event_loop, window, image, canvas).await;
    //             });
    //         }
    //
    //         || ()
    //     },
    //     (surface.clone(), image.clone(), canvas.clone(), (*loaded).clone()),
    // );

    html! {
        // <>
        // <canvas ref={surface} />
        // <canvas ref={canvas} style="display:none" />
        // <img crossorigin="Anonymous" style="display:none" {onload} src={url} ref={image} />
        // </>
    }
}
