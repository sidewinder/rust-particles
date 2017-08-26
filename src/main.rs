extern crate sdl2;
extern crate gl;
extern crate rand;

mod particle_system;
mod graphics;

use graphics::shader;
use particle_system::ParticleSystem;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;

use std::time::Instant;
use std::time::Duration;
use std::os::raw::c_void;
use std::os::raw::c_char;


fn update(dt: f64) {

}


fn render(particle_system: &ParticleSystem) {
    unsafe { 
        gl::Viewport(0, 0, 1600, 900);
        gl::ClearColor(0.2, 0.2, 0.2, 1.0);
        gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT); 
    }

    particle_system.render();

    unsafe {
        gl::Flush();
    }
}

#[no_mangle]
pub extern "system" fn debug_callback(source: gl::types::GLenum,
        err_type: gl::types::GLenum,
        id: gl::types::GLuint,
        severity: gl::types::GLenum,
        length: gl::types::GLsizei,
        message: *const c_char,
        userParam: *mut c_void) {

    unsafe{
        let err_text = std::ffi::CStr::from_ptr(message);
        println!("Type: {:#x} ID: {:#x} Severity: {:#x}:\n  {:#?}", err_type, id, severity, err_text.to_str().unwrap())
    }

    if severity >= gl::DEBUG_SEVERITY_HIGH {
        panic!();
    }
}

fn main() {
    
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let gl_attr = video_subsystem.gl_attr();

    gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
    // Set the context into debug mode
    gl_attr.set_context_flags().debug().set();

    gl_attr.set_context_version(4, 3);

    let window = video_subsystem.window("Rust SDL window", 1600, 900)
        .position_centered()
        .opengl()
        .build()
        .unwrap();

    let gl_context = window.gl_create_context();
    println!("Started with GL version: {:?}", gl_attr.context_version());

    gl::load_with(|name| video_subsystem.gl_get_proc_address(name) as *const _);
    unsafe { 
        gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA); 
        gl::Enable(gl::BLEND);

        gl::DebugMessageCallback(debug_callback, std::ptr::null()) 
    };

    video_subsystem.gl_set_swap_interval(1);

    let mut event_pump = sdl_context.event_pump().unwrap();

    unsafe { println!("OpenGL version is {:?}", gl::GetString(gl::VERSION)) };
    let mut particle_system = ParticleSystem::new(10);
    particle_system.init_graphics_resources();
    
    let mut prev_time = Instant::now();

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} | Event::KeyDown {keycode: Some(Keycode::Escape), ..} => {
                    break 'running
                }
                _ => {}
            }
        }

        let time_now = Instant::now();
        let dt = time_now - prev_time;
        let dt_sec = dt.as_secs() as f64 + dt.subsec_nanos() as f64 * 1e-9;
        prev_time = time_now;
        update(dt_sec);
        
        render(&particle_system);
        window.gl_swap_window();

        std::thread::sleep(Duration::new(0, 1_000_000_000u32/60));
    }

}
