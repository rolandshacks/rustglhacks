use crate::graphics::types::Vertex;

use super::{gl, types::Quad};

defaults!();

pub enum BufferType {
    ArrayBuffer,
    IndexBuffer,
    UniformBuffer,
    ShaderStorageBuffer
}

#[allow(dead_code)]
pub enum BufferUsage {
    StreamDraw,
    StreamRead,
    StreamCopy,
    StaticDraw,
    StaticRead,
    StaticCopy,
    DynamicDraw,
    DynamicRead,
    DynamicCopy
}

pub struct BufferObject {
    id: u32,
    buffer_type: BufferType,
    buffer_usage: BufferUsage
}

impl BufferObject {

    pub fn new(buffer_type: BufferType, buffer_usage: BufferUsage) -> Result<BufferObject, String> {

        let buffer = BufferObject {
            id: create_buffer(),
            buffer_type,
            buffer_usage
        };

        return Ok(buffer);
    }

    pub fn bind(&self) {
        bind_buffer(self.id, &self.buffer_type);
    }

    pub fn unbind(&self) {
        unbind_buffer(self.id, &self.buffer_type);
    }

    pub fn free(&mut self) {
        if self.id != 0 {
            delete_buffer(self.id);
            self.id = 0;
        }
    }

    pub fn copy_vertices_to_buffer(&mut self, data: &Vec<Vertex>, count: usize) {

        self.bind();

        copy_vertices_to_buffer(&self.buffer_type, &self.buffer_usage, data, count);

        self.unbind();

    }

    pub fn copy_quads_to_buffer(&mut self, data: &Vec<Quad>, count: usize) {

        self.bind();

        copy_quads_to_buffer(&self.buffer_type, &self.buffer_usage, data, count);

        self.unbind();

    }

    pub fn copy_u32_to_buffer(&self, data: &Vec<u32>, count: usize) {

        self.bind();

        copy_u32_to_buffer(&self.buffer_type, &self.buffer_usage, data, count);

        self.unbind();

    }

    pub fn copy_to_buffer(&mut self, data_ptr: *const u8, data_size: usize) {

        self.bind();

        copy_to_buffer(&self.buffer_type, &self.buffer_usage, data_ptr, data_size);

        self.unbind();

    }

}

impl Drop for BufferObject {
    fn drop(&mut self) {
        debug!("drop buffer object");
        self.free();
    }
}

pub struct ShaderStorageBufferObject {
    id: u32,
    binding_point: u32,
    buffer_usage: BufferUsage,
    initialized: bool
}

impl ShaderStorageBufferObject {
    pub fn new(binding_point: u32, buffer_usage: BufferUsage) -> Result<ShaderStorageBufferObject, String> {

        let buffer = ShaderStorageBufferObject {
            id: create_buffer(),
            binding_point,
            buffer_usage,
            initialized: false
        };

        return Ok(buffer);
    }

    pub fn bind(&self) {
        if !self.initialized {
            bind_buffer(self.id, &BufferType::ShaderStorageBuffer);
            bind_buffer_base(self.id, self.binding_point, &BufferType::ShaderStorageBuffer);
        } else {
            bind_buffer(self.id, &BufferType::ShaderStorageBuffer);
        }
    }

    pub fn unbind(&self) {
        unbind_buffer(self.id, &BufferType::ShaderStorageBuffer);
    }

    pub fn copy_to_buffer(&mut self, data_ptr: *const u8, data_size: usize) {
        self.bind();

        if !self.initialized {
            copy_to_buffer(&BufferType::ShaderStorageBuffer, &self.buffer_usage, data_ptr, data_size);
            self.initialized = true;
        } else {
            copy_to_buffer_sub(&BufferType::ShaderStorageBuffer, data_ptr, 0, data_size);
        }

        self.unbind();
    }

    pub fn free(&mut self) {
        if self.id != 0 {
            delete_buffer(self.id);
            self.id = 0;
        }
    }
}

