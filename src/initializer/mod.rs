use imgui::*;
use imgui_winit_support::{HiDpiMode, WinitPlatform};
use glium::{Display, Surface};
use glium::glutin::{self, Event, WindowEvent};
use imgui_glium_renderer::Renderer;
use std::time::Instant;

pub struct System {
    pub events_loop: glutin::EventsLoop,
    pub display: glium::Display,
    pub imgui: Context,
    pub platform: WinitPlatform,
    pub renderer: Renderer,
    pub font_size: f32,
}

impl System {
    pub fn main_loop<F: FnMut(&mut bool, &mut Ui)>(self, mut run_ui: F) {
        let System {
            mut events_loop,
            display,
            mut imgui,
            mut platform,
            mut renderer,
            ..
        } = self;
        let gl_window = display.gl_window();
        let window = gl_window.window();
        let mut last_frame = Instant::now();
        let mut run = true;

        while run {
            events_loop.poll_events(|event| {
                platform.handle_event(imgui.io_mut(), &window, &event);

                if let Event::WindowEvent { event, .. } = event {
                    if let WindowEvent::CloseRequested = event {
                        run = false;
                    }
                }
            });

            let io = imgui.io_mut();
            platform
                .prepare_frame(io, &window)
                .expect("Failed to start frame");
            last_frame = io.update_delta_time(last_frame);
            let mut ui = imgui.frame();
            run_ui(&mut run, &mut ui);

            let mut target = display.draw();
            target.clear_color_srgb(1.0, 1.0, 1.0, 1.0);
            platform.prepare_render(&ui, &window);
            let draw_data = ui.render();
            renderer
                .render(&mut target, draw_data)
                .expect("Rendering failed");
            target.finish().expect("Failed to swap buffers");
        }
    }
}

pub fn init(title: &str, win_with: f64, win_height: f64) -> System {
    let title = match title.rfind('/') {
        Some(idx) => title.split_at(idx + 1).1,
        None => title,
    };
    let events_loop = glutin::EventsLoop::new();
    let context = glutin::ContextBuilder::new().with_vsync(true);
    let builder = glutin::WindowBuilder::new()
        .with_title(title.to_owned())
        .with_dimensions(glutin::dpi::LogicalSize::new(win_with, win_height));
    let display =
        Display::new(builder, context, &events_loop).expect("Failed to initialize display");

    let mut imgui = Context::create();
    imgui.set_ini_filename(None);

    let mut platform = WinitPlatform::init(&mut imgui);
    {
        let gl_window = display.gl_window();
        let window = gl_window.window();
        platform.attach_window(imgui.io_mut(), &window, HiDpiMode::Rounded);
    }

    let hidpi_factor = platform.hidpi_factor();
    let font_size = (13.0 * hidpi_factor) as f32;
    imgui.fonts().add_font(&[
        FontSource::DefaultFontData {
            config: Some(FontConfig {
                size_pixels: font_size,
                ..FontConfig::default()
            }),
        },
    ]);

    imgui.io_mut().font_global_scale = (1.0 / hidpi_factor) as f32;

    let renderer = Renderer::init(&mut imgui, &display).expect("Failed to initialize renderer");

    System {
        events_loop,
        display,
        imgui,
        platform,
        renderer,
        font_size,
    }
}