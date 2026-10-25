use std::mem::ManuallyDrop;
use std::pin::Pin;
use std::sync::LazyLock;
use ouroboros::self_referencing;
use bevy::prelude::*;

use citro3d::{
    attrib,
    light::{LightEnv, LightLut, LightLutId, LutInput},
    math::{AspectRatio, ClipPlanes, FVec3, Matrix4, Projection, StereoDisplacement},
    render::{self, ClearFlags, ScreenTarget, Target},
    shader, texenv, uniform
};
use citro3d::macros::include_shader;
use citro3d::material::Material;
use ctru::services::gfx::{Gfx, RawFrameBuffer, Screen, TopScreen3D};
use crate::vertices::{Vertex, VERTICES};

static SHADER_BYTES: &[u8] = include_shader!("../../assets/vshader.v.pica");
const CLEAR_COLOR: u32 = 0x68_B0_D8_FF;

pub struct RenderPlugin; 

impl Plugin for RenderPlugin {
    fn build(&self, app: &mut App) {
        println!("init plugin");
        let instance = citro3d::Instance::new().expect("failed to initialize Citro3D");

        let gfx = app.world_mut().remove_non_send_resource::<Gfx>().expect("RenderPlugin requires the GFX resource");
        let targets = Targets::new(gfx, &instance);
        app.insert_non_send_resource(targets);

        app.insert_non_send_resource(instance);

        app.add_systems(Startup, init_scene);
        app.add_systems(PostUpdate, render);
        println!("initted");
    }
}

#[self_referencing]
struct TargetsInner {
    gfx: Gfx,
    #[borrows(gfx)]
    #[not_covariant]
    top_screen: ManuallyDrop<TopScreen3D<'this>>,
    #[borrows(mut top_screen)]
    #[not_covariant]
    targets: (ScreenTarget<'this>, ScreenTarget<'this>),
}

struct Targets(TargetsInner);

impl Targets {
    fn new(gfx: Gfx, instance: &citro3d::Instance) -> Self {
        Targets(TargetsInnerBuilder {
            gfx,
            top_screen_builder: |gfx| ManuallyDrop::new(TopScreen3D::from(&gfx.top_screen)),
            targets_builder: |top_screen| {
                let (mut top_left, mut top_right) = top_screen.split_mut();

                let RawFrameBuffer { width, height, .. } = top_left.raw_framebuffer();
                let top_left_target = instance
                    .render_target(
                        width,
                        height,
                        top_left,
                        Some(render::DepthFormat::Depth24Stencil8),
                    )
                    .expect("failed to create render target");

                let RawFrameBuffer { width, height, .. } = top_right.raw_framebuffer();
                let top_right_target = instance
                    .render_target(
                        width,
                        height,
                        top_right,
                        Some(render::DepthFormat::Depth24Stencil8),
                    )
                    .expect("failed to create render target");

                (top_left_target, top_right_target)
            }
        }.build())
    }
}

impl Drop for Targets {
    fn drop(&mut self) {
        // TODO: i think this is safe, not sure though
        unsafe {
            // this line doesn't compile however the line below should do the same thing
            // ManuallyDrop::drop(&mut self.0.top_screen);
            ctru_sys::gfxSet3D(false);
        }
    }
}

#[derive(Component)]
pub struct Camera {
    pub fov: f32,
}

#[derive(Component)]
pub struct Cube;

fn render(mut instance: NonSendMut<citro3d::Instance>, mut screen_targets: NonSendMut<Targets>, scene: NonSend<Scene>, camera: Query<(&Transform, &Camera)>, cubes: Query<&Transform, With<Cube>>) {
    println!("render frame");
    let Ok((camera_transform, camera)) = camera.get_single() else { return };

    let screen_depth = 2.0;
    let (left_eye, right_eye) = calculate_projections(camera.fov, screen_depth);
    let view = mat4_to_matrix4(camera_transform.compute_matrix().inverse());

    let [projection_idx, model_idx, view_idx] = get_uniform_indexes(&scene.program);

    screen_targets.0.with_targets_mut(|screen_targets| {
        let targets = [(&mut screen_targets.0, left_eye), (&mut screen_targets.1, right_eye)];
        instance.render_frame_with(|frame| {
            for (target, projection) in targets {
                target.clear(ClearFlags::ALL, CLEAR_COLOR, 0);

                let uniforms = [
                    (projection_idx, projection),
                    (view_idx, view),
                ];

                unsafe {
                    frame.instance.bind_program(&scene.program);
                    frame.instance.select_render_target(target).unwrap();

                    // Uniforms
                    frame.instance.set_attr_info(&scene.attr_info);

                    for &(index, uniform) in uniforms.iter() {
                        frame.instance.bind_vertex_uniform(index, uniform);
                    }

                    // Texenvs
                    let texenv_count: usize = 6;
                    for i in 0..texenv_count {
                        if let Some(texenv) = scene.texenv_stages.get(i) {
                            texenv
                                .set_texenv(i)
                                .expect("Texenv stage should always be in bounds");
                        } else {
                            let texenv = citro3d_sys::C3D_GetTexEnv(i as i32);
                            if !texenv.is_null() {
                                citro3d_sys::C3D_TexEnvInit(texenv);
                                citro3d_sys::C3D_DirtyTexEnv(texenv);
                            }
                        }
                    }

                    // Light env
                    let lightenv: *mut citro3d_sys::C3D_LightEnv = scene.light_env.as_raw() as *const _ as *mut _;
                    citro3d_sys::C3D_LightEnvBind(lightenv);
                }

                for transform in cubes.iter() {
                    render_cube(transform, frame.instance, &scene.attr_info, model_idx);
                }
            }

        });
    })
}

