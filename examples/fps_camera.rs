#![feature(allocator_api)]

use std::boxed::Box;
use std::f32::consts::FRAC_PI_2;
use std::pin::Pin;
use std::f32::consts::PI;
use std::mem::ManuallyDrop;

use ouroboros::self_referencing;

use citro3d::{
    attrib,
    light::{self, LightEnv, LightLut, LightLutId, LutInput},
    macros::include_shader,
    material::{Color, Material},
    math::{AspectRatio, ClipPlanes, FVec3, Matrix4, Projection, StereoDisplacement},
    render::{self, ClearFlags, Target, ScreenTarget},
    shader, texenv, uniform,
};
use ctru::services::{
    gfx::{Gfx, RawFrameBuffer, Screen, TopScreen3D},
};



use bevy::{log, prelude::*};
use ctru::applets::error;
use bevy::input::ButtonInput;
use bevy::input::gamepad::{GamepadButton, GamepadButtonType};

use bevy_3ds::vertices::{self, VERTICES};


static SHADER_BYTES: &[u8] = include_shader!("../assets/vshader.v.pica");
const CLEAR_COLOR: u32 = 0x68_B0_D8_FF;

fn main() {
    error::set_panic_hook(false);

    App::new()
        .add_plugins((bevy_3ds::DefaultPlugins
                          .set(log::LogPlugin {
                              level: log::Level::DEBUG,
                              ..default()
                          }),))
        .insert_non_send_resource(citro3d::Instance::new().expect("failed to initialize Citro3D"))
        .add_systems(Startup, (create_targets, init_scene, spawn_player, spawn_cubes))
        .add_systems(Update, (exit_on_start, move_player))
        .add_systems(PostUpdate, render)
        .run();
}

#[derive(Component)]
struct Player {
    move_speed: f32,
    camera_sensitivity: f32,
    fov: f32,
}

#[derive(Component)]
struct Cube {
    vertex_list: Vec<vertices::Vertex, ctru::linear::LinearAllocator>,
}

fn spawn_player(mut commands: Commands) {
    commands
        .spawn((
            Player {
                move_speed: 4.0,
                camera_sensitivity: 0.2,
                fov: 45.0,
            },
            Transform::default(),
        ));
}

fn spawn_cubes(mut commands: Commands) {
    let n = 10;
    for i in 0..n {
        let mut vertex_list = Vec::with_capacity_in(VERTICES.len(), ctru::linear::LinearAllocator);
        vertex_list.extend_from_slice(VERTICES);

        let angle = i as f32 * (2.0 * PI) / (n as f32);
        commands.spawn((
            Transform::from_xyz(angle.cos() * 5.0, 0.0, angle.sin() * 5.0),
            Cube { vertex_list },
        ));
    }
}

fn move_player(mut player: Query<(&mut Transform, &Player)>, axes: Res<Axis<GamepadAxis>>, time: Res<Time>) {
    let Ok((mut transform, player)) = player.get_single_mut() else { return };

    let gamepad = Gamepad { id: 0 };

    let mut movement = Vec3::ZERO;
    let mut yaw = 0.0;
    let mut pitch = 0.0;

    if let Some(lx) = axes.get(GamepadAxis::new(gamepad, GamepadAxisType::LeftStickX)) {
        movement.x = lx * player.move_speed * time.delta_seconds();
    }
    if let Some(ly) = axes.get(GamepadAxis::new(gamepad, GamepadAxisType::LeftStickY)) {
        movement.z = ly * player.move_speed * time.delta_seconds();
    }
    if let Some(rx) = axes.get(GamepadAxis::new(gamepad, GamepadAxisType::RightStickX)) {
        yaw = rx * player.camera_sensitivity * time.delta_seconds();
    }
    if let Some(ry) = axes.get(GamepadAxis::new(gamepad, GamepadAxisType::RightStickY)) {
        pitch = (ry * player.camera_sensitivity * time.delta_seconds()).clamp(-FRAC_PI_2, FRAC_PI_2);
    }

    let right = transform.right();
    let forward = transform.forward();
    transform.translation += movement.x * right + movement.z * forward;
    transform.rotate_y(-yaw);
    transform.rotate_x(-pitch);
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

fn create_targets(world: &mut World) {
    let gfx = world.remove_non_send_resource::<Gfx>().expect("`create_targets` requires `Gfx` resource");
    let instance = world.get_non_send_resource::<citro3d::Instance>().unwrap();

    let targets = Targets(TargetsInnerBuilder {
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
    }.build());
    world.insert_non_send_resource(targets);
}

fn render(mut instance: NonSendMut<citro3d::Instance>, mut screen_targets: NonSendMut<Targets>, scene: NonSend<Scene>, player: Query<(&Transform, &Player)>, cubes: Query<(&Transform, &Cube)>) {
    let Ok((player_transform, player)) = player.get_single() else { return };

    let screen_depth = 2.0;
    let (left_eye, right_eye) = calculate_projections(player.fov, screen_depth);
    let view = mat4_to_matrix4(player_transform.compute_matrix().inverse());

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
                    let TEXENV_COUNT: usize = 6;
                    for i in 0..TEXENV_COUNT {
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

                for (transform, cube) in cubes.iter() {
                    render_cube(transform, &cube.vertex_list, frame.instance, &scene.attr_info, model_idx);
                }
            }

        });
    })
}

fn render_cube(transform: &Transform, vertex_list: &Vec<vertices::Vertex, ctru::linear::LinearAllocator>, instance: &mut citro3d::Instance, attr_info: &attrib::Info, model_uniform_idx: uniform::Index) {
    instance.bind_vertex_uniform(model_uniform_idx, mat4_to_matrix4(transform.compute_matrix()));

    let mut buf_info = citro3d::buffer::Info::new(citro3d::buffer::Primitive::Triangles);
    let buffer_idx = buf_info.add(vertex_list, attr_info).unwrap();
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

    let mut light_env = Box::pin(light::LightEnv::new());
    light_env.as_mut().connect_lut(
        LightLutId::D0,
        LutInput::LightNormal,
        LightLut::from_fn(|v| v.powf(30.0), false),
    );

    light_env.as_mut().set_material(Material {
        ambient: Some(Color::new(0.2, 0.2, 0.2)),
        diffuse: Some(Color::new(0.4, 0.4, 0.4)),
        specular0: Some(Color::new(0.8, 0.8, 0.8)),
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

fn exit_on_start(
    button_inputs: Res<ButtonInput<GamepadButton>>,
    mut exit_events: EventWriter<AppExit>,
) {
    if button_inputs.just_pressed(GamepadButton::new(
        bevy_3ds::input::GAMEPAD,
        GamepadButtonType::Start,
    )) {
        exit_events.send(AppExit::Success);
    }
}
