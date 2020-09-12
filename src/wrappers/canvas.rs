use std::ffi::{CString, CStr};
use std::os::raw::c_char;

use crate::wrappers::structs::{
    Vector,
    Vector2,
    Vector2f,
    LinearColor
};

pub struct Canvas(usize);

impl Canvas {
    pub fn new(addr: usize) -> Canvas {
        Canvas(addr)
    }

    pub fn addr(&self) -> usize {
        self.0
    }

    pub fn set_position<T: Into<Vector2f>>(&self, pos: T) {
        let pos: Vector2f = pos.into();
        let pos = &pos as *const Vector2f;
        unsafe { Canvas_SetPosition(self.addr(), pos); }
    }

    pub fn get_position_float(&self) -> Vector2f {
        let mut result = Vector2f::new();
        let result_ptr = &result as *const Vector2f;
        unsafe { Canvas_GetPositionFloat(self.addr(), result_ptr); }
        result
    }

    pub fn set_color_chars(&self, red: u8, green: u8, blue: u8, alpha: u8) {
        unsafe { Canvas_SetColor_chars(self.addr(), red, green, blue, alpha); }
    }

    pub fn set_color(&self, color: LinearColor) {
        let color = &color as *const LinearColor;
        unsafe { Canvas_SetColor(self.addr(), color); }
    }

    pub fn get_color(&self) -> LinearColor {
        let mut result = LinearColor::new();
        let result_ptr = &result as *const LinearColor;
        unsafe { Canvas_GetColor(self.addr(), result_ptr); }
        result
    }

    pub fn draw_box<T: Into<Vector2f>>(&self, size: T) {
        let size: Vector2f = size.into();
        let size = &size as *const Vector2f;
        unsafe { Canvas_DrawBox(self.addr(), size); }
    }

    pub fn fill_box<T: Into<Vector2f>>(&self, size: T) {
        let size: Vector2f = size.into();
        let size = &size as *const Vector2f;
        unsafe { Canvas_FillBox(self.addr(), size); }
    }

    pub fn fill_triangle<T: Into<Vector2f>>(&self, p1: T, p2: T, p3: T) {
        let p1: Vector2f = p1.into();
        let p1 = &p1 as *const Vector2f;
        let p2: Vector2f = p2.into();
        let p2 = &p2 as *const Vector2f;
        let p3: Vector2f = p3.into();
        let p3 = &p3 as *const Vector2f;
        unsafe { Canvas_FillTriangle(self.addr(), p1, p2, p3); }
    }

    pub fn fill_triangle_color<T: Into<Vector2f>>(&self, p1: T, p2: T, p3: T, color: LinearColor) {
        let p1: Vector2f = p1.into();
        let p1 = &p1 as *const Vector2f;
        let p2: Vector2f = p2.into();
        let p2 = &p2 as *const Vector2f;
        let p3: Vector2f = p3.into();
        let p3 = &p3 as *const Vector2f;
        let color = &color as *const LinearColor;
        unsafe { Canvas_FillTriangle_color(self.addr(), p1, p2, p3, color); }
    }

    pub fn draw_string(&self, text: &str) {
        let c_text = CString::new(text).unwrap();
        let c_text: *const c_char = c_text.as_ptr();
        unsafe { Canvas_DrawString(self.addr(), c_text); }
    }

    pub fn draw_string_scale(&self, text: &str, x_scale: f32, y_scale: f32) {
        let c_text = CString::new(text).unwrap();
        let c_text: *const c_char = c_text.as_ptr();
        unsafe { Canvas_DrawString_pos(self.addr(), c_text, x_scale, y_scale); }
    }

    pub fn get_string_size(&self, text: &str, x_scale: f32, y_scale: f32) -> Vector2f {
        let c_text = CString::new(text).unwrap();
        let c_text: *const c_char = c_text.as_ptr();
        let mut result = Vector2f::new();
        let result_ptr = &result as *const Vector2f;
        unsafe { Canvas_GetStringSize(self.addr(), c_text, x_scale, y_scale, result_ptr); }
        result
    }

    pub fn draw_line<T: Into<Vector2f>>(&self, start: T, end: T) {
        let start: Vector2f = start.into();
        let start = &start as *const Vector2f;
        let end: Vector2f = end.into();
        let end = &end as *const Vector2f;
        unsafe { Canvas_DrawLine(self.addr(), start, end); }
    }

