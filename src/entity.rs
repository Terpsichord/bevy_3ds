use crate::vertices::{Vertex, VERTICES};
use crate::Scene;
use citro3d::{buffer, math::Matrix4, render::ScreenTarget, uniform, Frame, RenderPass};
use std::{default::Default, iter};

pub struct Entity {
    #[allow(dead_code)]
    update_flag: bool,
    render_flag: bool,

    vertex_list: Vec<Vertex, ctru::linear::LinearAllocator>,

    transform: Transform,
}

pub struct Transform {
    pub angle_x: f32,
    pub angle_y: f32,
    pub pos_x: f32,
    pub pos_z: f32,
    pub scale_x: f32,
    pub scale_y: f32,
    pub scale_z: f32,
}

impl Default for Transform {
    fn default() -> Self {
        Self {
            angle_x: 0.0,
            angle_y: 0.0,
            pos_x: 0.0,
            pos_z: 0.0,
            scale_x: 1.0,
            scale_y: 1.0,
            scale_z: 1.0,
        }
    }
}

impl Transform {
    fn model_matrix(&self) -> Matrix4 {
        let mut model = Matrix4::identity();

        model.scale(self.scale_x, self.scale_y, self.scale_z);
        model.rotate_x(self.angle_x);
        model.rotate_y(self.angle_y);
        model.translate(self.pos_x, 0.0, self.pos_z);

        model
    }
}

impl Entity {
    pub fn new(vertex_list: &[Vertex]) -> Self {
        let mut vbo_data = Vec::with_capacity_in(vertex_list.len(), ctru::linear::LinearAllocator);
        vbo_data.extend_from_slice(VERTICES);

        Self {
            update_flag: true,
            render_flag: true,
            vertex_list: vbo_data,
            transform: Transform::default(),
        }
    }

    pub fn with_transform(mut self, transform: Transform) -> Self {
        self.transform = transform;
        self
    }

    pub fn update_transform(&mut self, delta: (i16, i16, i16), is_scale: bool) {
        if is_scale {
            self.transform.scale_x += delta.0 as f32 * 0.1;
            self.transform.scale_y += delta.1 as f32 * 0.1;
            self.transform.scale_z += delta.2 as f32 * 0.1;
        } else {
            self.transform.angle_y += delta.0 as f32 * 0.02;
            self.transform.angle_x += delta.1 as f32 * 0.02;
            self.transform.pos_z += delta.2 as f32 * 0.1;
        }
    }

    pub fn render<'a>(
        &self,
        frame: &mut Frame<'_, 'a>,
        target: &'a ScreenTarget,
        scene: &'a Scene,
        uniforms: impl IntoIterator<Item = (uniform::Index, Matrix4)>,
        model_uniform_idx: uniform::Index,
    ) {
        let mut buf_info = buffer::Info::new(buffer::Primitive::Triangles);
        let buffer_idx = buf_info.add(&self.vertex_list, &scene.attr_info).unwrap();
        let vbo_data = buf_info.buffer(buffer_idx).unwrap();

        let pass = RenderPass::new(&scene.program, target, vbo_data, &scene.attr_info)
            .with_texenv_stages(&scene.texenv_stages)
            .with_lightenv(&scene.light_env)
            .with_vertex_uniforms(uniforms.into_iter().map(|(i, mtx)| (i, mtx.into())).chain(
                iter::once((model_uniform_idx, (&self.transform.model_matrix()).into())),
            ));

        frame.draw(&pass).unwrap();
    }

    pub fn toggle_render(&mut self) {
        self.render_flag = !self.render_flag;
    }
}
