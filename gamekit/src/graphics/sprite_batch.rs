//
// Sprite batch
//

use super::{types::{Vertex}, texture::Texture, primitives::{self, Primitives}, api::Api, buffers::{VertexArray, BufferObject, BufferUsage, BufferType}, sprite::Sprite};

defaults!();

pub struct SpriteBatch {
    texture_id: u32,
    vertices: Vec<Vertex>,
    _indices: Vec<u32>,
    vertex_array: VertexArray,
    index_buffer: BufferObject,
    size: usize,
    modified: bool,
    count: usize
}

impl SpriteBatch {

    pub fn new<'b>(texture: &Texture, size: usize) -> Result<SpriteBatch, String> {

        let vertices = vec![Vertex::zero(); size * 4];

        let mut indices = vec![0; size * 6];

        let mut idx = 0;
        let mut ofs = 0;
        while idx < size * 6 {

            // repeat indices: 3, 2, 0, 2, 1, 0

            indices[idx] = ofs + 3; idx += 1;
            indices[idx] = ofs + 2; idx += 1;
            indices[idx] = ofs + 0; idx += 1;
            indices[idx] = ofs + 2; idx += 1;
            indices[idx] = ofs + 1; idx += 1;
            indices[idx] = ofs + 0; idx += 1;

            ofs += 4;
        }

        let mut vertex_array = VertexArray::new(BufferUsage::StaticDraw, 0)?;
        vertex_array.copy_vertices_to_buffer(&vertices, vertices.len());

        let index_buffer = BufferObject::new(BufferType::IndexBuffer, BufferUsage::StaticDraw)?;
        index_buffer.copy_u32_to_buffer(&indices, indices.len());

        let sprite_batch = SpriteBatch {
            texture_id: texture.id(),
            vertices,
            _indices: indices,
            vertex_array,
            index_buffer,
            size,
            modified: false,
            count: 0
        };

        return Ok(sprite_batch);
    }

    pub fn update(&mut self) {
        if !self.modified { return; }
        if self.count > 0 {
            self.vertex_array.copy_vertices_to_buffer(&self.vertices, self.count * 4);
        }
        self.modified = false;
    }

    pub fn begin(&mut self) {
        self.count = 0;
    }

    pub fn end(&mut self) {
        self.update();
    }

    pub fn push(&mut self, sprite: &dyn Sprite) {
        if self.count >= self.size { panic!("sprite batch overflow"); }

        let index = self.count as u32;
        self.count += 1;
        self.modified = true;

        let data = sprite.get_sprite_data();

        let x0 = data.position.x;
        let y0 = data.position.y;
        let x1 = x0 + data.size.x;
        let y1 = y0 + data.size.y;
        let z = 0.0f32;

        let u0 = data.texture_coords.x;
        let v0 = data.texture_coords.y;
        let u1 = u0 + data.texture_coords.w;
        let v1 = v0 + data.texture_coords.h;

        let red = data.color.red;
        let green = data.color.green;
        let blue = data.color.blue;
        let alpha = data.color.alpha;

        let ofs = index as usize * 4;

        {
            let vertex = &mut self.vertices[ofs+0];
            vertex.coords.set(x0, y0, z);
            vertex.color.set(red, green, blue, alpha);
            vertex.texcoords.set(u0, v0);
        }

        {
            let vertex = &mut self.vertices[ofs+1];
            vertex.coords.set(x1, y0, z);
            vertex.color.set(red, green, blue, alpha);
            vertex.texcoords.set(u1, v0);
        }

        {
            let vertex = &mut self.vertices[ofs+2];
            vertex.coords.set(x1, y1, z);
            vertex.color.set(red, green, blue, alpha);
            vertex.texcoords.set(u1, v1);
        }

        {
            let vertex = &mut self.vertices[ofs+3];
            vertex.coords.set(x0, y1, z);
            vertex.color.set(red, green, blue, alpha);
            vertex.texcoords.set(u0, v1);
        }

    }

    pub fn push_vec(&mut self, sprites: Vec<&dyn Sprite>, offset: usize, count: usize) {
        assert!(offset+count < sprites.len());
        for i in offset..(offset+count) {
            self.push(sprites[i]);
        }
    }

    pub fn draw(&self, api: &mut dyn Api) {
        self.draw_buffered(self.count, api);
    }

    pub fn draw_buffered(&self, count: usize, api: &mut dyn Api) {

        if 0 == count {
            return;
        }

        Primitives::bind_texture(self.texture_id, 0);

        self.vertex_array.bind();
        self.index_buffer.bind();

        api.draw_elements(primitives::DrawMode::Triangles, count * 6);

        self.vertex_array.unbind();
        self.index_buffer.unbind();

    }

    pub fn capacity(&self) -> usize {
        return self.size;
    }

}