    pub fn draw_line_width<T: Into<Vector2f>>(&self, start: T, end: T, width: f32) {
        let start: Vector2f = start.into();
        let start = &start as *const Vector2f;
        let end: Vector2f = end.into();
        let end = &end as *const Vector2f;
        unsafe { Canvas_DrawLine_width(self.addr(), start, end, width); }
    }

    pub fn draw_rect<T: Into<Vector2f>>(&self, start: T, end: T) {
        let start: Vector2f = start.into();
        let start = &start as *const Vector2f;
        let end: Vector2f = end.into();
        let end = &end as *const Vector2f;
        unsafe { Canvas_DrawRect(self.addr(), start, end); }
    }

    pub fn project(&self, location: Vector) -> Vector2 {
        let location = &location as *const Vector;
        let mut result = Vector2::new();
        let result_ptr = &result as *const Vector2;
        unsafe { Canvas_Project(self.addr(), location, result_ptr); }
        result
    }

    pub fn project_f(&self, location: Vector) -> Vector2f {
        let location = &location as *const Vector;
        let mut result = Vector2f::new();
        let result_ptr = &result as *const Vector2f;
        unsafe { Canvas_ProjectF(self.addr(), location, result_ptr); }
        result
    }
    pub fn get_size(&self) -> Vector2 {
        let mut result = Vector2::new();
        let result_ptr = &result as *const Vector2;
        unsafe { Canvas_GetSize(self.addr(), result_ptr); }
        result
    }
}

extern "C" {
    fn Canvas_SetPosition(obj: usize, pos: *const Vector2f);
    fn Canvas_GetPositionFloat(obj: usize, result: *const Vector2f);
    fn Canvas_SetColor_chars(obj: usize, Red: u8, Green: u8, Blue: u8, Alpha: u8);
    fn Canvas_SetColor(obj: usize, color: *const LinearColor);
    fn Canvas_GetColor(obj: usize, result: *const LinearColor);
    fn Canvas_DrawBox(obj: usize, size: *const Vector2f);
    fn Canvas_FillBox(obj: usize, size: *const Vector2f);
    fn Canvas_FillTriangle(obj: usize, p1: *const Vector2f, p2: *const Vector2f, p3: *const Vector2f);
    fn Canvas_FillTriangle_color(obj: usize, p1: *const Vector2f, p2: *const Vector2f, p3: *const Vector2f, color: *const LinearColor);
    fn Canvas_DrawString(obj: usize, text: *const c_char);
    fn Canvas_DrawString_pos(obj: usize, text: *const c_char, xScale: f32, yScale: f32);
    fn Canvas_GetStringSize(obj: usize, text: *const c_char, xScale: f32, yScale: f32, result: *const Vector2f);
    fn Canvas_DrawLine(obj: usize, start: *const Vector2f, end: *const Vector2f);
    fn Canvas_DrawLine_width(obj: usize, start: *const Vector2f, end: *const Vector2f, width: f32);
    fn Canvas_DrawRect(obj: usize, start: *const Vector2f, end: *const Vector2f);
    fn Canvas_SetPositionI(obj: usize, pos: *const Vector2);
    fn Canvas_GetPositionI(obj: usize, result: *const Vector2);
    fn Canvas_DrawBoxI(obj: usize, size: *const Vector2);
    fn Canvas_FillBoxI(obj: usize, size: *const Vector2);
    fn Canvas_FillTriangleI(obj: usize, p1: *const Vector2, p2: *const Vector2, p3: *const Vector2);
    fn Canvas_FillTriangle_colorI(obj: usize, p1: *const Vector2, p2: *const Vector2, p3: *const Vector2, color: *const LinearColor);
    fn Canvas_DrawLineI(obj: usize, start: *const Vector2, end: *const Vector2);
    fn Canvas_DrawLineWidthI(obj: usize, start: *const Vector2, end: *const Vector2, width: f32);
    fn Canvas_DrawRectI(obj: usize, start: *const Vector2, end: *const Vector2);
    fn Canvas_Project(obj: usize, location: *const Vector, result: *const Vector2);
    fn Canvas_ProjectF(obj: usize, location: *const Vector, result: *const Vector2f);
    fn Canvas_GetSize(obj: usize, result: *const Vector2);
}