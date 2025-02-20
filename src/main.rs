#![feature(allocator_api)]

static SHADER_BYTES: &[u8] = include_shader!("../assets/vshader.v.pica");

const CLEAR_COLOR: u32 = 0x68_B0_D8_FF;

use std::boxed::Box;
use std::pin::Pin;

use citro3d::{
    attrib,
    light::{self, LightEnv, LightLut, LightLutId, LutInput},
    macros::include_shader,
    material::{Color, Material},
    math::{AspectRatio, ClipPlanes, FVec3, Matrix4, Projection, StereoDisplacement},
    render::{self, ClearFlags, Target},
    shader, texenv,
};
use ctru::services::{
    apt::Apt,
    gfx::{Gfx, RawFrameBuffer, Screen, TopScreen3D},
    hid::{Hid, KeyPad},
    soc::Soc,
};

mod entity;
mod vertices;

use entity::{Entity, Transform};
use vertices::*;

fn main() {
    let mut soc = Soc::new().expect("failed to get SOC");
    drop(soc.redirect_to_3dslink(true, true));

    let mut hid = Hid::new().expect("Couldn't obtain HID controller");
    let apt = Apt::new().expect("Couldn't obtain APT controller");

    let mut entities = [
        Entity::new(VERTICES).with_transform(Transform {
            pos_x: 2.0,
            pos_z: -10.0,
            ..Default::default()
        }),
        Entity::new(VERTICES).with_transform(Transform {
            pos_x: -2.0,
            pos_z: -10.0,
            ..Default::default()
        }),
    ];

    let gfx = Gfx::with_formats_shared(
        ctru::services::gspgpu::FramebufferFormat::Rgba8,
        ctru::services::gspgpu::FramebufferFormat::Rgba8,
    )
    .expect("Couldn't obtain GFX controller");

    let mut instance = citro3d::Instance::new().expect("failed to initialize Citro3D");

    let top_screen = TopScreen3D::from(&gfx.top_screen);

    let (mut top_left, mut top_right) = top_screen.split_mut();

    let RawFrameBuffer { width, height, .. } = top_left.raw_framebuffer();
    let mut top_left_target = instance
        .render_target(
            width,
            height,
            top_left,
            Some(render::DepthFormat::Depth24Stencil8),
        )
        .expect("failed to create render target");

    let RawFrameBuffer { width, height, .. } = top_right.raw_framebuffer();
    let mut top_right_target = instance
        .render_target(
            width,
            height,
            top_right,
            Some(render::DepthFormat::Depth24Stencil8),
        )
        .expect("failed to create render target");

    let scene = init_scene();

    let projection_idx = scene.program.get_uniform("projection").unwrap();
    let model_uniform_idx = scene.program.get_uniform("model").unwrap();
    let view_idx = scene.program.get_uniform("view").unwrap();

    let mut is_scale = false;
    let mut delta_bumper = 0;

    let mut touch_pos = (0, 0);
    let mut delta_touch = (0, 0);

    let mut selected_entity = 0;
    while apt.main_loop() {
        hid.scan_input();

        if hid.keys_down().contains(KeyPad::START) {
            break;
        }
        if hid.keys_down().contains(KeyPad::A) {
            is_scale = !is_scale;
        }

        if hid.keys_down().contains(KeyPad::TOUCH) {
            touch_pos = hid.touch_position();
        } else if hid.keys_held().contains(KeyPad::TOUCH) {
            let old_pos = touch_pos;
            touch_pos = hid.touch_position();
            delta_touch.0 = touch_pos.0 as i16 - old_pos.0 as i16;
            delta_touch.1 = touch_pos.1 as i16 - old_pos.1 as i16;
        }

        if hid.keys_held().contains(KeyPad::L) {
            delta_bumper -= 1;
        }
        if hid.keys_held().contains(KeyPad::R) {
            delta_bumper += 1;
        }

        if hid.keys_down().contains(KeyPad::DPAD_RIGHT) {
            selected_entity = (selected_entity + 1) % entities.len();
            println!("Selected entity {}", selected_entity);
        }
        if hid.keys_down().contains(KeyPad::DPAD_LEFT) {
            if selected_entity == 0 {
                selected_entity = entities.len() - 1;
            } else {
                selected_entity -= 1;
            }
            println!("Selected entity {}", selected_entity);
        }

        if hid.keys_held().contains(KeyPad::B) {
            entities[selected_entity].toggle_render();
        }

        entities[selected_entity]
            .update_transform((delta_touch.0, delta_touch.1, delta_bumper), is_scale);
        delta_touch = (0, 0);
        delta_bumper = 0;

        let fov = 40.0;
        let screen_depth = 2.0;
        let (left_eye, right_eye) = calculate_projections(fov, screen_depth);

        let targets = [
            (&mut top_left_target, left_eye),
            (&mut top_right_target, right_eye),
        ];

        instance.render_frame_with(|frame| {
            for (target, projection) in targets {
                target.clear(ClearFlags::ALL, CLEAR_COLOR, 0);
                for entity in entities.iter_mut() {
                    entity.render(
                        frame,
                        target,
                        &scene,
                        [
                            (projection_idx, projection),
                            (view_idx, Matrix4::identity()),
                        ],
                        model_uniform_idx,
                    )
                }
            }
        });
    }
}

struct Scene {
    program: shader::Program,
    attr_info: attrib::Info,
    light_env: Pin<Box<LightEnv>>,
    texenv_stages: Vec<texenv::TexEnv>,
}

fn init_scene() -> Scene {
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

    Scene {
        program,
        attr_info,
        light_env,
        texenv_stages: vec![stage0],
    }
}

fn calculate_projections(vertical_fov: f32, screen_depth: f32) -> (Matrix4, Matrix4) {
    let slider_val = ctru::os::current_3d_slider_state();
    let interocular_distance = slider_val / 3.0;

    let vertical_fov = vertical_fov.to_radians();

    let clip_planes = ClipPlanes {
        near: 0.01,
        far: 1000.0,
    };

    let (left, right) = StereoDisplacement::new(interocular_distance, screen_depth);

    let (left_eye, right_eye) =
        Projection::perspective(vertical_fov, AspectRatio::TopScreen, clip_planes)
            .stereo_matrices(left, right);

    (left_eye, right_eye)
}
