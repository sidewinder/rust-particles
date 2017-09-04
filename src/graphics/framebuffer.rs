use gl;
use std;
use std::ops::Drop;

pub struct FrameBuffer {
    gl_handle: u32,
    texture_color_buffer_handle: u32
}


impl FrameBuffer {
    pub fn new(width: i32, height: i32) -> FrameBuffer {
        let mut frame_buffer = FrameBuffer { gl_handle: 0, texture_color_buffer_handle: 0 };
        unsafe {
            gl::GenFramebuffers(1, &mut frame_buffer.gl_handle);
            gl::BindFramebuffer(gl::FRAMEBUFFER, frame_buffer.gl_handle);  
           
            gl::GenTextures(1, &mut frame_buffer.texture_color_buffer_handle);
            gl::BindTexture(gl::TEXTURE_2D, frame_buffer.texture_color_buffer_handle);
            gl::TexImage2D(gl::TEXTURE_2D, 0, gl::RGB as i32, width, height, 0, gl::RGB, 
                gl::UNSIGNED_BYTE, std::ptr::null());
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);
            gl::BindTexture(gl::TEXTURE_2D, 0);
            gl::FramebufferTexture2D(gl::FRAMEBUFFER, gl::COLOR_ATTACHMENT0, 
                gl::TEXTURE_2D, frame_buffer.texture_color_buffer_handle, 0);

            let mut render_buffer_obj = 0u32;
            gl::GenRenderbuffers(1, &mut render_buffer_obj);
            gl::BindRenderbuffer(gl::RENDERBUFFER, render_buffer_obj);
            gl::RenderbufferStorage(gl::RENDERBUFFER, gl::DEPTH24_STENCIL8, width, height);
            gl::BindRenderbuffer(gl::RENDERBUFFER, 0);

            gl::FramebufferRenderbuffer(gl::FRAMEBUFFER, gl::DEPTH_STENCIL_ATTACHMENT, 
                gl::RENDERBUFFER, render_buffer_obj);

            if gl::CheckFramebufferStatus(gl::FRAMEBUFFER) != gl::FRAMEBUFFER_COMPLETE {
                panic!("Framebuffer setup failed");
            }

            gl::BindFramebuffer(gl::FRAMEBUFFER, 0);
        }

        frame_buffer
    }

    pub fn bind(&mut self) {
        unsafe {
            gl::BindFramebuffer(gl::FRAMEBUFFER, self.gl_handle);
        }
    }

    pub fn unbind(&self) {
        unsafe {
            gl::BindFramebuffer(gl::FRAMEBUFFER, 0);
        }
    }

    pub fn get_texture_color_buffer(&self) -> u32 {
        self.texture_color_buffer_handle
    }
}


impl Drop for FrameBuffer {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteFramebuffers(1, &mut self.gl_handle);

        }
    }

}