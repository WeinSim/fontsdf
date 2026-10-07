use std::process::ExitCode;

use glfw::{self, Action, Context, Key, OpenGlProfileHint, Window, WindowEvent, WindowHint};

mod rendering;
use crate::rendering::shaders::ShaderProgram;
use crate::rendering::vao::Vao;

const WIDTH: u32 = 1280;
const HEIGHT: u32 = 720;

const KEY_MAP: &[(Key, Key)] = &[
    (Key::CapsLock, Key::Escape),
    (Key::Y, Key::Z),
    (Key::RightBracket, Key::KpAdd),
];

fn main() -> ExitCode {
    match create_glfw_window("FontSDF") {
        Ok(()) => ExitCode::SUCCESS,
        Err(msg) => {
            println!("{}", msg);
            ExitCode::FAILURE
        }
    }
}

fn create_glfw_window(title: &str) -> Result<(), String> {
    let mut glfw = glfw::init(glfw::fail_on_errors).expect("GLFW: Failed on init.");
    // create window
    glfw.window_hint(WindowHint::ContextVersion(4, 6));
    glfw.window_hint(WindowHint::OpenGlProfile(OpenGlProfileHint::Core));
    glfw.window_hint(WindowHint::OpenGlForwardCompat(true));
    glfw.window_hint(WindowHint::Resizable(true));
    glfw.window_hint(WindowHint::Maximized(true));
    // glfw.window_hint(WindowHint::Decorated(false));
    // let (mut window, events) =
    // unfortunately, fullscreen on wayland is kind of broken because the fullscreen window
    // disappears as soon as another window is focused.
    // search for left-most monitor
    let (mut window, events) = glfw
        // .with_connected_monitors(|glfw, ms| {
        //     let monitor = ms
        //         .iter()
        //         .reduce(|m1, m2| {
        //             if m1.get_pos().0 < m2.get_pos().0 {
        //                 m1
        //             } else {
        //                 m2
        //             }
        //         })
        //         .unwrap();
        //     glfw.create_window(WIDTH, HEIGHT, title, glfw::WindowMode::FullScreen(monitor))
        // })
        .create_window(WIDTH, HEIGHT, title, glfw::WindowMode::Windowed)
        .expect("GLFW: Failed on window creation.");
    // get the actual screen resulution size
    window.make_current();
    window.set_key_polling(true);
    // init opengl
    gl::load_with(|s| {
        window
            .get_proc_address(s)
            .map_or(std::ptr::null(), |p| p as *const _)
    });
    glfw.set_swap_interval(glfw::SwapInterval::Sync(1));
    let quad = Vao::create_quad()?;
    let shader = ShaderProgram::new(
        include_str!("../res/shaders/vertex.glsl"),
        include_str!("../res/shaders/fragment.glsl"),
    )?;
    // main loop
    while !window.should_close() {
        // handle events
        glfw.poll_events();
        for (_, event) in glfw::flush_messages(&events) {
            glfw_handle_event(&mut window, event);
        }
        // render
        let window_size = window.get_framebuffer_size();
        unsafe {
            gl::Viewport(0, 0, window_size.0, window_size.1);
            // gl::Clear(gl::COLOR_BUFFER_BIT);
            // gl::ClearColor(0.2, 0.3, 0.4, 1.0);
        }
        shader.start();
        quad.render();
        shader.stop();
        // draw on screen
        window.swap_buffers();
    }
    Ok(())
}

fn glfw_handle_event(window: &mut Window, event: WindowEvent) {
    if let WindowEvent::Key(key, _, Action::Press, _) = event {
        let key = correct_glfw_key(key);
        if key == Key::Escape {
            window.set_should_close(true);
        }
    }
}

fn correct_glfw_key(key: Key) -> Key {
    for (k1, k2) in KEY_MAP {
        if *k1 == key {
            return *k2;
        }
        if *k2 == key {
            return *k1;
        }
    }
    key
}