static CUBE_VERTICES: LazyLock<Vec<Vertex, ctru::linear::LinearAllocator>> = LazyLock::new(|| {
    let mut vertex_list = Vec::new_in(ctru::linear::LinearAllocator);
    vertex_list.extend_from_slice(VERTICES);
    vertex_list
});

fn render_cube(transform: &Transform, instance: &mut citro3d::Instance, attr_info: &attrib::Info, model_uniform_idx: uniform::Index) {
    println!("render cube: {transform:?}");

    instance.bind_vertex_uniform(model_uniform_idx, mat4_to_matrix4(transform.compute_matrix()));

    let mut buf_info = citro3d::buffer::Info::new(citro3d::buffer::Primitive::Triangles);
    let buffer_idx = buf_info.add(&CUBE_VERTICES, attr_info).unwrap();
    let vbo_data = buf_info.buffer(buffer_idx).unwrap();

    instance.draw(vbo_data);
}

fn temp_transform_to_matrix4(transform: &Transform) -> Matrix4 {
    let mut mtx = Matrix4::identity();

    mtx.scale(transform.scale.x, transform.scale.y, transform.scale.z);

    let (x, y, z) = transform.rotation.to_euler(EulerRot::XYZ);
    mtx.rotate_y(y);
    // mtx.rotate_x(x);
    // mtx.rotate_z(z);

    mtx.translate(transform.translation.x, transform.translation.y, transform.translation.z);

    mtx
}

fn mat4_to_matrix4(mat: Mat4) -> Matrix4 {
    // FIXME: not sure if this is correct

    // converts column-major xyzw to row-major wzyx
    let cols = mat.to_cols_array();
    Matrix4::from_cells_wzyx([
        cols[12], cols[8],  cols[4], cols[0],
        cols[13], cols[9],  cols[5], cols[1],
        cols[14], cols[10], cols[6], cols[2],
        cols[15], cols[11], cols[7], cols[3],
    ])
}

struct Scene {
    program: shader::Program,
    attr_info: attrib::Info,
    light_env: Pin<Box<LightEnv>>,
    texenv_stages: Vec<texenv::TexEnv>,
}


fn init_scene(world: &mut World) {
    let shader = shader::Library::from_bytes(SHADER_BYTES).unwrap();
    let program = shader::Program::new(shader, 0).unwrap();

    let mut attr_info = attrib::Info::new();

    let reg0 = attrib::Register::new(0).unwrap();
    let reg1 = attrib::Register::new(1).unwrap();
    let reg2 = attrib::Register::new(2).unwrap();

    attr_info.add_loader(reg0, attrib::Format::Float, 3).unwrap();
    attr_info.add_loader(reg1, attrib::Format::Float, 2).unwrap();
    attr_info.add_loader(reg2, attrib::Format::Float, 3).unwrap();

    let mut light_env = Box::pin(LightEnv::new());
    light_env.as_mut().connect_lut(
        LightLutId::D0,
        LutInput::LightNormal,
        LightLut::from_fn(|v| v.powf(30.0), false),
    );

    light_env.as_mut().set_material(Material {
        ambient: Some(citro3d::material::Color::new(0.2, 0.2, 0.2)),
        diffuse: Some(citro3d::material::Color::new(0.4, 0.4, 0.4)),
        specular0: Some(citro3d::material::Color::new(0.8, 0.8, 0.8)),
        ..Default::default()
    });

    let light = light_env.as_mut().create_light().unwrap();
    let mut light = light_env.as_mut().light_mut(light).unwrap();
    light.as_mut().set_color(1.0, 1.0, 1.0);
    light.as_mut().set_position(FVec3::new(6.0, 0.5, 0.0));

    let stage0 = texenv::TexEnv::new()
        .sources(
            texenv::Mode::BOTH,
            texenv::Source::FragmentPrimaryColor,
            Some(texenv::Source::FragmentSecondaryColor),
            None,
        )
        .func(texenv::Mode::BOTH, texenv::CombineFunc::Add);

    world.insert_non_send_resource(Scene {
        program,
        attr_info,
        light_env,
        texenv_stages: vec![stage0],
    });
}

fn get_uniform_indexes(program: &shader::Program) -> [uniform::Index; 3] {
    [
        program.get_uniform("projection").unwrap(),
        program.get_uniform("model").unwrap(),
        program.get_uniform("view").unwrap(),
    ]
}


fn calculate_projections(vertical_fov: f32, screen_depth: f32) -> (Matrix4, Matrix4) {
    let slider_val = ctru::os::current_3d_slider_state();
    let interocular_distance = slider_val / 3.0;

    let vertical_fov = vertical_fov.to_radians();

    let clip_planes = ClipPlanes {
        near: 0.01,
        far: 100.0,
    };

    let (left, right) = StereoDisplacement::new(interocular_distance, screen_depth);

    let (left_eye, right_eye) =
        Projection::perspective(vertical_fov, AspectRatio::TopScreen, clip_planes)
            .stereo_matrices(left, right);

    (left_eye, right_eye)
}