impl Drop for ShaderStorageBufferObject {
    fn drop(&mut self) {
        debug!("drop shader storage buffer object");
        self.free();
    }
}

pub struct UniformBufferObject {
    id: u32,
    binding_point: u32,
    buffer_usage: BufferUsage,
    initialized: bool
}

impl UniformBufferObject {
    pub fn new(binding_point: u32, buffer_usage: BufferUsage) -> Result<UniformBufferObject, String> {

        let buffer = UniformBufferObject {
            id: create_buffer(),
            binding_point,
            buffer_usage,
            initialized: false
        };

        return Ok(buffer);
    }

    pub fn bind(&self) {
        if !self.initialized {
            bind_buffer_base(self.id, self.binding_point, &BufferType::UniformBuffer);
        } else {
            bind_buffer(self.id, &BufferType::UniformBuffer);
        }
    }

    pub fn unbind(&self) {
        unbind_buffer(self.id, &BufferType::UniformBuffer);
    }

    pub fn copy_to_buffer(&mut self, data_ptr: *const u8, data_size: usize) {
        self.bind();

        if !self.initialized {
            copy_to_buffer(&BufferType::UniformBuffer, &self.buffer_usage, data_ptr, data_size);
            self.initialized = true;
        } else {
            copy_to_buffer_sub(&BufferType::UniformBuffer, data_ptr, 0, data_size);
        }

        self.unbind();
    }

    pub fn free(&mut self) {
        if self.id != 0 {
            delete_buffer(self.id);
            self.id = 0;
        }
    }
}


impl Drop for UniformBufferObject {
    fn drop(&mut self) {
        debug!("drop uniform buffer object");
        self.free();
    }
}

pub struct VertexArray {
    id: u32,
    buffer_object: BufferObject
}

impl VertexArray {

    pub fn new(usage: BufferUsage, start_index: u32) -> Result<VertexArray, String> {

        let buffer_object = BufferObject::new(BufferType::ArrayBuffer, usage)?;

        let mut id: gl::types::GLuint = 0;

        unsafe {
            gl::GenVertexArrays(1, &mut id);
        }

        let vertex_array = VertexArray {
            id,
            buffer_object
        };

        vertex_array.bind();
        vertex_array.buffer_object.bind();

        Vertex::declare_attrib_pointers(start_index);

        vertex_array.buffer_object.unbind();
        vertex_array.unbind();

        return Ok(vertex_array);
    }

    pub fn copy_vertices_to_buffer(&mut self, data: &Vec<Vertex>, count: usize) {
        self.buffer_object.copy_vertices_to_buffer(data, count);
    }

    pub fn copy_quads_to_buffer(&mut self, data: &Vec<Quad>, count: usize) {
        self.buffer_object.copy_quads_to_buffer(data, count);
    }

    pub fn bind(&self) {
        unsafe {
            gl::BindVertexArray(self.id);
        }
    }

    pub fn unbind(&self) {
        unsafe {
            gl::BindVertexArray(0);
        }
    }

    pub fn free(&mut self) {
        info!("Vertex array free");
        if self.id != 0 {
            unsafe {
                gl::DeleteVertexArrays(1, &mut self.id);
            }
            self.id = 0;
        }

        self.buffer_object.free();
    }

}

impl Drop for VertexArray {
    fn drop(&mut self) {
        debug!("drop vertex array");
        self.free();
    }
}

fn create_buffer() -> u32 {
    let mut id: gl::types::GLuint = 0;
    unsafe { gl::GenBuffers(1, &mut id); };
    return id as u32;
}

fn delete_buffer(id: u32) {
    if id != 0 {
        unsafe {
            let mut buffer_id = id;
            gl::DeleteBuffers(1, &mut buffer_id);
        }
    }
}

fn bind_buffer(id: u32, buffer_type: &BufferType) {
    unsafe {
        gl::BindBuffer(map_type(buffer_type), id);
    }
}

