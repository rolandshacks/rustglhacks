//
// Executor
//

defaults!();

static BATCH_CAPACITY: usize = 100;
static NUM_PARTICLES:usize = BATCH_CAPACITY;

use std::{mem};

use crate::{graphics::{
    shader::{Shader, ShaderType},
    program::Program,
    buffers::{BufferUsage, ShaderStorageBufferObject, UniformBufferObject},
    texture::Texture,
    application,
    api::{Api, BlendMode}, sprite_batch::{SpriteBatch}
}, entity::Entity};

//const SHADER: &[u8] = include_bytes!("<shader_name>.spv");

#[repr(C)]
struct ShaderData {
    resolution_x: f32,
    resolution_y: f32,
    x_min: f32,
    x_max: f32,
    y_min: f32,
    y_max: f32,
    time: f32,
    time_delta: f32,
    frame: i32,
}

#[repr(C)]
struct UniformData {
    a: f32,
    b: f32
}

#[allow(dead_code)]
pub struct MyExecutor {
    program: Program,
    texture: Texture,
    shader_data: ShaderData,
    shader_data_object: ShaderStorageBufferObject,
    uniform_data: UniformData,
    uniform_data_object: UniformBufferObject,
    sprite_batch: SpriteBatch,
    entities: Vec<Entity>
}

impl application::Executor for MyExecutor {

    fn new(api: &mut dyn Api) -> Result<MyExecutor, String> {
        info!("Executor new");

        let metrics = api.get_metrics();

        let texture = Texture::new("assets/particle.png")?;
        let vertex_shader = Shader::new(ShaderType::VertexShader, "shaders/shader.vert.spv")?;
        let fragment_shader = Shader::new(ShaderType::FragmentShader, "shaders/shader.frag.spv")?;
        let program = Program::new(&vec!(vertex_shader, fragment_shader))?;

        let shader_data = ShaderData {
            resolution_x: metrics.width as f32,
            resolution_y: metrics.height as f32,
            x_min: 0.0f32,
            x_max: metrics.width as f32,
            y_min: 0.0f32,
            y_max: metrics.height as f32,
            time: metrics.time_seconds,
            time_delta: metrics.delta_seconds,
            frame: metrics.frame_counter as i32
        };

        let mut shader_data_object = ShaderStorageBufferObject::new(1, BufferUsage::DynamicDraw)?;
        let data_size = std::mem::size_of_val(&shader_data);
        let data_ptr: *const u8 = unsafe { mem::transmute(&shader_data) };
        shader_data_object.copy_to_buffer(data_ptr, data_size);

        let uniform_data = UniformData {
            a: 1.11,
            b: 2.22
        };

        let mut uniform_data_object = UniformBufferObject::new(2, BufferUsage::DynamicDraw)?;
        let data_size = std::mem::size_of_val(&uniform_data);
        let data_ptr: *const u8 = unsafe { mem::transmute(&uniform_data) };
        uniform_data_object.copy_to_buffer(data_ptr, data_size);

        let sprite_batch = SpriteBatch::new(&texture, BATCH_CAPACITY)?;

        //let entities = vec![Entity::new(); sprite_batch.capacity()];
        let entities = vec![Entity::new(); NUM_PARTICLES];

        let executor = MyExecutor {
            program,
            texture,
            shader_data,
            shader_data_object,
            uniform_data,
            uniform_data_object,
            sprite_batch,
            entities
        };

        return Ok(executor);
    }

    fn initialize(&mut self, api: &mut dyn Api) {
        info!("Executor initialize");

        for entity in &mut self.entities {
            entity.initialize(api);
        }

    }

    fn free(&mut self, _api: &mut dyn Api) {
        info!("Executor free");
    }

    fn update_layout(&mut self, _api: &mut dyn Api) {
        info!("Executor update layout");
    }

    fn update_state(&mut self, api: &mut dyn Api, delta: f32) {
        //info!("Executor update state");

        self.sprite_batch.begin();

        for entity in &mut self.entities {
            entity.update(api, delta);
            self.sprite_batch.push(entity);
        }

        self.sprite_batch.end();

    }

    fn update_graphics(&mut self, api: &mut dyn Api) {
        //info!("Executor update graphics");

        api.clear();

        self.program.use_program();
        self.update_uniforms(api);
        self.shader_data_object.unbind();

        self.shader_data_object.bind();

        api.set_blend_mode(BlendMode::Additive);
        self.sprite_batch.draw(api);
        api.set_blend_mode(BlendMode::Normal);

        self.shader_data_object.unbind();

    }

}

impl MyExecutor {
    fn update_uniforms(&mut self, api: &mut dyn Api) {

        let metrics = api.get_metrics();

        {
            let data = &mut self.shader_data;

            data.time = metrics.time_seconds;
            data.time_delta = metrics.delta_seconds;
            data.frame = metrics.frame_counter as i32;

            let data_size = std::mem::size_of_val(data);
            let data_ptr: *const u8 = unsafe { mem::transmute(&self.shader_data) };
            self.shader_data_object.copy_to_buffer(data_ptr, data_size);
        }

        {
            let data = &mut self.uniform_data;

            data.a = metrics.time_seconds;
            data.b = metrics.time_seconds + 1.0;
            data.b = metrics.time_seconds + 2.0;

            let data_size = std::mem::size_of_val(data);
            let data_ptr: *const u8 = unsafe { mem::transmute(&self.uniform_data) };
            self.uniform_data_object.copy_to_buffer(data_ptr, data_size);
        }

    }
}
