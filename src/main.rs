#![feature(allocator_api)]

static SHADER_BYTES: &[u8] = include_shader!("../assets/vshader.v.pica");

const CLEAR_COLOR: u32 = 0x68_B0_D8_FF;

use std::boxed::Box;
use std::ops::ControlFlow;
use std::pin::Pin;
use std::f32::consts::PI;

use citro3d::{
    attrib,
    light::{self, LightEnv, LightLut, LightLutId, LutInput},
    macros::include_shader,
    material::{Color, Material},
    math::{AspectRatio, ClipPlanes, FVec3, Matrix4, Projection, StereoDisplacement},
    render::{self, ClearFlags, ScreenTarget, Target},
    shader, texenv, uniform,
};
use ctru::services::{
    apt::Apt,
    gfx::{Gfx, RawFrameBuffer, Screen, TopScreen3D},
    hid::{Hid, KeyPad},
    soc::Soc,
};

mod entity;
mod player;
mod vertices;

use entity::{Entity, Transform};
use vertices::*;
use player::Player;

struct App {
    is_scale: bool,
    delta_bumper: i16,
    touch_pos: (u16, u16),
    delta_touch: (i16, i16),

    player: Player,
    entities: Vec<Entity>,
    selected_entity: usize,
}

impl App {
    fn new(player: Player, entities: impl IntoIterator<Item = Entity>) -> Self {
        Self {
            is_scale: false,
            delta_bumper: 0,
            touch_pos: (0, 0),
            delta_touch: (0, 0),

            player,
            entities: entities.into_iter().collect(),
            selected_entity: 0,
        }
    }

    fn update(&mut self, hid: &Hid) -> ControlFlow<()> {
        if hid.keys_down().contains(KeyPad::START) {
            return ControlFlow::Break(());
        }
        if hid.keys_down().contains(KeyPad::A) {
            self.is_scale = !self.is_scale;
        }

        if hid.keys_down().contains(KeyPad::TOUCH) {
            self.touch_pos = hid.touch_position();
        } else if hid.keys_held().contains(KeyPad::TOUCH) {
            let old_pos = self.touch_pos;
            self.touch_pos = hid.touch_position();
            self.delta_touch.0 = self.touch_pos.0 as i16 - old_pos.0 as i16;
            self.delta_touch.1 = self.touch_pos.1 as i16 - old_pos.1 as i16;
        }

        if hid.keys_held().contains(KeyPad::L) {
            self.delta_bumper -= 1;
        }
        if hid.keys_held().contains(KeyPad::R) {
            self.delta_bumper += 1;
        }

        if hid.keys_down().contains(KeyPad::DPAD_RIGHT) {
            self.selected_entity = (self.selected_entity + 1) % self.entities.len();
        }
        if hid.keys_down().contains(KeyPad::DPAD_LEFT) {
            if self.selected_entity == 0 {
                self.selected_entity = self.entities.len() - 1;
            } else {
                self.selected_entity -= 1;
            }
        }

        if hid.keys_held().contains(KeyPad::B) {
            self.entities[self.selected_entity].toggle_render();
        }

        self.entities[self.selected_entity].update_transform(
            (self.delta_touch.0, self.delta_touch.1, self.delta_bumper),
            self.is_scale,
        );

        self.player.update(hid);

        self.delta_touch = (0, 0);
        self.delta_bumper = 0;


        ControlFlow::Continue(())
    }

    fn render<'a>(
        &mut self,
        instance: &mut citro3d::Instance,
        targets: (&mut ScreenTarget<'a>, &mut ScreenTarget<'a>),
        scene: &Scene,
    ) {
        let fov = 40.0;
        let screen_depth = 2.0;

        let (left_eye, right_eye) = calculate_projections(fov, screen_depth);
        let view = self.player.calculate_view();

        let [projection_idx, model_idx, view_idx] = get_uniform_indexes(&scene.program);

        let targets = [(targets.0, left_eye), (targets.1, right_eye)];
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

                for entity in self.entities.iter_mut() {
                    entity.render(&mut frame.instance, &scene.attr_info, model_idx);
                }
            }
        });
    }
}

fn main() {
    let (mut hid, apt, gfx) = init_services();

    let n = 10;
    let entities = (0..n).map(|i: u32| {
        let angle = i as f32 * (2.0 * PI) / (n as f32);
        Entity::new(VERTICES).with_transform(Transform {
            pos_x: angle.cos() * 5.0,
            pos_z: angle.sin() * 5.0,
            ..Default::default()
        })
    });

    let player = Player::new(0.2, 4.0);

    let mut app = App::new(player, entities);

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

    while apt.main_loop() {
        hid.scan_input();

        match app.update(&hid) {
            ControlFlow::Break(_) => break,
            ControlFlow::Continue(_) => {}
        }

        app.render(
            &mut instance,
            (&mut top_left_target, &mut top_right_target),
            &scene,
        );
    }
}

fn init_services() -> (Hid, Apt, Gfx) {
    let mut soc = Soc::new().expect("failed to get SOC");
    drop(soc.redirect_to_3dslink(true, true));

    let hid = Hid::new().expect("Couldn't obtain HID controller");
    let apt = Apt::new().expect("Couldn't obtain APT controller");

    let gfx = Gfx::with_formats_shared(
        ctru::services::gspgpu::FramebufferFormat::Rgba8,
        ctru::services::gspgpu::FramebufferFormat::Rgba8,
    )
    .expect("Couldn't obtain GFX controller");

    (hid, apt, gfx)
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
        far: 1000.0,
    };

    let (left, right) = StereoDisplacement::new(interocular_distance, screen_depth);

    let (left_eye, right_eye) =
        Projection::perspective(vertical_fov, AspectRatio::TopScreen, clip_planes)
            .stereo_matrices(left, right);

    (left_eye, right_eye)
}