fn bind_buffer_base(id: u32, binding_point: u32, buffer_type: &BufferType) {
    unsafe {
        gl::BindBufferBase(map_type(buffer_type), binding_point, id);
    }
}

fn unbind_buffer(_id: u32, buffer_type: &BufferType) {
    unsafe {
        gl::BindBuffer(map_type(buffer_type), 0);
    }
}

fn map_usage(usage: &BufferUsage) -> gl::types::GLenum {
    match usage {
        BufferUsage::StreamDraw => gl::STREAM_DRAW,
        BufferUsage::StreamRead => gl::STREAM_READ,
        BufferUsage::StreamCopy => gl::STREAM_COPY,
        BufferUsage::StaticDraw => gl::STATIC_DRAW,
        BufferUsage::StaticRead => gl::STATIC_READ,
        BufferUsage::StaticCopy => gl::STATIC_COPY,
        BufferUsage::DynamicDraw => gl::DYNAMIC_DRAW,
        BufferUsage::DynamicRead => gl::DYNAMIC_READ,
        BufferUsage::DynamicCopy => gl::DYNAMIC_COPY,
    }
}

fn map_type(buffer_type: &BufferType) -> gl::types::GLenum  {
    match buffer_type {
        BufferType::ArrayBuffer => gl::ARRAY_BUFFER,
        BufferType::IndexBuffer => gl::ELEMENT_ARRAY_BUFFER,
        BufferType::UniformBuffer => gl::UNIFORM_BUFFER,
        BufferType::ShaderStorageBuffer => gl::SHADER_STORAGE_BUFFER
    }
}

fn copy_vertices_to_buffer(buffer_type: &BufferType, buffer_usage: &BufferUsage, data: &Vec<Vertex>, count: usize) {

    let data_ptr = data.as_ptr();
    let data_size = count * std::mem::size_of::<Vertex>();

    copy_to_buffer(buffer_type, buffer_usage, data_ptr as *const u8, data_size);
}

fn copy_quads_to_buffer(buffer_type: &BufferType, buffer_usage: &BufferUsage, data: &Vec<Quad>, count: usize) {

    let data_ptr = data.as_ptr();
    let data_size = count * std::mem::size_of::<Quad>();

    copy_to_buffer(buffer_type, buffer_usage, data_ptr as *const u8, data_size);
}

fn copy_u32_to_buffer(buffer_type: &BufferType, buffer_usage: &BufferUsage, data: &Vec<u32>, count: usize) {
    let data_ptr = data.as_ptr();
    let data_size = count * std::mem::size_of::<u32>();
    copy_to_buffer(buffer_type, buffer_usage, data_ptr as *const u8, data_size);
}

fn copy_to_buffer(buffer_type: &BufferType, buffer_usage: &BufferUsage, data_ptr: *const u8, data_size: usize) {
    let data_usage = map_usage(buffer_usage);
    unsafe {
        gl::BufferData(
            map_type(buffer_type),
            data_size as gl::types::GLsizeiptr,
            data_ptr as *const gl::types::GLvoid,
            data_usage
        );
    }
}

fn copy_to_buffer_sub(buffer_type: &BufferType, data_ptr: *const u8, offset: usize, data_size: usize) {
    unsafe {
        gl::BufferSubData(
            map_type(buffer_type),
            offset as gl::types::GLintptr,
            data_size as gl::types::GLsizeiptr,
            data_ptr as *const gl::types::GLvoid
        );
    }
}

#[allow(dead_code)]
fn map_and_copy(buffer_type: &BufferType, data_ptr: *const u8, data_size: usize) {
    unsafe {
        let type_id = map_type(buffer_type);
        let ptr = gl::MapBuffer(type_id, gl::WRITE_ONLY);
        std::ptr::copy_nonoverlapping(data_ptr, ptr as *mut u8 , data_size);
        gl::UnmapBuffer(type_id);
    }
}
