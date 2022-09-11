#![allow(dead_code)]
#![allow(mutable_transmutes)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_assignments)]
#![allow(unused_mut)]
#![allow(clashing_extern_declarations)]
#![feature(c_variadic)]
#![feature(extern_types)]
#![feature(label_break_value)]

pub mod cffi;
pub mod gl;
pub mod ngl;
pub mod tigr;
pub use gl::types::*;

pub const GL_VERTEX_SHADER: u32 = 0x8B31 as cffi::c_int as GLenum;
pub const GL_FRAGMENT_SHADER: u32 = 0x8B30 as cffi::c_int as GLenum;

pub fn create_vertex_shader() -> GLenum {
    unsafe { tigr::glCreateShader(GL_VERTEX_SHADER) }
}

pub fn create_fragment_shader() -> GLenum {
    unsafe { tigr::glCreateShader(GL_FRAGMENT_SHADER) }
}

// pub fn create_shader_program(vertex_src: &str, fragment_src: &str) {}

pub fn create_vao() {
    let mut vao = 0;
    unsafe { tigr::glGenVertexArrays(1, &mut vao) };
}

// pub fn create_vbo(vertices: &[f32]) {}

pub struct RGBColor(pub i32, pub i32, pub i32);

pub struct GLWindow {
    width: i32,
    height: i32,
    title: String,
    flag: i32,
    window_ptr: *mut tigr::Tigr,
}

impl GLWindow {
    pub fn new(width: i32, height: i32, title: &str, flag: i32) -> GLWindow {
        let window_ptr: *mut tigr::Tigr = unsafe {
            tigr::tigrWindow(
                width as cffi::c_int,
                height as cffi::c_int,
                title.as_bytes().as_ptr() as *const cffi::c_char,
                flag as cffi::c_int,
            )
        };
        GLWindow {
            width: width,
            height: height,
            title: String::from(title),
            flag: flag,
            window_ptr: window_ptr,
        }
    }

    pub fn clear(&self, color: RGBColor) {
        unsafe {
            tigr::tigrClear(self.window_ptr, rgb(color.0, color.1, color.2));
        }
    }

    pub fn update(&self) {
        unsafe {
            tigr::tigrUpdate(self.window_ptr);
        }
    }

    pub fn is_running(&self) -> bool {
        unsafe { tigr::tigrClosed(self.window_ptr) as i32 == 0 }
    }

    pub fn print_bitmap(
        &self,
        font: *mut tigr::TigrFont,
        x: i32,
        y: i32,
        color: RGBColor,
        text: &str,
    ) {
        unsafe {
            tigr::tigrPrint(
                self.window_ptr,
                font,
                x as cffi::c_int,
                y as cffi::c_int,
                rgb(color.0, color.1, color.2),
                text.as_bytes().as_ptr() as *const cffi::c_char,
            );
        }
    }

    pub fn begin_gl(&self) {
        unsafe {
            tigr::tigrBeginOpenGL(self.window_ptr);
        }
    }

    pub fn cleanup(&self) {
        unsafe {
            tigr::tigrFree(self.window_ptr);
        }
    }
}

pub fn rgb(r: i32, g: i32, b: i32) -> tigr::TPixel {
    unsafe {
        tigr::tigrRGB(
            r as cffi::c_int as cffi::c_uchar,
            g as cffi::c_int as cffi::c_uchar,
            b as cffi::c_int as cffi::c_uchar,
        )
    }
}
