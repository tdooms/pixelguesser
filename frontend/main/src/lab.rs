use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, HtmlImageElement};
use yew::{function_component, html, use_effect_with_deps, use_node_ref, use_state, Html};

use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
    platform::web::WindowBuilderExtWebSys,
    window::{Window, WindowBuilder},
};

struct State {
    surface: wgpu::Surface,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    size: winit::dpi::PhysicalSize<u32>,
}

impl State {
    async fn new(window: &Window, image: HtmlImageElement, canvas: HtmlCanvasElement) -> Self {
        let size = window.inner_size();

        let instance = wgpu::Instance::new(wgpu::Backends::all());
        let surface = unsafe { instance.create_surface(window) };

        let options = wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::default(),
            compatible_surface: Some(&surface),
            force_fallback_adapter: false,
        };
        let adapter = instance.request_adapter(&options).await.unwrap();

        let descriptor = wgpu::DeviceDescriptor {
            features: wgpu::Features::empty(),
            limits: wgpu::Limits::downlevel_webgl2_defaults(),
            label: None,
        };
        let (device, queue) = adapter.request_device(&descriptor, None).await.unwrap();

        let context = canvas
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into::<CanvasRenderingContext2d>()
            .unwrap();

        let (width, height) = (image.width(), image.height());
        context
            .draw_image_with_html_image_element_and_dw_and_dh(
                &image,
                0.0,
                0.0,
                width as f64,
                height as f64,
            )
            .unwrap();

        log::info!("{}", canvas.to_data_url().unwrap());

        // context.get_image_data(0.0, 0.0, width as f64, height as f64).unwrap().data();
        // let texture_size = wgpu::Extent3d { width, height, depth_or_array_layers: 1 };
        //
        // let diffuse_texture = device.create_texture(&wgpu::TextureDescriptor {
        //     size: texture_size,
        //     mip_level_count: 1,
        //     sample_count: 1,
        //     dimension: wgpu::TextureDimension::D2,
        //     format: wgpu::TextureFormat::Rgba8UnormSrgb,
        //     usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
        //     label: Some("diffuse_texture"),
        // });

        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface.get_supported_formats(&adapter)[0],
            width: size.width,
            height: size.height,
            present_mode: wgpu::PresentMode::Fifo,
        };
        surface.configure(&device, &config);
        Self { surface, device, queue, config, size }
    }

    fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.size = new_size;
            self.config.width = new_size.width;
            self.config.height = new_size.height;
            self.surface.configure(&self.device, &self.config);
        }
    }

    fn input(&mut self, event: &WindowEvent) -> bool {
        false
    }

    fn update(&mut self) {}

    fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        let output = self.surface.get_current_texture()?;
        let view = output.texture.create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Render Encoder"),
        });

        let color = wgpu::Color { r: 0.1, g: 0.2, b: 0.3, a: 1.0 };
        let attachment = wgpu::RenderPassColorAttachment {
            view: &view,
            resolve_target: None,
            ops: wgpu::Operations { load: wgpu::LoadOp::Clear(color), store: true },
        };

        {
            let _render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(attachment)],
                depth_stencil_attachment: None,
            });
        }

        // submit will accept anything that implements IntoIter
        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();

        Ok(())
    }
}

pub async fn run(
    event_loop: EventLoop<()>,
    window: Window,
    image: HtmlImageElement,
    canvas: HtmlCanvasElement,
) {
    let mut state = State::new(&window, image, canvas).await;

    event_loop.run(move |event, _, control_flow| {
        match event {
            Event::WindowEvent { ref event, window_id } if window_id == window.id() => {
                if !state.input(event) {
                    match event {
                        WindowEvent::CloseRequested
                        | WindowEvent::KeyboardInput {
                            input:
                                KeyboardInput {
                                    state: ElementState::Pressed,
                                    virtual_keycode: Some(VirtualKeyCode::Escape),
                                    ..
                                },
                            ..
                        } => *control_flow = ControlFlow::Exit,
                        WindowEvent::Resized(physical_size) => {
                            state.resize(*physical_size);
                        }
                        WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                            state.resize(**new_inner_size);
                        }
                        _ => {}
                    }
                }
            }
            Event::RedrawRequested(window_id) if window_id == window.id() => {
                state.update();
                match state.render() {
                    Ok(_) => {}
                    // Reconfigure the surface if lost
                    Err(wgpu::SurfaceError::Lost) => state.resize(state.size),
                    // The system is out of memory, we should probably quit
                    Err(wgpu::SurfaceError::OutOfMemory) => *control_flow = ControlFlow::Exit,
                    // All other errors (Outdated, Timeout) should be resolved by the next frame
                    Err(e) => eprintln!("{:?}", e),
                }
            }
            Event::MainEventsCleared => {
                // RedrawRequested will only trigger once, unless we manually
                // request it.
                window.request_redraw();
            }
            _ => {}
        }
    });
}

#[function_component(Test)]
pub fn test() -> Html {
    let surface = use_node_ref();
    let image = use_node_ref();
    let canvas = use_node_ref();

    let loaded = use_state(|| false);
    let onload = ywt::callback!(loaded; move |_| loaded.set(true));

    let url = "https://images.unsplash.com/photo-1514888286974-6c03e2ca1dba?ixid=MnwzNDk1MTF8MHwxfHNlYXJjaHwxfHxjYXR8ZW58MHx8fHwxNjU4Nzk0MDMz&ixlib=rb-1.2.1";

    use_effect_with_deps(
        |(surface, image, canvas, loaded)| {
            log::error!("haaaa {}", loaded);
            if *loaded {
                let surface = surface.cast::<HtmlCanvasElement>();
                let canvas = canvas.cast::<HtmlCanvasElement>().unwrap();
                log::info!("{image:?}");
                let image = image.cast::<HtmlImageElement>().unwrap();

                let event_loop = EventLoop::new();

                let window = WindowBuilder::new()
                    .with_canvas(surface)
                    .with_title("A fantastic window!")
                    .build(&event_loop)
                    .unwrap();

                ywt::spawn!(async move {
                    run(event_loop, window, image, canvas).await;
                });
            }

            || ()
        },
        (surface.clone(), image.clone(), canvas.clone(), (*loaded).clone()),
    );

    html! {
        <>
        <canvas ref={surface} />
        <canvas ref={canvas} style="display:none" />
        <img crossorigin="Anonymous" style="display:none" {onload} src={url} ref={image} />
        </>
    }
}
