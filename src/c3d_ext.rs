// use std::mem::MaybeUninit;
// use citro3d::math::FVec3;
//
// #[doc(alias = "C3D_Material")]
// // TODO: remove pub
// pub struct Material(pub citro3d_sys::C3D_Material);
//
//
// impl Material {
//     #[doc(alias = "C3D_MaterialInit")]
//     pub const fn new(ambient: [f32; 3], diffuse: [f32; 3], specular0: [f32; 3], specular1: [f32; 3], emission: [f32; 3]) -> Self {
//         Self(citro3d_sys::C3D_Material {
//             ambient,
//             diffuse,
//             specular0,
//             specular1,
//             emission,
//         })
//     }
// }
//
// #[doc(alias = "C3D_LightEnv")]
// // TODO: remove pub
// pub struct LightEnv(pub citro3d_sys::C3D_LightEnv);
//
// impl LightEnv {
//     #[doc(alias = "C3D_LightEnvInit")]
//     pub fn new() -> Self {
//         let light_env = unsafe {
//             let mut light_env = MaybeUninit::uninit();
//             citro3d_sys::C3D_LightEnvInit(light_env.as_mut_ptr());
//
//             light_env.assume_init()
//         };
//
//         Self(light_env)
//     }
//
//     #[doc(alias = "C3D_LightEnvMaterial")]
//     pub fn with_material(mut material: Material) -> Self {
//         let mut light_env = Self::new();
//
//         unsafe {
//             citro3d_sys::C3D_LightEnvMaterial(&mut light_env.0, &mut material.0);
//         }
//
//         light_env
//     }
//
//     /// Set the light look-up table with the given ID and input
//     pub fn set_lut(&mut self, lut: &mut LightLut, id: LutId, input: LutInput, negative: bool) {
//         unsafe {
//             citro3d_sys::C3D_LightEnvLut(&mut self.0, id as u8, input as u8, negative, &mut lut.0);
//         }
//     }
//
//     #[doc(alias = "C3D_LightInit")]
//     pub fn light(&mut self) -> Light {
//         let mut light = MaybeUninit::uninit();
//         unsafe {
//             citro3d_sys::C3D_LightInit(light.as_mut_ptr(), &mut self.0);
//             Light(light.assume_init())
//         }
//     }
// }
//
// #[derive(Clone, Copy)]
// #[doc(alias = "GPU_LIGHTLUTID")]
// #[repr(u8)]
// pub enum LutId {
//     D0 = ctru_sys::GPU_LUT_D0,
//     D1 = ctru_sys::GPU_LUT_D1,
//     Spotlight = ctru_sys::GPU_LUT_SP,
//     Fresnel = ctru_sys::GPU_LUT_FR,
//     ReflectionBlue = ctru_sys::GPU_LUT_RB,
//     ReflectionGreen = ctru_sys::GPU_LUT_RG,
//     ReflectionRed = ctru_sys::GPU_LUT_RR,
//     DistanceAttenuation = ctru_sys::GPU_LUT_DA,
// }
//
// #[derive(Clone, Copy)]
// #[doc(alias = "GPU_LIGHTLUTINPUT")]
// #[repr(u8)]
// pub enum LutInput {
//     /// Normal * Half Vector
//     NormalHalf = ctru_sys::GPU_LUTINPUT_NH,
//     /// View * Half Vector
//     ViewHalf = ctru_sys::GPU_LUTINPUT_VH,
//     /// Normal * View
//     NormalView = ctru_sys::GPU_LUTINPUT_NV,
//     /// Light Vector * Normal
//     LightNormal = ctru_sys::GPU_LUTINPUT_LN,
//     /// -(Light Vector) * Spotlight Vector
//     Spotlight = ctru_sys::GPU_LUTINPUT_SP,
//     /// Cosine of phi
//     CosPhi = ctru_sys::GPU_LUTINPUT_CP,
// }
//
//
// pub trait InstanceExt {
//     #[doc(alias = "C3D_LightEnvBind")]
//     fn bind_light_env(&mut self, light_env: &mut LightEnv) {
//         unsafe {
//             citro3d_sys::C3D_LightEnvBind(&mut light_env.0);
//         }
//     }
// }
//
// impl InstanceExt for citro3d::Instance {}
//
// #[doc(alias = "C3D_LightLut")]
// pub struct LightLut(citro3d_sys::C3D_LightLut);
//
// impl LightLut {
//     #[doc(alias = "LightLut_FromArray")]
//     pub fn from_array(mut data: [f32; 512]) -> Self {
//         let mut lut = MaybeUninit::uninit();
//
//         let lut = unsafe {
//             citro3d_sys::LightLut_FromArray(lut.as_mut_ptr(), data.as_mut_ptr());
//             lut.assume_init()
//         };
//
//         Self(lut)
//     }
//
//     #[doc(alias = "LightLut_Phong")]
//     pub fn phong(shininess: f32) -> Self {
//         let mut lut_phong = MaybeUninit::uninit();
//
//         extern "C" fn powf(a: f32, b: f32) -> f32 {
//             a.powf(b)
//         }
//         let lut_phong = unsafe {
//             citro3d_sys::LightLut_FromFunc(lut_phong.as_mut_ptr(), Some(powf), shininess, false);
//             lut_phong.assume_init()
//         };
//
//         Self(lut_phong)
//     }
//
//     #[doc(alias = "LightLut_Spotlight")]
//     pub fn spotlight(angle: f32) -> Self {
//         let mut lut_spotlight = MaybeUninit::uninit();
//
//         extern "C" fn spot_step(angle: f32, cutoff: f32) -> f32 {
//             if angle >= cutoff {
//                 1.0
//             } else {
//                 0.0
//             }
//         }
//
//         let lut_spotlight = unsafe {
//             citro3d_sys::LightLut_FromFunc(lut_spotlight.as_mut_ptr(), Some(spot_step), angle.cos(), true);
//             lut_spotlight.assume_init()
//         };
//
//         Self(lut_spotlight)
//     }
// }
//
// #[doc(alias = "C3D_Light")]
// pub struct Light(citro3d_sys::C3D_Light);
//
// impl Light {
//     #[doc(alias = "C3D_LightColor")]
//     pub fn color(&mut self, r: f32, g: f32, b: f32) -> &mut Self {
//         unsafe {
//             citro3d_sys::C3D_LightColor(&mut self.0, r, g, b);
//         }
//         self
//     }
//
//     #[doc(alias = "C3D_LightPosition")]
//     pub fn position(&mut self, pos: FVec3) -> &mut Self {
//         unsafe {
//             // FIXME: get the internal field
//             let mut internal = citro3d_sys::FVec3_New(pos.x(), pos.y(), pos.z());
//             citro3d_sys::C3D_LightPosition(&mut self.0, &mut internal);
//         }
//         self
//     }
// }
//
//
//
//
// // #[derive(Copy, Clone)]
// // pub enum Operand {
// //     SourceColor,
// //     OneMinusSourceColor,
// //     SourceAlpha,
// //     OneMinusSourceAlpha,
// //     SourceRed,
// //     OneMinusSourceRed,
// //     SourceGreen,
// //     OneMinusSourceGreen,
// //     SourceBlue,
// //     OneMinusSourceBlue,
// // }
// //
// // impl Operand {
// //     fn value(self, mode: Mode) {
// //         // ```c
// //         // /// Texture RGB combiner operands.
// //         // typedef enum
// //         // {
// //         // 	GPU_TEVOP_RGB_SRC_COLOR           = 0x00, ///< Source color.
// //         // 	GPU_TEVOP_RGB_ONE_MINUS_SRC_COLOR = 0x01, ///< Source color - 1.
// //         // 	GPU_TEVOP_RGB_SRC_ALPHA           = 0x02, ///< Source alpha.
// //         // 	GPU_TEVOP_RGB_ONE_MINUS_SRC_ALPHA = 0x03, ///< Source alpha - 1.
// //         // 	GPU_TEVOP_RGB_SRC_R               = 0x04, ///< Source red.
// //         // 	GPU_TEVOP_RGB_ONE_MINUS_SRC_R     = 0x05, ///< Source red - 1.
// //         // 	GPU_TEVOP_RGB_0x06                = 0x06, ///< Unknown.
// //         // 	GPU_TEVOP_RGB_0x07                = 0x07, ///< Unknown.
// //         // 	GPU_TEVOP_RGB_SRC_G               = 0x08, ///< Source green.
// //         // 	GPU_TEVOP_RGB_ONE_MINUS_SRC_G     = 0x09, ///< Source green - 1.
// //         // 	GPU_TEVOP_RGB_0x0A                = 0x0A, ///< Unknown.
// //         // 	GPU_TEVOP_RGB_0x0B                = 0x0B, ///< Unknown.
// //         // 	GPU_TEVOP_RGB_SRC_B               = 0x0C, ///< Source blue.
// //         // 	GPU_TEVOP_RGB_ONE_MINUS_SRC_B     = 0x0D, ///< Source blue - 1.
// //         // 	GPU_TEVOP_RGB_0x0E                = 0x0E, ///< Unknown.
// //         // 	GPU_TEVOP_RGB_0x0F                = 0x0F, ///< Unknown.
// //         // } GPU_TEVOP_RGB;
// //         //
// //         // /// Texture Alpha combiner operands.
// //         // typedef enum
// //         // {
// //         // 	GPU_TEVOP_A_SRC_ALPHA           = 0x00, ///< Source alpha.
// //         // 	GPU_TEVOP_A_ONE_MINUS_SRC_ALPHA = 0x01, ///< Source alpha - 1.
// //         // 	GPU_TEVOP_A_SRC_R               = 0x02, ///< Source red.
// //         // 	GPU_TEVOP_A_ONE_MINUS_SRC_R     = 0x03, ///< Source red - 1.
// //         // 	GPU_TEVOP_A_SRC_G               = 0x04, ///< Source green.
// //         // 	GPU_TEVOP_A_ONE_MINUS_SRC_G     = 0x05, ///< Source green - 1.
// //         // 	GPU_TEVOP_A_SRC_B               = 0x06, ///< Source blue.
// //         // 	GPU_TEVOP_A_ONE_MINUS_SRC_B     = 0x07, ///< Source blue - 1.
// //         // } GPU_TEVOP_A;
// //         // ```
// //         //
// //         // Mode is either Mode::RGB or Mode::ALPHA
// //         match (mode, self) {
// //             (Mode::RGB, Operand::SourceColor) => ctru_sys::GPU_TEVOP_RGB_SRC_COLOR,
// //             (Mode::RGB, Operand::OneMinusSourceColor) => ctru_sys::GPU_TEVOP_RGB_ONE_MINUS_SRC_COLOR,
// //             (Mode::RGB, Operand::SourceAlpha) => ctru_sys::GPU_TEVOP_RGB_SRC_ALPHA,
// //             (Mode::RGB, Operand::OneMinusSourceAlpha) => ctru_sys::GPU_TEVOP_RGB_ONE_MINUS_SRC_ALPHA,
// //             (Mode::RGB, Operand::SourceRed) => ctru_sys::GPU_TEVOP_RGB_SRC_R,
// //             (Mode::RGB, Operand::OneMinusSourceRed) => ctru_sys::GPU_TEVOP_RGB_ONE_MINUS_SRC_R,
// //             (Mode::RGB, Operand::SourceGreen) => ctru_sys::GPU_TEVOP_RGB_SRC_G,
// //             (Mode::RGB, Operand::OneMinusSourceGreen) => ctru_sys::GPU_TEVOP_RGB_ONE_MINUS_SRC_G,
// //             (Mode::RGB, Operand::SourceBlue) => ctru_sys::GPU_TEVOP_RGB_SRC_B,
// //             (Mode::ALPHA, Operand::SourceAlpha) => ctru_sys::GPU_TEVOP_A_SRC_ALPHA,
// //             (Mode::ALPHA, Operand::OneMinusSourceAlpha) => ctru_sys::GPU_TEVOP_A_ONE_MINUS_SRC_ALPHA,
// //             (Mode::ALPHA, Operand::SourceRed) => ctru_sys::GPU_TEVOP_A_SRC_R,
// //             (Mode::ALPHA, Operand::OneMinusSourceRed) => ctru_sys::GPU_TEVOP_A_ONE_MINUS_SRC_R,
// //             (Mode::ALPHA, Operand::SourceGreen) => ctru_sys::GPU_TEVOP_A_SRC_G,
// //             (Mode::ALPHA, Operand::OneMinusSourceGreen) => ctru_sys::GPU_TEVOP_A_ONE_MINUS_SRC_G,
// //             (Mode::ALPHA, Operand::SourceBlue) => ctru_sys::GPU_TEVOP_A_SRC_B,
// //             (Mode::ALPHA, Operand::OneMinusSourceBlue) => ctru_sys::GPU_TEVOP_A_ONE_MINUS_SRC_B,
// //             _ => panic!("Invalid operand for mode"),
// //         }
// //     }
// // }
// //
// // pub trait TexEnvExt {
// //     /// Configure the source values of the texture combiner.
// //     ///
// //     /// # Parameters
// //     ///
// //     /// - `mode`: which [`Mode`]\(s) to set the operand(s) for.
// //     /// - `source0`: the first [`Source`] operand to the texture combiner
// //     /// - `source1` and `source2`: optional additional [`Source`] operands to use
// //     fn op(&mut self, mode: Mode, op0: Operand, op1: Option<Operand>, op2: Option<Operand>) {
// //
// //     }
// // }