#![allow(clippy::unnecessary_cast)]
#![allow(unused_imports)]
#![allow(dead_code)]
// #![feature(const_trait_impl)]
// #![feature(type_alias_impl_trait)]
#![allow(unused_mut)]
#![allow(non_camel_case_types)]
#![feature(variant_count)]
#![feature(strict_overflow_ops)]
#![feature(iter_intersperse)]
#![feature(trivial_bounds)]
#![feature(impl_trait_in_assoc_type)]
#![feature(const_closures)]
// #![feature(impl_trait_existential_types)]
// #![feature(option_get_or_insert_default)]
#![feature(let_chains)]
// #![feature(const_mut_refs)]
// #![feature(int_roundings)]
// #![recursion_limit = "1024"]
// #![feature(const_fn_floating_point_arithmetic)]

// pub mod bundletree;
// pub mod ui;
// mod dialogue;
// mod mycommand;

// use crate::dialogue::DialogueTree

use bevy::ecs::query::{ReadOnlyQueryData, WorldQuery};

use {core::hash,
     std::{cell::LazyCell, sync::LazyLock}};

use {bevy::{app::AppExit,
            asset::{AssetServer, Handle},
            core_pipeline::bloom::{Bloom, BloomCompositeMode, BloomPrefilter},
            ecs::{entity::EntityHashMap,
                  observer::Observers,
                  query::{QueryData, QueryFilter},
                  system::SystemParam},
            image::ImageAddressMode,
            input::keyboard::KeyboardInput,
            math::primitives,
            pbr::{DistanceFog, NotShadowCaster, NotShadowReceiver, StandardMaterial},
            prelude::{Name, *},
            render::view::screenshot::{save_to_disk, Screenshot},
            utils::{HashMap, HashSet},
            window::WindowMode},
     bevy_embedded_assets::*,
     // bevy_panorbit_camera::PanOrbitCamera,
     // bevy_quill::{prelude::*, QuillPlugin, ViewChild},
     // bevy_quill_overlays::QuillOverlaysPlugin,
     bevy_voxel_world::prelude::*,
     enum_assoc::Assoc,
     fancy_constructor::new,
     noise::Perlin,
     rand::{random, thread_rng},
     rust_utils::*,
     std::{cmp::Ordering,
           f32::consts::{PI, TAU},
           mem::variant_count,
           ops::DerefMut,
           sync::Arc}};
use {bevy::{core_pipeline::{tonemapping::{DebandDither, Tonemapping},
                            Skybox},
            render::{camera::Exposure, view::ColorGrading}},
     noise::NoiseFn};

// ui::UIData

const ROTATIONS: fn() -> [Quat; 5] = || {
  [Quat::IDENTITY,
   Quat::from_rotation_z(-PI),
   Quat::from_rotation_z(PI),
   Quat::from_rotation_x(PI),
   Quat::from_rotation_x(-PI)]
};
const ROTATIONS2: LazyLock<[Quat; 5]> = LazyLock::new(|| {
  [Quat::IDENTITY,
   Quat::from_rotation_z(-PI),
   Quat::from_rotation_z(PI),
   Quat::from_rotation_x(PI),
   Quat::from_rotation_x(-PI)]
});
fn rotations() -> [Quat; 5] {
  [Quat::IDENTITY,
   Quat::from_rotation_z(-PI),
   Quat::from_rotation_z(PI),
   Quat::from_rotation_x(PI),
   Quat::from_rotation_x(-PI)]
}
mod mycolor {
  pub use bevy::color::palettes::tailwind::*;
  use bevy::color::{palettes::tailwind::*, Color};
  // {
  //   // bevy::color::prelude::
  // }
  pub const CLEAR: Color = Color::hsv(301.0, 1.0, 0.5);

  pub const GLOWY: Color = Color::srgb(13.99, 11.32, 50.0);
  pub const GLOWY_2: Color = Color::srgb(30.0, 20.7, 10.5);
  pub const GLOWY_3: Color = Color::srgb(0.0, 30.0, 0.0);
  pub const EXPLOSION: Color = Color::srgb(8.0, 3.0, 3.0);
  pub const LASER: Color = Color::hsv(60.0, 1.0, 4.0);
  // hsv(61, 100%, 100%)
}
pub const BILLBOARD_REL_SCALE: f32 = 2.0;
pub const TEXT_SCALE: f32 = 0.013;
pub const ENABLE_SHADOWS_OTHER_THAN_SUN: bool = false;

enum Shape {
  Circle(f32),
  Rectangle(f32, f32)
}

impl Shape {
  fn area(&self) -> f32 {
    use Shape::*;
    match self {
      Circle(r) => PI * r * r,
      Rectangle(w, h) => w * h
    }
  }
}

#[derive(Copy, Clone, Hash, Eq, PartialEq)]
struct MySprite {
  path: &'static str
}
impl MySprite {
  const fn new(path: &'static str) -> Self { Self { path } }
  const ASTEROID: Self = Self::new("asteroid.png");
  const CAR: Self = Self::new("car.png");
  const COFFEE: Self = Self::new("coffee.png");
  const COIN: Self = Self::new("coin.png");
  const CONTAINER: Self = Self::new("container.png");
  const CRYSTAL_ASTEROID: Self = Self::new("crystal_asteroid.png");
  const CRYSTAL_MONSTER: Self = Self::new("crystal_monster.png");
  const EVIL_ROBOT: Self = Self::new("evil_robot.png");
  const EXAMPLE: Self = Self::new("example_voxel_texture.png");
  const FLOATING_ISLAND: Self = Self::new("floating_island.png");
  const GATE: Self = Self::new("gate.png");
  const GRASS: Self = Self::new("grass.png");
  const GROUND: Self = Self::new("ground.png");
  const HPBOX: Self = Self::new("hpbox.png");
  const ICE: Self = Self::new("ice_planet.png");
  const ICEBERG: Self = Self::new("iceberg.png");
  const ICESTEROID: Self = Self::new("icesteroid.png");
  const LAVA: Self = Self::new("lava_planet.png");
  const MUSHROOM: Self = Self::new("mushroom_man.png");
  const NASA: Self = Self::new("nasa_starmap.jpeg");
  const NOTE: Self = Self::new("note.png");
  const PENGUIN: Self = Self::new("penguin.png");
  const PLAYER: Self = Self::new("player.png");
  const PORTAL: Self = Self::new("portal.png");
  const PURPLEENEMYSHIP: Self = Self::new("purpleenemyship.png");
  const SANDPLANET: Self = Self::new("sandplanet.png");
  const SIGN: Self = Self::new("sign.png");
  const SKY: Self = Self::new("sky.jpg");
  const SNOW: Self = Self::new("snow.png");
  const SPACE_CAT: Self = Self::new("space_cat.png");
  const SPACE_STATION: Self = Self::new("space_station.png");
  const SPACECOWBOY: Self = Self::new("spacecowboy.png");
  const SPACEMAN: Self = Self::new("spaceman.png");
  const SPACEPIRATEBASE: Self = Self::new("spacepiratebase.png");
  const SPACESHIPABANDONED: Self = Self::new("spaceshipabandoned.png");
  const SPACESHIPBLUE: Self = Self::new("spaceshipblue.png");
  const SPACESHIPDARKRED: Self = Self::new("spaceshipdarkred.png");
  const SPACESHIPGREEN: Self = Self::new("spaceshipgreen.png");
  const SPACESHIPPURPLE: Self = Self::new("spaceshippurple.png");
  const SPACESHIPRED: Self = Self::new("spaceshipred.png");
  const SPACESHIPWHITE2: Self = Self::new("spaceshipwhite2.png");
  const SPACESHIPWHITE: Self = Self::new("spaceshipwhite.png");
  const SPACEWIZARD: Self = Self::new("spacewizard.png");
  const SPHERICAL: Self = Self::new("spherical_cow.png");
  const STICKMAN: Self = Self::new("stickman.png");
  const STONE: Self = Self::new("stone.png");
  const SUN: Self = Self::new("sun.png");
  const TENT: Self = Self::new("tent.png");
  const TORCH: Self = Self::new("torch.png");
  const TREE: Self = Self::new("tree.png");
  const TREEMONSTER: Self = Self::new("treemonster.png");
  const TURRET: Self = Self::new("turret.png");
  const WATER: Self = Self::new("water.png");
  const WHITE_CORNERS: Self = Self::new("white_corners.png");
  const WIZARDSPACESHIP: Self = Self::new("wizardspaceship.png");
  const WORMHOLE: Self = Self::new("wormhole.png");
  const ZORP: Self = Self::new("zorp.png");

  const BLOCK_TEXTURES: Self = Self::new("pixelc/block_textures.png");
  const BRICKS: Self = Self::new("pixelc/bricks.png");
  const BROWNGASGIANT: Self = Self::new("pixelc/browngasgiant.png");
  const CHEST: Self = Self::new("pixelc/chest.png");
  const FIRE: Self = Self::new("pixelc/fire.png");
  const FURNACE: Self = Self::new("pixelc/furnace.png");
  const GRAYMOON: Self = Self::new("pixelc/graymoon.png");
  const GRAYTRANSPARENTSQUARE: Self = Self::new("pixelc/graytransparentsquare.png");
  const HABITABLEPLANET: Self = Self::new("pixelc/habitableplanet.png");
  const MARSLIKEPLANET: Self = Self::new("pixelc/marslikeplanet.png");
  const MISSILE: Self = Self::new("pixelc/missile.png");
  const SPACEBACKGROUND: Self = Self::new("pixelc/spacebackground.png");
  const YELLOWSTAR: Self = Self::new("pixelc/yellowstar.png");
}
#[derive(Copy, Clone, Hash, Eq, PartialEq)]
struct MyImageMaterial {
  img: MySprite,
  mat_fn: fn(Handle<Image>) -> StandardMaterial
}
impl MyImageMaterial {
  const fn new(mat_fn: fn(Handle<Image>) -> StandardMaterial, img: MySprite) -> Self {
    Self { img, mat_fn }
  }
  pub fn val(&self, h: Handle<Image>) -> StandardMaterial { (self.mat_fn)(h) }
  pub fn img(&self) -> MySprite { self.img }
  const GROUND: Self = Self::new(|h| StandardMaterial { perceptual_roughness: 0.8,
                                                        metallic: 0.0,
                                                        reflectance: 0.2,
                                                        ..h.into() },
                                 MySprite::GROUND);
  const SNOW: Self = Self::new(|h| StandardMaterial { perceptual_roughness: 0.4,
                                                      metallic: 0.0,
                                                      reflectance: 0.5,
                                                      ior: 1.31,
                                                      ..h.into() },
                               MySprite::SNOW);
  const WATER: Self = Self::new(|h| StandardMaterial { perceptual_roughness: 0.3,
                                                       metallic: 0.0,
                                                       reflectance: 0.5,
                                                       ..h.into() },
                                MySprite::WATER);
  const STONE: Self = Self::new(|h| StandardMaterial { perceptual_roughness: 0.8,
                                                       metallic: 0.0,
                                                       reflectance: 0.3,
                                                       ..h.into() },
                                MySprite::STONE);
  // const BRICKS: Self = Self::new(|h| StandardMaterial { perceptual_roughness: 0.95,
  //                                                       metallic: 0.0,
  //                                                       reflectance: 0.1,
  //                                                       ..h.into() },
  //                                MySprite::BRICKS);
  const GRASS: Self = Self::new(|h| StandardMaterial { perceptual_roughness: 0.8,
                                                       metallic: 0.0,
                                                       reflectance: 0.2,
                                                       ..h.into() },
                                MySprite::GRASS);
  // const PENGUIN: Self = Self::new(From::from, MySprite::PENGUIN);
}
#[derive(Copy, Clone, Hash, Eq, PartialEq)]
struct MyMaterial {
  mat_fn: fn() -> StandardMaterial
}
impl MyMaterial {
  const fn new(mat_fn: fn() -> StandardMaterial) -> Self { Self { mat_fn } }
  pub fn val(&self) -> StandardMaterial { (self.mat_fn)() }
  const BLOCKS: Self = Self::new(|| StandardMaterial { perceptual_roughness: 0.8,
                                                       metallic: 0.0,
                                                       reflectance: 0.3,
                                                       ..default() });
  const GLOWY: Self = Self::new(|| StandardMaterial { unlit: true,
                                                      alpha_mode: AlphaMode::Mask(0.0),
                                                      ..mycolor::GLOWY_3.into() });
  const GLOWY_2: Self = Self::new(|| StandardMaterial { unlit: true,
                                                        alpha_mode: AlphaMode::Mask(0.0),
                                                        ..mycolor::GLOWY_2.into() });
  const GLOWY_3: Self = Self::new(|| StandardMaterial { unlit: true,
                                                        alpha_mode: AlphaMode::Mask(0.0),
                                                        ..mycolor::GLOWY_3.into() });
  const EXPLOSION: Self = Self::new(|| StandardMaterial { unlit: true,
                                                          alpha_mode:
                                                            AlphaMode::Mask(0.0001),
                                                          ..mycolor::EXPLOSION.into() });
  const LASER: Self = Self::new(|| StandardMaterial { unlit: true,
                                                      alpha_mode:
                                                        AlphaMode::Mask(0.0001),
                                                      ..mycolor::LASER.into() });
  const PARTICLE: Self = Self::new(|| StandardMaterial::from(Color::srgb(0.2, 0.7, 0.9)));
  const INVISIBLE: Self = Self::new(|| StandardMaterial { unlit: true,
                                                          alpha_mode: AlphaMode::Blend,
                                                          ..Color::srgba(0.0, 0.0, 0.0,
                                                                         0.0).into() });
  const HOVERED: Self = Self::new(|| StandardMaterial { unlit: true,
                                                        alpha_mode: AlphaMode::Blend,
                                                        ..Color::srgba(0.0, 0.3, 1.0,
                                                                       0.1).into() });
  const PRESSED: Self = Self::new(|| StandardMaterial { unlit: true,
                                                        alpha_mode: AlphaMode::Blend,
                                                        ..Color::srgba(0.0, 0.3, 1.0,
                                                                       0.3).into() });
  const SELECTED: Self = Self::new(|| StandardMaterial { unlit: true,
                                                         alpha_mode: AlphaMode::Blend,
                                                         ..Color::srgba(0.0, 0.3, 1.0,
                                                                        0.2).into() });
}
#[derive(Copy, Clone, Hash, Eq, PartialEq)]
pub struct MyScene {
  path: &'static str,
  label: &'static str
}
impl MyScene {
  pub const fn new(path: &'static str, label: &'static str) -> Self { Self { path, label } }
  pub const fn path_and_label(&self) -> (&'static str, &'static str) {
    //     match self{

    // }
    (self.path, self.label)
  }
  pub const LUNAR_LANDER: Self = Self::new("lunarlander.glb", "Scene0");
  pub const CHARACTER_CONTROLLER_DEMO: Self =
    Self::new("character_controller_demo.glb", "Scene0");
  pub const LEVEL: Self = Self::new("level.glb", "Scene0");
  pub const A_LEVEL: Self = Self::new("alevel.gltf", "Scene0");
  pub const ISLAND_LEVEL: Self = Self::new("this_here_level.glb", "Scene0");
  pub const SOME_SKETCH_LEVEL: Self = Self::new("somesketchlevel.glb", "Scene0");
  pub const SNOWMAN: Self = Self::new("snowman.glb", "Scene0");
  pub const COFFEE_SCENE: Self = Self::new("coffee.glb", "Scene0");
  pub const GOXEL_LEVEL: Self = Self::new("goxel_level.glb", "Scene0");
  pub const TURTLE_LEVEL: Self = Self::new("turtle level.gltf", "Scene0");
  pub const WAT: Self = Self::new("wat.glb", "Scene0");
}
#[derive(Copy, Clone, Hash, Eq, PartialEq)]
pub struct GenMesh {
  gen_fn: fn() -> Mesh
}
impl GenMesh {
  const fn new(gen_fn: fn() -> Mesh) -> Self { Self { gen_fn } }
  pub fn gen(&self) -> Mesh { (self.gen_fn)() }
  pub const UNIT_CUBE: Self = Self::new(|| Cuboid::new(1.0, 1.0, 1.0).into());
  pub const UNIT_CYLINDER: Self = Self::new(|| primitives::Cylinder::new(1.0, 1.0).into());
  pub const CUBE: Self = Self::new(|| Cuboid::new(0.7, 0.7, 0.7).into());
  pub const BOX: Self = Self::new(|| Cuboid::new(2.0, 1.0, 1.0).into());
  pub const FLAT_BOX: Self = Self::new(|| Cuboid::new(2.1, 0.3, 2.1).into());
  pub const CAPSULE: Self = Self::new(|| primitives::Capsule3d::default().into());
  pub const TORUS: Self = Self::new(|| primitives::Torus::default().into());
  pub const SPHERE: Self = Self::new(|| primitives::Sphere { radius: 1.0 }.into());
  pub const PLANE_SIZE_50: Self = Self::new(|| Cuboid::new(25.0, 0.1, 25.0).into());
  pub const BILLBOARD_MESH_SQUARE: Self = Self::new(|| {
    primitives::Rectangle::new(BILLBOARD_REL_SCALE, BILLBOARD_REL_SCALE).into()
  });
}
mod mymesh {
  use {crate::BILLBOARD_REL_SCALE,
       bevy::{math::primitives,
              prelude::{Cuboid, Mesh}}};
  type MeshFn = fn() -> Mesh;
  pub const UNIT_CUBE: MeshFn = || Cuboid::new(1.0, 1.0, 1.0).into();
  pub const UNIT_CYLINDER: MeshFn = || primitives::Cylinder::new(1.0, 1.0).into();
  pub const CUBE: MeshFn = || Cuboid::new(0.7, 0.7, 0.7).into();
  pub const BOX: MeshFn = || Cuboid::new(2.0, 1.0, 1.0).into();
  pub const FLAT_BOX: MeshFn = || Cuboid::new(2.1, 0.3, 2.1).into();
  pub const CAPSULE: MeshFn = || primitives::Capsule3d::default().into();
  pub const TORUS: MeshFn = || primitives::Torus::default().into();
  pub const SPHERE: MeshFn = || primitives::Sphere { radius: 1.0 }.into();
  pub const PLANE_SIZE_50: MeshFn = || Cuboid::new(25.0, 0.1, 25.0).into();
  pub const BILLBOARD_MESH_SQUARE: MeshFn =
    || primitives::Rectangle::new(BILLBOARD_REL_SCALE, BILLBOARD_REL_SCALE).into();
}

// pub struct GenMeshNew(fn() -> dyn Into<Mesh>);

// impl GenMeshNew {
//   const fn new(f: fn() -> Box<dyn Into<Mesh>>) -> Self { Self(f) }
//   pub const UNIT_CUBE: Self = Self::new({
//     let f: fn() -> dyn Into<Mesh> = || Cuboid::new(1.0, 1.0, 1.0);
//     // let d: &'static dyn IntoFn<Mesh> = &|| Cuboid::new(1.0, 1.0, 1.0);
//     f
//     // &|| Cuboid::new(1.0, 1.0, 1.0)
//   });
//   pub const UNIT_CUBE2: Self = Self::new(&|| Cuboid::new(1.0, 1.0, 1.0));
// }
const LKLH: &'static dyn Fn(i32) -> i32 = &(|a| a + 1);
// const LAAKLH: &'static dyn Fn(i32) -> i32 = &(|a| a + 1);
// const LAAKLH: dyn Fn(i32) -> i32 = (|a| a + 1);
type IntoDynFn<T> = fn() -> dyn Into<T>;
// const CUBOID_FN: IntoDynFn<Mesh> = || Cuboid::new(1.0, 1.0, 1.0);

trait IntoFn<Target> {
  fn call_to_get_target(&self) -> Target;
}
impl<T, I: Into<T>> IntoFn<T> for fn() -> I {
  fn call_to_get_target(&self) -> T { self().into() }
}

// #[derive(Hash, Eq, PartialEq)]
comment! {
  pub struct GenMeshNew(&'static dyn IntoFn<Mesh>);

  impl GenMeshNew {
    const fn new(f: &'static dyn IntoFn<Mesh>) -> Self { Self(f) }
    pub const UNIT_CUBE: Self = Self::new({
      const f: fn() -> Cuboid = || Cuboid::new(1.0, 1.0, 1.0);
      const k: &'static dyn IntoFn<Mesh> = &f;
      // let d: &'static dyn IntoFn<Mesh> = &|| Cuboid::new(1.0, 1.0, 1.0);
      k
      // &|| Cuboid::new(1.0, 1.0, 1.0)
    });
    pub const UNIT_CUBE2: Self = Self::new(&|| Cuboid::new(1.0, 1.0, 1.0));
  }
  pub static PLANE_SIZE_30: GenMeshNew = GenMeshNew::new(|| Cuboid::new(25.0, 0.1, 25.0));
}
macro_rules! lazy {
  ($NAME:ident, $type:ty, $value:expr) => {
    static $NAME: LazyLock<$type> = LazyLock::new(|| $value);
  };
}
static ASDFA: i32 = 5;

// lazy!(ASDAAAS, Vec<i32>, vec![1, 2, 3]);
// static LKJLJ: Box<i32> = Box::leak(5);

// static STATIC_CTOR: HashMap<u32, &'static str> = {
//   let mut m = HashMap::new();
//   m.insert(0, "foo");
//   m.insert(1, "bar");
//   m.insert(2, "baz");
//   // libc_eprintln!("STATIC_CTOR");
//   m
// };
fn array_range<const LEN: usize>() -> [usize; LEN] {
  let mut arr = [0; LEN];
  for i in 0..LEN {
    arr[i] = i;
  }
  arr
}
fn prob(p: f32) -> bool { p > rand::random::<f32>() }
// pub fn cuboid_coords(IVec3 { x, y, z }: IVec3) -> impl Iterator<Item = IVec3> {
//   (-x..x).flat_map(move |a| {
//            (-y..y).flat_map(move |b| (-z..z).map(move |c| IVec3::new(a, b, c)))
//          })
// }
pub fn cuboid_coords(IVec3 { x, y, z }: IVec3) -> impl Iterator<Item = IVec3> {
  (-x..x).flat_map(move |a| {
           (-y..y).flat_map(move |b| (-z..z).map(move |c| IVec3::new(a, b, c)))
         })
}
// pub fn sphere_full_iter(center: IVec3, radius: i32) -> impl Iterator<Item = IVec3> {
//   cuboid_full_iter(center - IVec3::splat(radius),IVec3::splat(radius * 2)).filter(move |v: &IVec3| v.distance_squared(center) <= radius.pow(2))
// }
#[derive(Clone, Copy, PartialEq, Eq, Default)]
enum VisualsOrientation {
  DirectToCamera,
  DirectUp,
  #[default]
  VerticalToCamera
}
#[derive(Component, Clone, PartialEq, Eq, Default)]
pub struct Visuals {
  text: Option<String>,
  material_mesh: Option<(MyMaterial, GenMesh)>,
  sprite: Option<MySprite>,
  orientation: VisualsOrientation,
  character: Option<char>,
  unlit: bool,
  done: bool
}

impl Visuals {
  fn none() -> Self { default() }
  fn sprite(sprite: MySprite) -> Self {
    Self { sprite: Some(sprite),
           ..default() }
  }
  fn unlit_sprite(sprite: MySprite) -> Self {
    Self { sprite: Some(sprite),
           unlit: true,
           ..default() }
  }
  fn character(chardisplay: char) -> Self {
    Self { character: Some(chardisplay),
           ..default() }
  }
  fn material_mesh(material: MyMaterial, mesh: GenMesh) -> Self {
    Self { material_mesh: Some((material, mesh)),
           ..default() }
  }
  fn material_sphere(material: MyMaterial) -> Self {
    Self::material_mesh(material, GenMesh::SPHERE)
  }
  fn with_text(self, text: impl ToString) -> Self {
    Self { text: Some(text.to_string()),
           ..self }
  }
}
#[derive(Component, Clone)]
pub struct VisualSprite;

pub fn visuals(camq: Query<&GlobalTransform, With<Camera3d>>,
               serv: Res<AssetServer>,
               mut c: Commands,
               mut n: Local<u32>,
               mut visuals_q: Query<(Entity, Mut<Visuals>)>,
               mut visuals_sprites_q: Query<(&mut Transform, &GlobalTransform),
                     With<VisualSprite>>,
               mut sprite_3d_params: bevy_sprite3d::Sprite3dParams,
               mut sprite_handles: Local<HashMap<MySprite, Handle<Image>>>,
               mut mesh_handles: Local<HashMap<GenMesh, Handle<Mesh>>>,
               mut material_handles: Local<HashMap<MyMaterial, Handle<StandardMaterial>>>,
               mut visual_child_entities: Local<HashMap<Entity, Entity>>) {
  let mut get_material_handle = |material: MyMaterial| {
    material_handles.entry(material)
                    .or_insert_with(|| serv.add(material.val()))
                    .clone()
  };

  let mut get_mesh_handle = |mesh: GenMesh| {
    mesh_handles.entry(mesh)
                .or_insert_with(|| serv.add(mesh.gen()))
                .clone()
  };

  let mut get_sprite_handle = |sprite: MySprite| {
    sprite_handles.entry(sprite)
                  .or_insert_with(|| serv.load(format!("embedded://{}", sprite.path)))
                  .clone()
  };

  let text_color: TextColor = Color::WHITE.into();
  let text_font: TextFont = TextFont { font: default(),
                                       font_size: 20.0,
                                       font_smoothing: Default::default() };

  // let text_style = TextStyle { font_size: 30.0,
  //                              ..default() };
  let invisible_material = get_material_handle(MyMaterial::INVISIBLE);

  for (e, mut visuals) in &mut visuals_q {
    if visuals.is_changed() || !visuals.done {
      visuals.done = true;
      *n += 1;
      if *n % 100 == 0 {
        println!("{}", *n);
      }
      //       Text2d

      // /// # let font_handle: Handle<Font> = Default::default();
      // /// # let mut world = World::default();
      // /// #
      // /// // Basic usage.
      // /// world.spawn(Text2d::new("hello world!"));
      // ///
      // /// // With non-default style.
      // /// world.spawn((
      // ///     Text2d::new("hello world!"),
      // ///     TextFont {
      // ///         font: font_handle.clone().into(),
      // ///         font_size: 60.0,
      // ///         ..Default::default()
      // ///     },
      // ///     TextColor(BLUE.into()),
      // /// ));
      // ///
      // /// // With text justification.
      // /// world.spawn((
      // ///     Text2d::new("hello world\nand bevy!"),
      // ///     TextLayout::new_with_justify(JustifyText::Center)
      // /// ));
      // /// ```
      // #[derive(Component, Clone, Debug, Default, Deref, DerefMut, Reflect)]
      // #[reflect(Component, Default, Debug)]
      // #[require(
      //     TextLayout,
      //     TextFont,
      //     TextColor,
      //     TextBounds,
      //     Anchor,
      //     SpriteSource,
      //     Visibility,
      //     Transform
      // )]
      // pub struct Text2d(pub String);
      // Text2dBundle { text: Text::from_section(text, text_style.clone()),
      //                transform:
      //                  Transform::from_xyz(0.0, 1.5, 0.0).with_scale(Vec3::splat(0.07)),
      //                ..default() };

      if let Some(text) = visuals.text.clone() {
        c.entity(e).insert((Text2d::new(text),
                            TextFont { // font: font_handle.clone().into(),
                                       font_size: 30.0,
                                       ..default() },
                            TextColor(bevy::color::palettes::css::WHITE.into())));
      }
      if let Some(sprite) = visuals.sprite {
        let sprite_handle = get_sprite_handle(sprite);
        if let Some(image) = sprite_3d_params.images.get(&sprite_handle) {
          let pixels_per_metre = image.height() as f32;
          let builder = bevy_sprite3d::Sprite3dBuilder { image: sprite_handle,
                                                         pixels_per_metre,
                                                         double_sided: true,
                                                         alpha_mode: AlphaMode::Blend,
                                                         unlit: visuals.unlit,
                                                         ..default() };
          let bundle = builder.bundle(&mut sprite_3d_params);
          c.entity(e).insert(bundle);
          // println("asdfasdf");
        } else {
          visuals.done = false;
        }
      }
      if let Some((material, gen_mesh)) = visuals.material_mesh {
        let material = get_material_handle(material);
        let mesh = get_mesh_handle(gen_mesh);
        c.entity(e).insert((Mesh3d(mesh), MeshMaterial3d(material)));
        // c.entity(e).insert(PbrBundle { material,
        //                                mesh,
        //                                ..default() });
      }
    }
  }
}
pub fn set_prev_loc(mut c: Commands,
                    mut locq: Query<(Entity, Option<&mut PrevLocation>, &Location)>) {
  for (e, mut oprevloc, loc) in &mut locq {
    let newprevloc = PrevLocation(*loc);
    if let Some(mut prevloc) = oprevloc {
      *prevloc = newprevloc;
    } else {
      c.entity(e).insert(newprevloc);
    }
  }
}
pub fn position_sprite_billboards(camq: Single<&Transform, With<Camera3d>>,
                                  mut c: Commands,
                                  time: ResMut<TimeTicks>,
                                  // frame_timestamp: ResMut<FrameTimeStamp>,
                                  mut billboardsq: Query<(Entity,
                                         &Location,
                                         Option<&PrevLocation>,
                                         &mut Transform),
                                        (With<Visuals>,
                                         Without<Camera3d>)>) {
  // a frame every TICK_TIME ticks
  let time_since = time.time_since_sim_frame();
  let cam_transform = **camq;
  for (e, loc, oprevloc, mut transform) in &mut billboardsq {
    // let dir = Vec3 { y: 0.0,
    //                  ..(transform.translation - cam_transform.translation) };
    let dir = transform.translation - cam_transform.translation;
    let updir = cam_transform.up();
    // let x = Some(42);

    // let Some(val @ 42 | val @ 43) = x;

    // let Some(&PrevLocation(prevloc))k = oprevloc ... else ..
    let prevloc = oprevloc.map_or(*loc, |&PrevLocation(loc)| loc);

    // let prevloc = oprevloc.map_or(*loc,);
    // let curr_loc = curr_locs.get(&e).copied().unwrap_or(loc);
    let frac = (time_since as f32 / FRAME_TIME_TICKS as f32).min(1.0);
    let translation = Vec3::from(prevloc).lerp(Vec3::from(*loc), frac) + Vec3::splat(0.5);
    *transform = Transform::from_translation(translation).looking_to(dir, updir);
  }
}

#[derive(Component, Clone)]
struct RandomMovement;
#[derive(Component, Clone)]
struct EnemyMovement;
#[derive(Component, Clone)]
struct AttackPlayer;
#[derive(Component, Clone)]
struct PlayerFollower;
#[derive(Component, Clone)]
struct DragonAttack;
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
enum Dir {
  North,
  Northeast,
  East,
  Southeast,
  South,
  Southwest,
  West,
  Northwest,
  #[default]
  Here
}

impl From<Dir> for IVec2 {
  fn from(dir: Dir) -> Self {
    use Dir::*;
    match dir {
      North => IVec2::NEG_Y,                    // Up
      Northeast => IVec2::X + IVec2::NEG_Y,     // Up-Right
      Northwest => IVec2::NEG_X + IVec2::NEG_Y, // Up-Left
      South => IVec2::Y,                        // Down
      Southwest => IVec2::NEG_X + IVec2::Y,     // Down-Left
      Southeast => IVec2::X + IVec2::Y,         // Down-Right
      East => IVec2::X,                         // Right
      West => IVec2::NEG_X,                     // Left
      Here => IVec2::ZERO                       // No movement
    }
  }
}
const DIRS: &[Dir] = &[Dir::North,
                       Dir::Northeast,
                       Dir::East,
                       Dir::Southeast,
                       Dir::South,
                       Dir::Southwest,
                       Dir::West,
                       Dir::Northwest,
                       Dir::Here];
impl Dir {
  fn rand() -> Self { *pick(DIRS).unwrap() }
  fn rel(loc1: Location, loc2: Location) -> Self {
    // Calculate the delta in x and z coordinates
    let delta_x = loc2.0.x.cmp(&loc1.0.x);
    let delta_z = loc2.0.z.cmp(&loc1.0.z);

    // Use DIRS to match based on the direction of delta_x and delta_z
    DIRS.iter()
        .find(|&&dir| {
          let dir_vec = IVec2::from(dir);
          (delta_x, delta_z)
          == (dir_vec.x.cmp(&IVec2::ZERO.x), dir_vec.y.cmp(&IVec2::ZERO.y))
        })
        .copied()
        .unwrap_or(Dir::Here)
  }
}

#[derive(Component, Debug, Default)]
pub struct MovesTo(Option<Location>);

#[derive(Component, Default)]
#[require(MovesTo)]
pub struct TryToMove(Dir);
const NORMAL_NPC_SCALE: f32 = 1.9;
const NORMAL_NPC_THRUST: f32 = 400.0;
#[derive(Component)]
pub struct Fire {
  pub dir: Dir
}
#[derive(Component)]
pub struct Combat {
  pub is_hostile: bool,
  pub hp: u32,
  pub damage: u32
}
#[derive(Component, Clone, Default)]
pub struct SpaceObject {
  pub scale: f32,
  pub click_target_entity: Option<Entity>
}

enum Alignment {
  LawfulGood,
  LawfulNeutral,
  LawfulEvil,
  NeutralGood,
  Neutral,
  NeutralEvil,
  ChaoticGood,
  ChaoticNeutral,
  ChaoticEvil
}
#[derive(Eq, PartialEq, Clone, Copy, Assoc, Default, Debug)]
#[func(pub const fn alignment(&self) -> Alignment)]
pub enum Faction {
  #[default]
  #[assoc(alignment = Alignment::Neutral)]
  Wanderers,
  #[assoc(alignment = Alignment::LawfulGood)]
  SpacePolice,
  #[assoc(alignment = Alignment::ChaoticEvil)]
  SpacePirates,
  #[assoc(alignment = Alignment::ChaoticNeutral)]
  SPACEWIZARDs,
  #[assoc(alignment = Alignment::NeutralGood)]
  Traders,
  #[assoc(alignment = Alignment::LawfulEvil)]
  Invaders
}
impl Faction {
  fn is_good(&self) -> bool {
    matches!(self.alignment(),
             Alignment::LawfulGood | Alignment::NeutralGood | Alignment::ChaoticGood)
  }
  fn is_bad(&self) -> bool { !self.is_good() }
  fn is_hostile(&self, target: Self) -> bool {
    (self.is_bad() || target.is_bad()) && (*self != target)
  }
}

// type Spawnable = fn() -> Box<dyn FnOnce(&mut Commands, Pos)>;
// impl<B: Bundle> From<B> for Box<dyn FnOnce(&mut Commands, Pos)> {
//   fn from(b: B) -> Self {
//     Box::new(|commands, pos| {
//       commands.spawn((pos, b));
//     })
//   }
// }

#[derive(Component)]
struct Char(char);
#[derive(Component)]
struct Container(Vec<Item>);
#[derive(Component)]
struct Quality(u8);

#[derive(Clone, Copy)]
enum Stone {
  Granite,
  Marble,
  Obsidian
}
#[derive(Clone, Copy)]
enum Metal {
  Iron,
  Gold,
  Silver,
  Steel,
  Mithril
}
#[derive(Clone, Copy)]
enum Wood {
  Oak,
  Pine,
  Birch,
  Ironwood
}
#[derive(Clone, Copy)]
enum Gem {
  Diamond,
  Ruby,
  Sapphire
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum MiscItem {
  Loot,
  Wood,
  Fish,
  SpaceCat,
  Person,
  Spice,
  Coffee,
  SpaceCoin,
  Crystal,
  DiHydrogenMonoxide,
  Rock,
  SpaceMinerals
}
#[derive(Clone, Copy)]
enum Material {
  Stone,
  // Stone(Stone),
  Metal(Metal),
  // Wood(Wood),
  Wood,
  Gem(Gem),
  CrystalOre,
  MetalOre,
  Dirt
}

#[derive(Clone, Copy)]
enum Tool {
  Pickaxe,
  Axe,
  Hammer
}
#[derive(Clone, Copy)]
enum Weapon {
  Sword,
  Bow,
  Spear
}

#[derive(Clone)]
enum Furniture {
  Table,
  Chair,
  Forge,
  Stairs,
  CraftingStation,
  Door,
  Bed
}
#[derive(Clone)]
enum Item {
  Raw(Material),
  Block(Material),
  Furniture { mat: Material, kind: Furniture },
  Tool { mat: Material, kind: Tool },
  Weapon { mat: Material, kind: Weapon },
  Gem(Gem),
  Misc(MiscItem)
}

#[derive(Clone, Component)]
struct TakeableItem(Item);
fn takeable_item_entity(loc: Location, item: Item) -> impl Bundle {
  (TakeableItem(item), loc)
}
fn takeable_tool_entity(loc: Location, mat: Material, kind: Tool) -> impl Bundle {
  takeable_item_entity(loc, Item::Tool { mat, kind })
}
fn takeable_weapon_entity(loc: Location, mat: Material, kind: Weapon) -> impl Bundle {
  takeable_item_entity(loc, Item::Weapon { mat, kind })
}
struct BoxFn<A, B>(Box<dyn Fn(A) -> B>);
impl<A, B> BoxFn<A, B> {
  fn new(f: impl Fn(A) -> B + 'static) -> Self { Self(Box::new(f)) }
}

comment! {
  #[derive(Clone, Copy)]
  enum Labor {
    Mine,
    Woodcut,
    Mason,
    Smith
  }

  #[derive(Component)]
  struct Minable {
    item: Item,
    labor: Labor
  }
  #[derive(Component)]
  struct Digger {
    speed: f32
  }
  #[derive(Component)]
  struct Craftable {
    skill: i32
  }
  #[derive(Component)]
  struct Fluid {
    flow: f32
  }
  #[derive(Component)]
  struct Ore {
    value: i32
  }
  #[derive(Component)]
  struct Magic {
    mana: i32
  }
  #[derive(Component)]
  struct Job(&'static str);
  #[derive(Component)]
  struct Burns {
    temp: i32
  }
  #[derive(Component)]
  struct Light(f32);
  #[derive(Component)]
  struct Flying;
}
// fn granite() -> impl Bundle { block(Material::Stone(Stone::Granite)) }
// fn marble() -> impl Bundle { block(Material::Stone(Stone::Marble)) }
// fn gold_ore() -> impl Bundle { block(Material::Metal(Metal::Gold)) }

// fn granite_table() -> impl Bundle {
//   furniture(Material::Stone(Stone::Granite), Furniture::Table)
// }

// fn oak_bed() -> impl Bundle { furniture(Material::Wood(Wood::Oak), Furniture::Bed) }
fn torch(loc: Location) -> impl Bundle {
  (loc, Visuals::unlit_sprite(MySprite::TORCH), colored_light(mycolor::ORANGE_500.into()))
}

fn basic_animal(nameval: &'static str, ch: char) -> impl Bundle {
  (name(nameval), RandomMovement, Visuals::character(ch))
}
fn basic_npc(loc: Location, nameval: &'static str, spr: MySprite) -> impl Bundle {
  (loc, name(nameval), RandomMovement, Visuals::sprite(spr))
}
fn snowman() -> impl Bundle { basic_animal("snowman", '⛄') }
fn sheep() -> impl Bundle { basic_animal("sheep", '🐑') }
fn duck() -> impl Bundle { basic_animal("duck", '🦆') }
fn rabbit() -> impl Bundle { basic_animal("rabbit", '🐇') }

fn enemy() -> impl Bundle {
  let k: &dyn std::ops::Add<i32, Output = i32> = &5;
  // EntityMut
  // World::entity_mut
  // EventReader
  (name("enemy"),
   EnemyMovement,
   AttackPlayer,
   Combat { hp: 30,
            damage: 1,
            is_hostile: true },
   // SpaceObjectBundle::new(NORMAL_NPC_SCALE, true, Visuals::character('👿'))
  )
}

fn spider() -> impl Bundle {
  (name("spider"),
   EnemyMovement,
   AttackPlayer,
   Combat { hp: 40,
            damage: 1,
            is_hostile: true },
   // SpaceObjectBundle::new(NORMAL_NPC_SCALE, true, Visuals::character('🕷'))
  )
}

fn fire() -> impl Bundle {
  (name("fire"),
   Fire { dir: Dir::East },
   // SpaceObjectBundle::new(1.0, false, Visuals::character('🔥'))
  )
}
fn monster(name: &'static str, ch: char, hp: u32, dmg: i32) -> impl Bundle {
  (Char(ch),
   Combat { hp,
            damage: 3,
            is_hostile: true })
}
// fn block(name: &'static str, color: &str) -> impl Bundle {
//   (Name(name), Tile { bg: Color::hex(color).unwrap() })
// }
// fn craftsman(name: &'static str, ch: char, job: &'static str) -> impl Bundle {
//   (entity(name, ch), Job(job), Container)
// }
// fn mage(name: &'static str, ch: char, mana: i32) -> impl Bundle {
//   (monster(name, ch, 50, 5), Magic { mana })
// }
// fn farmland() -> impl Bundle { (block("farmland", "5A3A1A"), Grows(WHEAT)) }
// fn magma() -> impl Bundle {
//   (block("magma", "FF4500"), Fluid { flow: 0.5 }, Burns { temp: 2000 }, Light(8.0))
// }

// Creature Templates
// fn dwarf() -> impl Bundle {
//   (craftsman("dwarf", '🧔', "miner"), Combat { hp: 100, dmg: 8 }, Digger { speed: 1.0 })
// }

// fn wizard() -> impl Bundle { mage("wizard", '🧙', 100) }
fn dragon() -> impl Bundle {
  (name("dragon"),
   EnemyMovement,
   DragonAttack,
   AttackPlayer,
   Combat { hp: 60,
            damage: 1,
            is_hostile: true },
   // SpaceObjectBundle::new(NORMAL_NPC_SCALE, true, Visuals::character('🐉'))
  )
}
// fn troll() -> impl Bundle { (monster("troll", '👹', 200, 30), Burns { temp: -10 }) }
// fn goblin() -> impl Bundle { (monster("goblin", '👺', 40, 8), Container) }

// Item Templates
// fn pickaxe() -> impl Bundle {
//   (entity("pickaxe", '⛏'), Tool { durability: 100 }, Craftable { skill: 5 })
// }
// fn wheat() -> impl Bundle { (entity("wheat", '🌾'), Grows(WHEAT_MATURE)) }
// fn wheat_mature() -> impl Bundle { (entity("mature wheat", '🌾'), Craftable { skill: 1 }) }
// fn potion() -> impl Bundle {
//   (entity("potion", '🧪'), Magic { mana: 50 }, Craftable { skill: 8 })
// }
// fn ring() -> impl Bundle { (entity("ring", '💍'), Magic { mana: 100 }, Light(3.0)) }

// // Machines/Workshops
// fn brewing_stand() -> impl Bundle {
//   (block("brewing stand", "8B4513"), Container, Craftable { skill: 3 })
// }
// fn anvil() -> impl Bundle {
//   (block("anvil", "4A4A4A"), Tool { durability: 1000 }, Craftable { skill: 10 })
// }

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u8)]
pub enum BlockTexture {
  Bricks,
  Grass,
  Rocks,
  Snow,
  Stone,
  Sand,
  Dirt,
  Slime,
  Water,
  StoneBricks,
  Lava
}
impl BlockTexture {
  pub const NUM: usize = variant_count::<Self>();
  pub const fn all_same(self) -> [Self; 3] { [self; 3] }
}
impl From<u8> for BlockTexture {
  fn from(index: u8) -> Self { unsafe { std::mem::transmute(index) } }
}
impl From<BlockTexture> for u8 {
  fn from(block_texture: BlockTexture) -> u8 { block_texture as u8 }
}
impl From<BlockTexture> for u32 {
  fn from(block_texture: BlockTexture) -> u32 { block_texture as u32 }
}
#[derive(Clone, Copy, PartialEq, Eq, Debug, Assoc, Component, Hash, Default)]
#[require(Location)]
#[func(pub const fn textures(&self) -> [BlockTexture; 3])]
#[repr(u8)]
pub enum BlockType {
  #[assoc(textures = BlockTexture::Bricks.all_same())]
  Bricks,
  #[assoc(textures = [BlockTexture::Grass, BlockTexture::Grass, BlockTexture::Dirt])]
  Grass,
  #[assoc(textures = BlockTexture::Rocks.all_same())]
  Rocks,
  #[assoc(textures = [BlockTexture::Snow, BlockTexture::Snow, BlockTexture::Dirt])]
  Snow,
  #[default]
  #[assoc(textures = BlockTexture::Stone.all_same())]
  Stone,
  #[assoc(textures = BlockTexture::Sand.all_same())]
  Sand,
  #[assoc(textures = BlockTexture::Dirt.all_same())]
  Dirt,
  #[assoc(textures = BlockTexture::Slime.all_same())]
  Slime,
  #[assoc(textures = BlockTexture::Water.all_same())]
  Water,
  #[assoc(textures = BlockTexture::StoneBricks.all_same())]
  StoneBricks,
  #[assoc(textures = BlockTexture::Lava.all_same())]
  Lava
}
impl BlockType {
  pub const NUM: usize = variant_count::<Self>();
}
impl From<u8> for BlockType {
  fn from(index: u8) -> Self { unsafe { std::mem::transmute(index) } }
}
impl From<BlockType> for u8 {
  fn from(block_type: BlockType) -> u8 { block_type as u8 }
}
impl From<BlockType> for u32 {
  fn from(block_type: BlockType) -> u32 { block_type as u32 }
}
fn block_entity(loc: Location, block_type: BlockType) -> impl Bundle {
  (loc, block_type, NewBlock)
}
// enum WorldLocationKind{
//   Wall,Floor,Air
// }
#[derive(QueryData)]
#[query_data(mutable)]
struct NameQ {
  name: &'static mut Name
}
fn cartesian_product<C: Clone,
                     D: Clone,
                     A: IntoIterator<Item = C>,
                     B: IntoIterator<Item = D>>(
  iter_a: A,
  iter_b: B)
  -> Vec<(C, D)> {
  let vec_b = vec(iter_b);
  fold(|acc: Vec<(C, D)>, item1| {
         vec_push_many(acc,
                       map(|item2| (item1.clone(), item2.clone()), vec_b.clone()))
       },
       default(),
       iter_a)
}

#[derive(Component, Copy, Clone, Debug, Default)]
#[component(storage = "SparseSet")]
pub struct NewBlock;
#[derive(Component, Copy, Clone, Debug, Default, Hash, Eq, PartialEq)]
pub struct Location(pub IVec3);
impl Location {
  pub const fn new(x: i32, y: i32, z: i32) -> Self { Self(IVec3::new(x, y, z)) }
  pub fn adjacents(self) -> Vec<Self> {
    let range = [-1, 0, 1];
    let rel_coords = map(|(x, (y, z))| IVec3::new(x, y, z),
                         cartesian_product(range, cartesian_product(range, range)));
    let coords = mapv(|rel_pos| (self.0 + rel_pos).into(), rel_coords);
    coords
    // mapv(|pos| (self.0 + pos).into(), [IVec3::new(1, 0, 0),
    //                                    IVec3::new(-1, 0, 0),
    //                                    IVec3::new(0, 0, 1),
    //                                    IVec3::new(0, 0, -1),
    //                                    IVec3::new(1, 1, 0),
    //                                    IVec3::new(-1, 1, 0),
    //                                    IVec3::new(0, 1, 1),
    //                                    IVec3::new(0, 1, -1),
    //                                    IVec3::new(1, -1, 0),
    //                                    IVec3::new(-1, -1, 0),
    //                                    IVec3::new(0, -1, 1),
    //                                    IVec3::new(0, -1, -1)])
  }
  pub const fn above(self) -> Self { Self(IVec3::new(self.0.x, self.0.y + 1, self.0.z)) }
  pub const fn below(self) -> Self { Self(IVec3::new(self.0.x, self.0.y - 1, self.0.z)) }
}
impl From<IVec3> for Location {
  fn from(value: IVec3) -> Self { Self(value) }
}
impl From<Location> for Vec3 {
  fn from(Location(ivec3): Location) -> Self { ivec3.as_vec3() }
}
impl From<Vec3> for Location {
  fn from(Vec3 { x, y, z }: Vec3) -> Self {
    let f = |k: f32| k.floor() as i32;
    Self(IVec3::new(f(x), f(y), f(z)))
  }
}
impl From<Location> for Transform {
  fn from(loc: Location) -> Self { Transform::from_translation(loc.into()) }
}

#[derive(Component, Copy, Clone, Debug, Default, Hash, Eq, PartialEq)]
pub struct PrevLocation(pub Location);
// #[derive(Default, Resource)]
// pub struct WorldLocationMap {
//   blocks: HashMap<Location, BlockType>,
//   player_loc: Option<Location>,
//   entities: HashMap<Location, HashSet<Entity>>,
//   entity_positions: HashMap<Entity, Location>
// }

// const WIDTH: usize = 256;
// const HEIGHT: usize = 64;
// const LENGTH: usize = 256;

type SpawnFn = Box<dyn FnOnce(&mut Commands, Location)>;

#[derive(Default)]
struct WorldTile {
  block: Option<BlockType>,
  entities: Vec<SpawnFn>,
  entities_above: Vec<SpawnFn>
}

impl WorldTile {
  fn empty() -> Self { default() }
  fn block(block_type: BlockType) -> Self {
    Self { block: Some(block_type),
           ..default() }
  }
  fn block_with_entities_above(block_type: BlockType, entities_above: Vec<SpawnFn>) -> Self {
    Self { block: Some(block_type),
           entities_above,
           ..default() }
  }
  fn entities(entities: Vec<SpawnFn>) -> Self {
    Self { entities,
           ..default() }
  }
}

fn bundle_spawn<B: Bundle>(b: B) -> SpawnFn {
  Box::new(move |c: &mut Commands, loc| {
    c.spawn(b).insert(loc);
  })
}

const WORLD_HEIGHT: u32 = 10;
fn generate_tile(noise: &Perlin, pos: IVec3) -> WorldTile {
  let IVec3 { x, y, z } = pos;
  let loc = Location::from(pos);
  let wanderer = bundle_spawn((Name::new("Wanderer"),
                               TryToMove::default(),
                               RandomMovement,
                               Visuals::sprite(MySprite::PLAYER)));
  let tree = bundle_spawn((Name::new("tree"), Visuals::sprite(MySprite::TREE)));
  let noise3d = |n: f64| noise.get([x as f64 * n, y as f64 * n, z as f64 * n]);
  let noise2d = |n: f64| noise.get([x as f64 * n, z as f64 * n]);
  let elevnoise = |scale: f64, n: f64| scale * noise.get([x as f64 * n, z as f64 * n]);
  // https://www.redblobgames.com/maps/terrain-from-noise/
  let elevation = (
    elevnoise(5.0, 0.03)
    // + elevnoise(0.01, 20.0)
    // + elevnoise(20.0, 0.01)
  ) as i32;
  let prob = |p| rand::random::<f32>() < p;
  // let surface_height = 5;
  let cave = noise.get([x as f64 / 20.0, y as f64 / 20.0, z as f64 / 20.0]) > 0.6;
  let height = elevation as i32;

  if cave || y > height {
    WorldTile::empty()
  } else if y == height {
    if prob(0.03) {
      WorldTile::block_with_entities_above(BlockType::Grass, vec![tree])
    } else if prob(0.01) {
      WorldTile::block_with_entities_above(BlockType::Grass, vec![wanderer])
    } else {
      WorldTile::block(BlockType::Grass)
    }
  } else if y > height - 3 {
    WorldTile::block(BlockType::Dirt)
  } else {
    WorldTile::block(BlockType::Stone)
  }
}

pub fn spawn_world(mut c: &mut Commands, mut blocksparam: BlocksParam) {
  let noise = Perlin::new(5);
  let bounds = IVec3::new(100, 10, 100);
  let coords = cuboid_coords(bounds);
  for (pos, tile) in coords.map(move |pos| (pos, generate_tile(&noise, pos))) {
    let loc = Location::from(pos);
    if let Some(block) = tile.block {
      blocksparam.set(loc, Some(block));
    }
    for spawn in tile.entities {
      spawn(&mut c, loc);
    }
    for spawn in tile.entities_above {
      spawn(&mut c, loc.above());
    }
  }
}

// fn player_movement(
//     keys: Res<ButtonInput<KeyCode>>,
//     mut player_query: Query<&mut Location, With<Player>>
// ) {
//     if let Ok(mut pos) = player_query.get_single_mut() {
//         if let Some(&key) = keys.get_just_pressed().nth(0) {
//             let dir = match key {
//                 KeyCode::KeyW => Some(IVec3::Z),
//                 KeyCode::KeyS => Some(IVec3::NEG_Z),
//                 KeyCode::KeyA => Some(IVec3::NEG_X),
//                 KeyCode::KeyD => Some(IVec3::X),
//                 KeyCode::Space => Some(IVec3::Y),
//                 KeyCode::ShiftLeft => Some(IVec3::NEG_Y),
//                 _ => None
//             };

//             if let Some(dir) = dir {
//                 pos.0 += dir;
//             }
//         }
//     }
// }

use {bevy::render::camera::CameraRenderGraph, bevy_mod_index::prelude::*};

// struct Location;
impl IndexInfo for Location {
  type Component = Location;
  type Value = Self::Component;
  type Storage = HashmapStorage<Self>;
  const REFRESH_POLICY: IndexRefreshPolicy = IndexRefreshPolicy::WhenRun;
  fn value(c: &Self::Component) -> Self::Value { *c }
}

fn cube_positions(center: Location,
                  radius: i32,
                  include_center: bool)
                  -> impl Iterator<Item = Location> {
  // let r = radius;
  let range = move || -radius..=radius;

  range().flat_map(move |dx| {
           range().flat_map(move |dy| {
                    range().filter_map(move |dz| {
                      let pos = IVec3::new(dx, dy, dz);
                      (include_center || pos != IVec3::ZERO).then_some(Location(center.0
                                                                                + pos))
                    })
                  })
         })
}
// pub fn get_loc(&self, entity: Entity) -> Option<Location> {
//   self.entity_positions.get(&entity).copied()
// }
// pub fn get_entities_at(&self, loc: Location) -> impl Iterator<Item = Entity> + '_ {
//   self.entities.get(&loc).into_iter().flatten().copied()
// }

// pub fn get_entities_adjacent(&self, entity: Entity) -> Vec<Entity> {
//   self.get_loc(entity)
//       .map(|loc| {
//         Self::cube_positions(loc, 1, false).flat_map(|p| self.get_entities_at(p))
//                                            .collect()
//       })
//       .unwrap_or_default()
// }
// pub fn get_entity_distance(&self, a: Entity, b: Entity) -> Option<i32> {
//   let pos_a = self.entity_positions.get(&a)?.0;
//   let pos_b = self.entity_positions.get(&b)?.0;
//   let diff = pos_a - pos_b;
//   Some(diff.x.abs().max(diff.y.abs()).max(diff.z.abs()))
// }
// pub fn are_adjacent(&self, a: Entity, b: Entity) -> bool {
//   self.get_entity_distance(a, b).map_or(false, |d| d == 1)
// }

// pub fn get_player_loc(&self) -> Option<Location> { self.player_loc }

// #[derive(Resource, Default)]
// struct LocationsCache(HashMap<Location, HashSet<Entity>>);

// #[derive(SystemParam)]
// struct LocationsParam<'w, 's> {
//   locsq: Query<'w, 's, (Entity, &'static mut Location)>,
//   cache: ResMut<'w, LocationsCache>
// }
#[derive(SystemParam)]
struct BlocksParam<'w, 's> {
  // locsparam: LocationsParam<'w, 's>,
  locsindex: Index<'w, 's, Location>,
  blocks: Query<'w, 's, (Entity, &'static mut BlockType)>,
  voxel_world: VoxelWorld<'w, MyMainWorld>,
  commands: Commands<'w, 's>
}

impl<'w, 's> BlocksParam<'w, 's> {
  /// Sets or removes the block type for the given location. Spawns an entity if none exists.
  fn set(&mut self, loc: Location, option_new_block_type: Option<BlockType>) {
    let Self { blocks,
               commands,
               locsindex,
               voxel_world } = self;
    let preexistingentities = locsindex.lookup(&loc);
    let preexistingblockentity = find(|&e| blocks.contains(e), preexistingentities);
    if let Some(block_entity) = preexistingblockentity {
      commands.entity(block_entity).despawn();
    }
    if let Some(new_block_type) = option_new_block_type {
      commands.spawn((loc, new_block_type));
    }
    voxel_world.set_voxel(loc.0,
                          option_new_block_type.map_or(WorldVoxel::Air, WorldVoxel::Solid));
  }

  pub fn get(&mut self, loc: Location) -> Option<BlockType> {
    let e = self.locsindex.lookup_single(&loc).ok()?;
    let (_, &block_type) = self.blocks.get(e).ok()?;
    Some(block_type)
  }
  fn get_adjacent_walkable(&mut self, loc: Location) -> Vec<Location> {
    let adjacents = [IVec3::new(1, 0, 0),
                     IVec3::new(-1, 0, 0),
                     IVec3::new(0, 0, 1),
                     IVec3::new(0, 0, -1),
                     IVec3::new(1, 1, 0),
                     IVec3::new(-1, 1, 0),
                     IVec3::new(0, 1, 1),
                     IVec3::new(0, 1, -1),
                     IVec3::new(1, -1, 0),
                     IVec3::new(-1, -1, 0),
                     IVec3::new(0, -1, 1),
                     IVec3::new(0, -1, -1)];

    adjacents.iter()
             .map(|&offset| Location(loc.0 + offset))
             .filter(|&pos| {
               let below = Location(pos.0 - IVec3::Y);
               self.get(pos).is_none() && // No block at position
                self.get(below).is_some() // Has floor
                                          //  &&
                                          // !self.get_entities_at(pos).next().is_some() // No entities
             })
             .collect()
  }
}

// pub fn sync_locations_new(mut world_locs: ResMut<WorldLocationMap>,
//                           mut playerq: Option<Single<&Location, With<Player>>>,
//                           // mut er_on_remove: EventReader<OnRemove>,
//                           added: Query<(Entity, &Location, Option<&BlockType>),
//                                 Added<Location>>,
//                           mut removed_locs: RemovedComponents<Location>,
//                           moved: Query<(Entity, &Location), Changed<Location>>,

//                           mut voxel_world: VoxelWorld<MyMainWorld>) {
//   // SystemParam
//   // QueryData
//   // let commands:Commands;
//   //  commands.spawn(Screenshot::primary_window())
//   //     .observe(save_to_disk("screenshot.png"));
//   // save_to_disk
//   // Screenshot
//   // for k in er_on_remove.read() {}
//   // Observer
//   // KeyboardInput
//   // Event
//   // OnRemove
//   // Changed
//   world_locs.player_loc = playerq.as_deref().copied().copied();
//   let WorldLocationMap { blocks,
//                          entities,
//                          player_loc,
//                          entity_positions } = world_locs.as_mut();

//   // Update player location
//   *player_loc = playerq.iter().next().copied();

//   // Handle new/added entities
//   for (e, &loc, block_type) in &added {
//     if let Some(&bt) = block_type {
//       blocks.insert(loc, bt);
//       let v: WorldVoxel<BlockType> = WorldVoxel::Solid(bt);
//       voxel_world.set_voxel(loc.0, v);
//     }
//     entities.entry(loc).or_default().insert(e);
//     entity_positions.insert(e, loc);
//   }

//   // Handle removed entities/blocks
//   for e in removed_locs.read() {
//     if let Some(loc) = entity_positions.remove(&e) {
//       if let Some(entity_set) = entities.get_mut(&loc) {
//         entity_set.remove(&e);
//         if entity_set.is_empty() {
//           entities.remove(&loc);
//         }
//       }

//       if blocks.remove(&loc).is_some() {
//         voxel_world.set_voxel(loc.0, WorldVoxel::Unset);
//       }
//     }
//   }

//   // Handle moved entities/blocks
//   for (e, &new_loc) in &moved {
//     // First handle the old location if it exists
//     if let Some(old_loc) = entity_positions.get(&e).copied() {
//       if let Some(entity_set) = entities.get_mut(&old_loc) {
//         entity_set.remove(&e);
//         if entity_set.is_empty() {
//           entities.remove(&old_loc);
//         }
//       }

//       comment! {
//         if blocks.remove(&old_loc).is_some() {
//           voxel_world.set_voxel(old_loc.0, WorldVoxel::Unset);
//         }
//       }
//     }

//     // Then handle the new location
//     entities.entry(new_loc).or_default().insert(e);
//     entity_positions.insert(e, new_loc);

//     // bevy::math::IVec3
//     if let Some(&bt) = blocks.get(&new_loc) {
//       voxel_world.set_voxel(new_loc.0, bt.into());
//     }
//   }
// }

// fn maintain_voxel_scene(mut voxel_world: VoxelWorld<MyMainWorld>,
//                         mut world_locs: ResMut<WorldLocationMap>) {
//   // Then we can use the `u8` consts to specify the type of voxel

//   // 20 by 20 floor
//   for x in -10..10 {
//     for z in -10..10 {
//       voxel_world.set_voxel(IVec3::new(x, -1, z), BlockType::Snow.into());

//       // Grassy floor
//     }
//   }

//   // Some bricks
//   // voxel_world.set_voxel(IVec3::new(0, 0, 0), BlockType::Snow.into());
//   // voxel_world.set_voxel(IVec3::new(0, 0, 0), BlockType::Snow.into());
//   // voxel_world.set_voxel(IVec3::new(1, 0, 0), BlockType::Snow.into());
//   // voxel_world.set_voxel(IVec3::new(0, 0, 1), BlockType::Snow.into());
//   // voxel_world.set_voxel(IVec3::new(0, 0, -1), BlockType::Stone.into());
//   // voxel_world.set_voxel(IVec3::new(-1, 0, 0), BlockType::Stone.into());
//   // voxel_world.set_voxel(IVec3::new(-2, 0, 0), BlockType::Sand.into());
//   // voxel_world.set_voxel(IVec3::new(-1, 1, 0), BlockType::Bricks.into());
//   // voxel_world.set_voxel(IVec3::new(-2, 1, 0), BlockType::Snow.into());
//   // voxel_world.set_voxel(IVec3::new(0, 1, 0), BlockType::Snow.into());
// }

// fn set_frame_timestamp(time: Res<TimeTicks>, mut frame_timestamp: ResMut<FrameTimeStamp>) {
//   frame_timestamp.0 = time.0;
// }
// fn player_movement(keys: Res<ButtonInput<KeyCode>>,
//                    mut player_query: Query<&mut TryToMove, With<Player>>) {
//   if let Some(&key) = keys.pressed().nth(0)
//      && let Ok(mut player_trytomove) = player_query.get_single_mut()
//   {
//     let dir = match key {
//       KeyCode::KeyW => Dir::North,
//       KeyCode::KeyA => Dir::West,
//       KeyCode::KeyS => Dir::South,
//       KeyCode::KeyD => Dir::East,
//       _ => Dir::Here
//     };
//     *player_trytomove = TryToMove(dir);
//   }
// }
#[derive(Default, Resource)]
pub struct PressedKeys(HashSet<KeyCode>);

fn get_pressed_keys(keys: Res<ButtonInput<KeyCode>>, mut pressed_keys: ResMut<PressedKeys>) {
  pressed_keys.0.extend(keys.get_pressed());
}
fn player_movement(// keys: Res<ButtonInput<KeyCode>>,
                   mut pressed_keys: ResMut<PressedKeys>,
                   mut player_trytomove: Single<&mut TryToMove, With<Player>>) {
  // Define key-to-direction mapping
  let dir = [(KeyCode::KeyW, Dir::North),
             (KeyCode::KeyA, Dir::West),
             (KeyCode::KeyS, Dir::South),
             (KeyCode::KeyD, Dir::East)].iter()
                                        .find_map(|&(key, dir)| {
                                          pressed_keys.0.contains(&key).then_some(dir)
                                        })
                                        .unwrap_or(Dir::Here);

  pressed_keys.0 = default();
  **player_trytomove = TryToMove(dir);
}

fn random_movement(mut moversq: Query<&mut TryToMove, With<RandomMovement>>) {
  for mut trytomove in &mut moversq {
    // println("aaaaaa");
    *trytomove = TryToMove(Dir::rand());
  }
}
fn try_movement(mut moversq: Query<(&Location, &TryToMove, &mut MovesTo),
                      Without<BlockType>>,
                // mut blocksparam: BlocksParam,
                mut locsindex: Index<Location>,
                blocksq: Query<(Entity, &BlockType)>,
                // world_locs: Res<WorldLocationMap>,
                time: Res<TimeTicks>) {
  let mut is_solid = |pos: Location| locsindex.lookup(&pos).any(|e| blocksq.contains(e));
  // let mut is_walkable = |&pos: &Location| // is_solid(pos.below()) &&
  //   !is_solid(pos);
  for (&loc, &TryToMove(try_to_move_dir), mut moves_to) in &mut moversq {
    let mut adjacent_walkable: Vec<_> = loc.adjacents()
                                           .iter()
                                           .copied()
                                           .filter(|&pos: &Location| !is_solid(pos))
                                           .collect();
    let mut is_place_to_fall_down = |pos: Location| !is_solid(pos.below());
    // gravity
    moves_to.0 = if is_place_to_fall_down(loc) {
      Some(loc.below())
    } else if let Some(&new_pos) =
      adjacent_walkable.iter()
                       .find(|&&otherloc| Dir::rel(otherloc, loc) == try_to_move_dir)
    {
      Some(new_pos)
    } else {
      None
    }
  }
}

fn movement(mut moversq: Query<(&mut MovesTo, &mut Location)>) {
  for (mut moves_to, mut loc) in &mut moversq {
    if let MovesTo(Some(new_loc)) = *moves_to {
      *loc = new_loc;
      *moves_to = default();
    }
  }
}

// fn camera_movement(mut camq: Query<(&mut PanOrbitCamera, &Transform)>,
//                    world_locs: Res<WorldLocationMap>,
//                    mut cam_target_pos: Local<Vec3>,
//                    keys: Res<ButtonInput<KeyCode>>,
//                    player_query: Query<Entity, With<Player>>,
//                    mut targetq: Query<&mut Location, With<CameraTarget>>,
//                    mut posindidcatiorq: Query<&mut Transform,
//                          (With<CameraPosIndicator>,
//                           Without<PanOrbitCamera>)>) {
//   if let Some(player_pos) = world_locs.get_player_loc()
//      && let Ok((mut cam, cam_transform)) = camq.get_single_mut()
//      && let Ok(mut targetpos) = targetq.get_single_mut()
//      && let Ok(mut posindicatortransform) = posindidcatiorq.get_single_mut()
//   {
//     let up = Vec3::Y;
//     let forward = Vec3 { y: 0.0,
//                          ..cam_transform.forward().into() }.normalize_or_zero();
//     let right = forward.cross(up);

//     let Vec3 { x, y, z } =
//       sum(filter_map(|(key, v)| keys.pressed(key).then_some(v),
//                      [(KeyCode::KeyA, Vec3::NEG_X),
//                       (KeyCode::KeyS, Vec3::NEG_Z),
//                       (KeyCode::KeyD, Vec3::X),
//                       (KeyCode::KeyW, Vec3::Z),
//                       (KeyCode::ControlLeft, Vec3::NEG_Y),
//                       (KeyCode::ShiftLeft, Vec3::Y)])).normalize_or_zero();
//     let keyb_dir = (x * right) + (z * forward) + (y * up);
//     *cam_target_pos += keyb_dir * 0.15;
//     // cam.target_focus = player_pos.0.as_vec3();
//     cam.target_focus = *cam_target_pos;
//     *targetpos = Location::from(*cam_target_pos);
//     *posindicatortransform =
//       Transform::from_translation(*cam_target_pos).with_scale(Vec3::splat(0.1));
//   }
// }

fn camera_follow_player(mut camq: Single<(&mut Camera3d, &mut Transform),
                               Without<Player>>,
                        // world_locs: Res<WorldLocationMap>,
                        mut cam_target_pos: Local<Vec3>,
                        keys: Res<ButtonInput<KeyCode>>,
                        playerq: Single<&Transform, With<Player>>) {
  let player_transform = **playerq;
  let (mut cam, mut cam_transform) = camq.into_inner();
  // println("asdfasdfsad");
  cam_transform.translation = player_transform.translation + Vec3::Y * 15.0;
  // *cam_transform.rotation = *Quat::from_rotation_x(-PI * 0.5);
  // Rotate the camera to look down at the player and add 180 degrees around Y-axis
  let base_rotation = Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2); // -PI/2 for looking down
  let y_rotation = Quat::from_rotation_y(std::f32::consts::PI); // 180 degrees around Y-axis
  cam_transform.rotation = y_rotation * base_rotation; // Combine rotations

  // cam.target_focus = player_transform.translation;
  // *targetpos = Location::from(*cam_target_pos);
  // *posindicatortransform =
  //   Transform::from_translation(*cam_target_pos).with_scale(Vec3::splat(0.1));
}

#[derive(Component)]
struct Sun;

pub fn sun_movement(mut camq: Single<&GlobalTransform, With<Camera3d>>,
                    time: Res<Time>,
                    mut sunq: Single<&mut Transform, With<Sun>>) {
  // World::load_asset_with_settings(…) (as DirectAssetAccessExt)
  // System
  let camera_globaltransform = *camq;
  let mut sun_transform = sunq.into_inner();
  let rot_time_seconds = 10.0;
  let time_seconds = time.elapsed_secs();
  let rot_radians = (time_seconds / rot_time_seconds) * TAU;

  let cam_pos = camera_globaltransform.translation();
  let new_sun_pos = cam_pos
                    + Vec3 { x: rot_radians.cos() * 100.0,
                             y: 60.0,
                             z: rot_radians.sin() * 100.0 };

  sun_transform.translation = new_sun_pos;
  let dir = new_sun_pos - cam_pos;
  sun_transform.look_to(dir, Vec3::Y);
}
const ADD_ONE: fn(i32) -> i32 = |x| x + 1;

// Simple movement system example
// pub fn move_entities(world: Res<WorldLocationMap>,
//                      mut query: Query<(&mut Position, &Movement)>,
//                      time: Res<Time>) {
//   for (mut pos, movement) in query.iter_mut() {
//     let target = pos.0 + movement.direction;
//     if world.is_walkable(target) {
//       pos.0 = target;
//     }
//   }
// }

#[derive(Clone)]
enum Tile {
  Wall(Material),
  // Floor(Material),
  Open {
    floor: Option<Material>,
    entities: Vec<Entity>
  }
}

comment! {
  fn furniture(mat: Material, kind: Furniture) -> impl Bundle {
    (Tile::Furniture(kind, mat), Furniture)
  }

  fn forge(mat: Material) -> impl Bundle {
    (name("forge"), Tile::Furniture(Furniture::Forge, mat), Container(vec![]), Quality(0))
  }
}

pub fn from<B, A: From<B>>(b: B) -> A { A::from(b) }

// #[derive(Assoc, Copy, Clone, Hash, Eq, PartialEq, Debug)]
// #[func(pub fn probs(&self) -> Spawnable)]
// pub enum ZoneType {
//   #[assoc(probs = Spawnable::SPACE_PIRATE_ASTEROID_FIELD)]
//   SpacePirateASTEROIDField,
//   #[assoc(probs = Spawnable::INVADERS)]
//   InvaderAttack,
//   #[assoc(probs = Spawnable::TRADING_ZONE)]
//   TradingZone,

//   #[assoc(probs = Spawnable::NORMAL_ASTEROID_FIELD)]
//   ASTEROIDField,
//   #[assoc(probs = Spawnable::NON_COMBAT_ICE_ASTEROID_FIELD)]
//   ICEASTEROIDField,
//   #[assoc(probs = Spawnable::PIRATE_ICE_ASTEROID_FIELD)]
//   PirateICEASTEROIDField,
//   #[assoc(probs = Spawnable::SPACE_STATION_ZONE_PROBS)]
//   SPACESTATIONZone // #[assoc(probs = Spawnable::ANOMALY_CLUSTER_PROBS)]
//                    // AnomalyCluster,
//                    // #[assoc(probs = Spawnable::EXOTIC_LIFE_ZONE_PROBS)]
//                    // ExoticLifeZone,
//                    // #[assoc(probs = Spawnable::MINEFIELD_ZONE_PROBS)]
//                    // MinefieldZone
// }

#[derive(Component)]
struct OriginTime(u32);
fn origin_time(q: Query<Entity, Without<OriginTime>>,
               time_ticks: Res<TimeTicks>,
               mut c: Commands) {
  for e in &q {
    c.entity(e).insert(OriginTime(time_ticks.0));
  }
}

// #[derive(Component, Default, Clone)]
// pub struct Container(pub HashSet<Entity>);
// impl Container {
//   pub fn empty() -> Container { Container::default() }
// }
pub fn name(s: &'static str) -> Name { Name::new(s) }
#[derive(Component, Clone)]
pub struct TimedAnimation {
  pub num_frames: usize,
  pub time_per_frame_in_ticks: usize
}
// #[derive(Component, Clone)]
// pub struct PlayerFollower;
pub fn pick<T>(coll: impl IntoIterator<Item = T>) -> Option<T> {
  rand::seq::IteratorRandom::choose(coll.into_iter(), &mut thread_rng())
}
fn avg<T: std::iter::Sum + std::ops::Div<f32, Output = T>>(coll: impl IntoIterator<Item = T>)
                                                           -> Option<T> {
  let v = vec(coll);
  let n = v.len();
  let s = v.into_iter().sum::<T>();
  (n != 0).then(|| s / (n as f32))
}

#[derive(Component, Default, Debug, Clone, Copy)]
enum Navigation {
  #[default]
  None,
  Dir(Dir),
  Loc(Location),
  Chase(Entity)
}

#[derive(Default, Resource)]
pub struct TimeTicks(pub u32);

impl TimeTicks {
  fn time_since_sim_frame(&self) -> usize { self.0 as usize % FRAME_TIME_TICKS }
  fn is_sim_frame(&self) -> bool { self.time_since_sim_frame() == 0 }
}
fn is_sim_frame_condition(time_ticks: Res<TimeTicks>) -> bool { time_ticks.is_sim_frame() }

// fn every_n_ticks<const N: usize>(time: Res<TimeTicks>) -> bool { time.0 as usize % N == 0 }
// #[derive(Default, Resource)]
// pub struct FrameTimeStamp(pub u32);

pub fn increment_time(mut time: ResMut<TimeTicks>) { time.0 += 1; }
// pub fn timed_animation_system(time_ticks: Res<TimeTicks>,
//                               mut q: Query<(&TimedAnimation, &mut TextureAtlas)>) {
//   // Populated::
//   for (&TimedAnimation { num_frames,
//                          time_per_frame_in_ticks },
//        mut atlas) in &mut q
//   {
//     let time = time_ticks.0 as usize;
//     let index = |time| (time / time_per_frame_in_ticks) % num_frames;
//     let old_index = index(time.saturating_sub(1));
//     let new_index = index(time);
//     if new_index != old_index {
//       atlas.index = new_index;
//     }
//   }
// }

fn close_on_esc(mut exit: EventWriter<AppExit>, keyboard_input: Res<ButtonInput<KeyCode>>) {
  if keyboard_input.just_pressed(KeyCode::Escape) {
    exit.send(AppExit::Success);
  }
}

fn namefmt(oname: Option<&Name>) -> String {
  // ASDAAAS.
  match oname {
    Some(name) => name.to_string(),
    None => "unnamed entity".to_string()
  }
}

pub fn intersperse_newline<T: ToString>(coll: impl IntoIterator<Item = T>) -> String {
  concat_strings(coll.into_iter()
                     .map(|v| v.to_string())
                     .intersperse("\n".to_string()))
}
#[derive(Resource, Default, Clone)]
pub struct UIData {
  pub current_time_ticks: u32,
  pub interact_message: Option<String>,
  pub note: Vec<String>,
  pub game_over: bool,
  pub infobox_data: Vec<String>
}

const UI_BACKGROUND_COLOR: Color = Color::srgba(0.9, 0.9, 0.7, 1.0);
const UI_BORDER_COLOR: Color = Color::srgba(0.8, 0.8, 0.7, 1.0);
const UI_FONT_COLOR: Color = Color::srgba(0.3, 0.3, 0.3, 1.0);

comment! {
  pub fn common_style(sb: &mut StyleBuilder) {
    sb.font_size(32.0)
      .display(Display::Block)
      .border(3)
      .border_color(UI_BORDER_COLOR)
      .background_color(UI_BACKGROUND_COLOR)
      .position(bevy::ui::PositionType::Absolute)
      .color(UI_FONT_COLOR)
    // .margin(5)
    // .padding(3)
      .pointer_events(false);
  }
  #[derive(Clone, PartialEq)]
  pub struct UIPopup {
    pub style: fn(&mut StyleBuilder),
    pub display_text_fn: fn(&UIData) -> Vec<String>
  }
  impl ViewTemplate for UIPopup {
    type View = impl View;
    fn create(&self, cx: &mut Cx) -> Self::View {
      let &Self { display_text_fn,
                  style } = self;
      let uidata = cx.use_resource::<UIData>();
      let display_text = display_text_fn(uidata);
      Element::<NodeBundle>::new().style((common_style, style))
                                  .children(intersperse_newline(display_text.clone()))
    }
  }
  // const MESSAGE_SHOW_TIME_TICKS: u32 = 600;
  fn ui(mut c: Commands,
        // camq: Query<(Entity, &GlobalTransform), With<Camera3d>>,
        playerq: Query<(Entity, &Player, &Transform)>,
        mut ui_data: ResMut<UIData>,
        time: Res<TimeTicks>,
        view_root_q: Query<Entity, With<ViewRoot>>) {
    if view_root_q.is_empty() {
      let game_over_popup = UIPopup { style: |sb| {
        sb.font_size(55.0)
          .justify_self(JustifySelf::Center)
          .top(Val::Percent(50.0));
      },
                                      display_text_fn: |&UIData { game_over, .. }| {
                                        if game_over {
                                          vec!["Game Over".to_string()]
                                        } else {
                                          default()
                                        }
                                      } };
      let note_popup = UIPopup { style: |sb| {
        sb.justify_self(JustifySelf::Center)
          .top(Val::Percent(50.0));
      },
                                 display_text_fn: |uidata| uidata.note.clone() };
      let infobox = UIPopup { style: |sb| {
        sb.left(0).top(0);
      },
                              display_text_fn: |uidata| uidata.infobox_data.clone() };
      let root = vec![ViewChild::new(note_popup),
                      ViewChild::new(infobox),
                      ViewChild::new(game_over_popup),];
      c.spawn(root.to_root());
    }
    if let Ok((player_entity, player, player_transform)) = playerq.get_single() {
      let player_pos = player_transform.translation;
      // let player_light_on = player.light_on;
      let infobox_data = map(ToString::to_string, [format!("{:.1}", player_pos).as_str(),
                                                   // format!("you've found {} notes", player.notes_found.len()).as_str(),
                                                   // format!("light on: {player_light_on}",).as_str(),
                                                   "w,a,s,d: move",
                                                   "f: toggle flashlight"]) // .chain(map(|(item, n)| {
      //              format!("{} {:?}s", n, item)
      //            },
      //            player_inventory.0.clone()))
        .collect();

      let current_time = time.0;
      let current_time_ticks = current_time;
      let old_ui_data = ui_data.clone();
      *ui_data = UIData { current_time_ticks,
                          infobox_data,
                          ..old_ui_data };
    }
  }
}

pub fn string(t: impl ToString) -> String { t.to_string() }

const FRAME_TIME_TICKS: usize = 10;

pub const BLOOM: Bloom = Bloom { intensity: 0.5,
                                 low_frequency_boost: 0.0,
                                 prefilter: BloomPrefilter { threshold: 2.2,
                                                             threshold_softness: 0.0 },

                                 composite_mode: BloomCompositeMode::Additive,
                                 ..Bloom::NATURAL };

const TONEMAPPING: bevy::core_pipeline::tonemapping::Tonemapping =
  bevy::core_pipeline::tonemapping::Tonemapping::Reinhard;

const FOG_SETTINGS: DistanceFog =
  DistanceFog { color: Color::srgb(0.25, 0.25, 0.25),
                falloff: FogFalloff::ExponentialSquared { density: 0.5 },
                directional_light_color: Color::NONE,
                directional_light_exponent: 8.0 };

pub const AMBIENT_LIGHT: AmbientLight = AmbientLight { color: Color::hsv(0.0, 0.0, 1.0),
                                                       brightness: 2000.0 };
fn colored_light(color: Color) -> PointLightBundle {
  PointLightBundle { point_light: PointLight { color,
                                               intensity: 900_000.0,
                                               radius: 0.0,
                                               range: 5.0,
                                               shadows_enabled: false,
                                               ..default() },
                     ..default() }
}
// const COLORED_LIGHT: fn(Color) -> PointLightBundle =
//   |color| PointLightBundle { point_light: PointLight { color,
//                                                        intensity: 900_000.0,
//                                                        radius: 0.0,
//                                                        range: 5.0,
//                                                        shadows_enabled: false,
//                                                        ..default() },
//                              ..default() };
const CAMERA_TARGET_LIGHT: PointLight =
  PointLight { color: Color::hsv(33.0, 1.0, 0.5),
               intensity: 100_000.0,
               radius: 0.0,
               range: 13.0,
               shadows_enabled: true,
               shadow_depth_bias: PointLight::DEFAULT_SHADOW_DEPTH_BIAS / 10.0,
               shadow_normal_bias: PointLight::DEFAULT_SHADOW_NORMAL_BIAS / 10.0,
               shadow_map_near_z: PointLight::DEFAULT_SHADOW_MAP_NEAR_Z };

#[derive(Component)]
struct Note(&'static str);
#[derive(Default, Debug, Clone, PartialEq, Eq, Hash, States)]
enum GameState {
  Running,
  #[default]
  NotStarted,
  GameOver
}

fn rangerand(lo: f32, hi: f32) -> f32 { lo.lerp(hi, rand::random::<f32>()) }
fn random_normalized_vector() -> Vec3 { random::<Quat>() * Vec3::X }

// fn init(mut world: &mut World) { world.clear_entities() }
type NumberFunction = fn(i32) -> i32;

const INC: NumberFunction = |x| x + 1;
const DEC: &'static dyn Fn(i32) -> i32 = &|x| x - 1;

// fn player() -> impl Bundle {
//   (name("You"),
//    TryToMove::default(),
//    Player {},
//    Combat { hp: 30,
//             damage: 1,
//             is_hostile: false },
//    Visuals::sprite(MySprite::SPACEWIZARD))
// }
#[derive(Component, Debug, Default)]
#[require(
  Name(|| name("You")),
  TryToMove(TryToMove::default),
  Combat(|| Combat { hp: 30,
                  damage: 1,
                  is_hostile: false }),
  Visuals(|| Visuals::sprite(MySprite::SPACEWIZARD))
)]
pub struct Player {}

#[derive(Component, Debug)]
pub struct CameraPosIndicator;
#[derive(Component, Debug)]
pub struct CameraTarget;
// {
//   Display
// }

fn default_array<T: Default + Copy, const N: usize>() -> [T; N] { [T::default(); N] }

use haalka::{prelude::*, raw::Spawnable};
#[derive(Component)]
struct Counter(Mutable<i32>);

fn ui_root() -> impl Element {
  let counter = Mutable::new(0);
  El::<Node>::new()
        .height(Val::Percent(100.))
        .width(Val::Percent(100.))
        .align_content(Align::center())
        .child(
            Row::<Node>::new()
                .with_node(|mut node| node.column_gap = Val::Px(15.0))
                .item(counter_button(counter.clone(), "-", -1))
                .item(
                    El::<Text>::new()
                        .text_font(TextFont::from_font_size(25.))
                        .text_signal(counter.signal_ref(ToString::to_string).map(Text)),
                )
                .item(counter_button(counter.clone(), "+", 1))
                .update_raw_el(move |raw_el| raw_el.insert(Counter(counter))),
        )
}

fn counter_button(counter: Mutable<i32>, label: &str, step: i32) -> impl Element {
  let hovered = Mutable::new(false);
  El::<Node>::new().width(Val::Px(45.0))
                   .align_content(Align::center())
                   .background_color_signal(hovered.signal()
                                                   .map_bool(|| Color::hsl(300., 0.75, 0.85),
                                                             || Color::hsl(300., 0.75, 0.75))
                                                   .map(Into::into))
                   .border_radius(BorderRadius::MAX)
                   .hovered_sync(hovered)
                   .on_click(move || *counter.lock_mut() += step)
                   .child(El::<Text>::new().text_font(TextFont::from_font_size(25.))
                                           .text(Text::new(label)))
}

pub fn setup(playerq: Query<&Transform, With<Player>>,
             serv: Res<AssetServer>,
             mut meshes: ResMut<Assets<Mesh>>,
             mut materials: ResMut<Assets<StandardMaterial>>,
             mut c: Commands,
             mut blocksparam: BlocksParam) {
  c.spawn((Sun,
           Visuals::unlit_sprite(MySprite::SUN),
           NotShadowCaster,
           NotShadowReceiver,
           DirectionalLightBundle { directional_light:
                                      DirectionalLight { color: Color::WHITE,
                                                       illuminance: 11000.0,
                                                       shadows_enabled: true,
                                                       ..default()
                                                       // shadow_depth_bias: todo!(),
                                                       // shadow_normal_bias: todo!()
                                    },

                                    // Self {
                                    //     num_cascades: 1,
                                    //     minimum_distance: 0.1,
                                    //     maximum_distance: 100.0,
                                    //     first_cascade_far_bound: 5.0,
                                    //     overlap_proportion: 0.2,
                                    // }
                                    // cascade_shadow_config: CascadeShadowConfig
                                    transform: Transform::from_scale(Vec3::NEG_ONE),
                                    ..default() }));
  let mut spawn_block = |x, y, z| {
    c.spawn(block_entity(Location::new(x, y, z),
                         pick([BlockType::Dirt,
                               BlockType::Snow,
                               BlockType::Rocks,
                               BlockType::Stone,
                               BlockType::Bricks]).unwrap()));
  };
  spawn_block(0, 0, 0);
  spawn_block(1, 0, 0);
  spawn_block(0, 0, 1);
  spawn_block(0, 0, -1);
  spawn_block(-1, 0, 0);
  spawn_block(-2, 0, 0);
  spawn_block(-1, 1, 0);
  spawn_block(-2, 1, 0);
  spawn_block(0, 1, 0);

  for x in -10..10 {
    for z in -10..10 {
      spawn_block(x, 8, z);
    }
  }

  c.spawn((Location::default(),
           CameraPosIndicator,
           Visuals::material_sphere(MyMaterial::GLOWY),
           colored_light(mycolor::TEAL_50.into())));
  c.spawn((Location::default(),
           CameraTarget,
           Visuals::unlit_sprite(MySprite::WHITE_CORNERS)));
  // c.spawn((Location::default(), Player::default()));
  c.spawn(Player {}).insert(Location::new(5, 12, 5));

  c.spawn((Location::new(5, 12, 5), Visuals::sprite(MySprite::COFFEE)));
  c.spawn((Location::new(5, 12, 5), Visuals::sprite(MySprite::COFFEE)));
  c.spawn((Location::new(5, 12, 5), Visuals::sprite(MySprite::COFFEE)));
  c.spawn(basic_npc(Location::new(5, 12, 5), "Zorp", MySprite::ZORP));
  c.spawn(basic_npc(Location::new(5, 12, 5), "Zorp", MySprite::ZORP));
  c.spawn(basic_npc(Location::new(5, 12, 5), "Zorp", MySprite::ZORP));
  c.spawn(basic_npc(Location::new(5, 12, 5), "You", MySprite::PLAYER));

  c.spawn((Location::new(5, 12, 4), Visuals::sprite(MySprite::GATE)));
  c.spawn((Location::new(5, 12, 3), Visuals::sprite(MySprite::PORTAL)));
  c.spawn((Location::new(5, 12, 2), Visuals::sprite(MySprite::SPACEMAN)));
  c.spawn((Location::new(5, 12, 1), Visuals::sprite(MySprite::SPACEWIZARD)));
  c.spawn((Location::new(5, 12, 0), Visuals::sprite(MySprite::ICESTEROID)));
  c.spawn((Location::new(5, 12, -1), Visuals::sprite(MySprite::EVIL_ROBOT)));
  c.spawn((Location::new(5, 12, -2), Visuals::sprite(MySprite::BROWNGASGIANT)));
  c.spawn(torch(Location::new(4, 12, 5)));
  c.spawn(torch(Location::new(6, 12, 3)));
  c.spawn((Location::new(5, 12, 5), Visuals::sprite(MySprite::TREE)));
  spawn_world(&mut c, blocksparam);

  let fov = std::f32::consts::PI / 4.0;

  let pitch_upper_limit_radians = 1.0;
  let pitch_lower_limit_radians = 0.2;
  let camera = (
    Camera { hdr: true,

             ..default() },
    ColorGrading{..default()},
    Camera3d::default(),
    IsDefaultUiCamera,
    BLOOM,
    TONEMAPPING,
    Projection::Perspective(PerspectiveProjection { fov, ..default() }),
    Exposure { ev100: 10.0 },
    // Skybox { image: skybox_handle.clone(),
    //          brightness: 600.0 },
    Camera2d,
    VoxelWorldCamera::<MyMainWorld>::default(),
    Transform::from_xyz(10.0, 10.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
    // Camera3dBundle { camera: Camera { hdr: true,

    //                                   ..default() },
    //                  ..default() }
  );
  // let camera =
  //   (IsDefaultUiCamera,
  //    BLOOM,
  //    // Skybox { image: skybox_handle.clone(),
  //    //          brightness: 600.0 },
  //    Camera2d,
  //    // FOG_SETTINGS,
  //    VoxelWorldCamera::<MyMainWorld>::default(),
  //    Camera3dBundle { camera: Camera { hdr: true,

  //                                      ..default() },
  //                     transform:
  //                       Transform::from_xyz(10.0, 10.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y),

  //                     tonemapping: TONEMAPPING,
  //                     projection:
  //                       Projection::Perspective(PerspectiveProjection { fov, ..default() }),
  //                     exposure: bevy::render::camera::Exposure { ev100: 10.0 },
  //                     ..default() });
  c.spawn(camera);
  // Pointer
  // light
  c.spawn(PointLightBundle { point_light: PointLight { shadows_enabled: true,
                                                       ..default() },
                             transform: Transform::from_xyz(4.0, 8.0, 4.0),
                             ..default() });
  c.spawn(PointLightBundle { transform: Transform::from_translation(Vec3::new(25.0,
                                                                              25.0,
                                                                              25.0)),
                             point_light: PointLight { range: 200.0,
                                                       intensity: 8000.0,
                                                       ..Default::default() },
                             ..Default::default() });
  println("setup");
}
#[derive(Resource, Clone, Default)]
struct MyMainWorld;

impl VoxelWorldConfig for MyMainWorld {
  type MaterialIndex = BlockType;
  fn spawning_distance(&self) -> u32 { 10 }
  fn chunk_despawn_strategy(&self) -> ChunkDespawnStrategy {
    ChunkDespawnStrategy::default()
  }
  fn chunk_spawn_strategy(&self) -> ChunkSpawnStrategy { ChunkSpawnStrategy::default() }
  fn max_spawn_per_frame(&self) -> usize { 10000 }
  fn spawning_rays(&self) -> usize { 100 }
  fn spawning_ray_margin(&self) -> u32 { 25 }
  fn debug_draw_chunks(&self) -> bool { false }
  fn voxel_lookup_delegate(&self) -> VoxelLookupDelegate<Self::MaterialIndex> {
    Box::new(|_| Box::new(|_| WorldVoxel::Unset))
  }
  fn init_custom_materials(&self) -> bool { true }
  fn init_root(&self, mut _commands: Commands, _root: Entity) {}
  fn texture_index_mapper(&self)
                          -> Arc<dyn Fn(Self::MaterialIndex) -> [u32; 3] + Send + Sync> {
    Arc::new(|_mat| [0, 0, 0])
  }
  // fn texture_index_mapper(&self) -> Arc<dyn Fn(u8) -> [u32; 3] + Send + Sync> {
  //   // WorldVoxel
  //   Arc::new(|vox_mat: u8| {
  //     let block_type = BlockType::from(vox_mat);
  //     let textures = block_type.textures();
  //     let texture_indexes = textures.map(Into::into);
  //     texture_indexes
  //   })
  // }
  fn voxel_texture(&self) -> Option<(String, u32)> {
    Some(("block_textures.png".to_string(), BlockTexture::NUM as u32))
  }
  // fn voxel_texture(&self) -> Option<(String, u32)> {
  //   Some((
  //     "block_textures.png".into(),
  //     11 // BlockTexture::NUM as u32
  //   ))
  // }
}

impl From<BlockType> for WorldVoxel {
  fn from(block_type: BlockType) -> Self { WorldVoxel::Solid(u8::from(block_type)) }
}
// fn create_voxel_scene(mut voxel_world: VoxelWorld<MyMainWorld>) {
//   // Then we can use the `u8` consts to specify the type of voxel

//   // 20 by 20 floor
//   for x in -10..10 {
//     for z in -10..10 {
//       voxel_world.set_voxel(IVec3::new(x, -1, z), BlockType::Snow.into());
//       // Grassy floor
//     }
//   }

//   // Some bricks
//   voxel_world.set_voxel(IVec3::new(0, 0, 0), BlockType::Snow.into());
//   voxel_world.set_voxel(IVec3::new(0, 0, 0), BlockType::Snow.into());
//   voxel_world.set_voxel(IVec3::new(1, 0, 0), BlockType::Snow.into());
//   voxel_world.set_voxel(IVec3::new(0, 0, 1), BlockType::Snow.into());
//   voxel_world.set_voxel(IVec3::new(0, 0, -1), BlockType::Stone.into());
//   voxel_world.set_voxel(IVec3::new(-1, 0, 0), BlockType::Stone.into());
//   voxel_world.set_voxel(IVec3::new(-2, 0, 0), BlockType::Sand.into());
//   voxel_world.set_voxel(IVec3::new(-1, 1, 0), BlockType::Bricks.into());
//   voxel_world.set_voxel(IVec3::new(-2, 1, 0), BlockType::Snow.into());
//   voxel_world.set_voxel(IVec3::new(0, 1, 0), BlockType::Snow.into());
// }

// struct Spawnable(fn(&mut Commands, Location));

type Template = fn(&mut Commands, Vec3);
macro_rules! template {
  ($name:ident, $bundle:expr) => {
    static $name: Template = |c: &mut Commands, pos: Vec3| {
      c.spawn($bundle).insert(Transform::from_translation(pos));
    };
  };
}
template!(SNOWMAN, basic_animal("snowman", '⛄'));

comment! {
  macro_rules! create_spawnables {
    ($($name:ident => $body:expr),* $(,)?) => {
      impl Spawnable {
        $(
          pub const $name: Self = Self(|commands:&mut Commands, pos:Pos| {
            let bundle_without_pos = $body;
            commands.spawn((pos,bundle_without_pos));
          });
        )*
      }
    };
  }
  create_spawnables! {
    SNOWMAN =>  basic_animal( "snowman", '⛄'),
    SHEEP =>  basic_animal( "sheep", '🐑'),
    DUCK =>  basic_animal( "duck", '🦆'),
    RABBIT =>  basic_animal( "rabbit", '🐇'),

    // WALL =>  basic_tile( false, "#666666", "wall", Some('#')),
    // TREE =>  basic_tile( false, "#27AD00", "tree", Some('🌲')),
    // ROCK =>  basic_tile( false, "#71A269", "rock", Some('🪨')),
    // WATER =>  basic_tile( false, "#5961FF", "water", None),
    // SAND =>  basic_tile( true, "#D9DC60", "sand", None),
    // GRASS =>  basic_tile( true, "#22B800", "grass", None),
    ENEMY =>  {
      (
        name("enemy"),
        EnemyMovement,
        AttackPlayer,
        Combat { hp: 30, damage: 1 ,is_hostile:true},

        // SpaceObjectBundle::new( NORMAL_NPC_SCALE, true, Visuals::character('👿'))
      )
    },
    SPIDER =>  {
      (
        name("spider"),
        EnemyMovement,
        AttackPlayer,
        Combat { hp: 40, damage: 1 ,is_hostile:true},
        // SpaceObjectBundle::new( NORMAL_NPC_SCALE, true, Visuals::character('🕷'))
      )
    },
    DRAGON =>  {
      (
        name("dragon"),
        EnemyMovement,
        DragonAttack,
        AttackPlayer,
        Combat { hp: 60, damage: 1 ,is_hostile:true},
        // SpaceObjectBundle::new( NORMAL_NPC_SCALE, true, Visuals::character('🐉'))
      )
    },
    FIRE =>  {
      (
        name("fire"),
        Fire { dir: Dir::East },
        // SpaceObjectBundle::new( 1.0, false, Visuals::character('🔥'))
      )
    },
    // SPACE_PIRATE =>  {
    //   (IsHostile(true),
    //    scaled_enemy(
    //                 NORMAL_NPC_SCALE,
    //                 "space pirate",
    //                 NORMAL_NPC_THRUST,
    //                 Faction::SpacePirates,
    //                 50,
    //                 MySprite::SPACESHIPRED))
    // },
    // SPACE_PIRATE_BASE =>  {
    //   (Combat { hp: 120,
    //             is_hostile: false,
    //             ..default() },
    //    Interact::SingleOption(InteractSingleOption::Describe),
    //    name("space pirate base"),
    //    SpaceObjectBundle::new( 4.0, false, Visuals::sprite(MySprite::SPACEPIRATEBASE)))
    // },
    // SPACE_STATION =>  {
    //   (Combat { hp: 120,
    //             is_hostile: false,
    //             ..default() },
    //    Interact::SingleOption(InteractSingleOption::Describe),
    //    name("space station"),
    //    SpaceObjectBundle::new( 4.0, false, Visuals::sprite(MySprite::SPACESTATION)))
    // },
    // TRADER =>  {
    //   scaled_npc(
    //              NORMAL_NPC_SCALE,
    //              "Trader",
    //              NORMAL_NPC_THRUST,
    //              Faction::Traders,
    //              30,
    //              MySprite::SPACESHIPWHITE2)
    // },
    // SPACE_COP =>  {
    //   scaled_npc(
    //              NORMAL_NPC_SCALE,
    //              "space cop",
    //              NORMAL_NPC_THRUST,
    //              Faction::SpacePolice,
    //              70,
    //              MySprite::SPACESHIPBLUE)
    // },
    // SPACE_WIZARD =>  {
    //   scaled_npc(
    //              NORMAL_NPC_SCALE,
    //              "space wizard",
    //              NORMAL_NPC_THRUST,
    //              Faction::SPACEWIZARDs,
    //              40,
    //              MySprite::WIZARDSPACESHIP)
    // },
    // NOMAD =>  {
    //   scaled_npc(
    //              NORMAL_NPC_SCALE,
    //              "nomad",
    //              NORMAL_NPC_THRUST,
    //              Faction::Wanderers,
    //              35,
    //              MySprite::SPACESHIPGREEN)
    // },
    // ALIEN_SOLDIER =>  {
    //   (IsHostile(true),
    //    scaled_enemy(
    //                 NORMAL_NPC_SCALE,
    //                 "alien soldier",
    //                 NORMAL_NPC_THRUST,
    //                 Faction::Invaders,
    //                 80,
    //                 MySprite::PURPLEENEMYSHIP))
    // },
    // NPC =>  {
    //   scaled_npc(
    //              NORMAL_NPC_SCALE,
    //              "npc",
    //              NORMAL_NPC_THRUST,
    //              Faction::default(),
    //              50,
    //              MySprite::SPACESHIPWHITE2)
    // },
    // MUSHROOM_MAN =>  {
    //   (PlayerFollower,
    //    scaled_npc(
    //               NORMAL_NPC_SCALE,
    //               "mushroom man",
    //               NORMAL_NPC_THRUST,
    //               Faction::Traders,
    //               40,
    //               MySprite::MUSHROOMMAN))
    // },
    // SPACE_COWBOY =>  {
    //   talking_person_in_space(
    //                           MySprite::SPACECOWBOY,
    //                           "space cowboy",
    //                           Dialogue::SPACE_COWBOY)
    // },
    // // SIGN =>  {
    // //   (Interact::SingleOption(InteractSingleOption::Describe),
    // //    SpaceObjectBundle::new(
    // //                           1.5,
    // //                           false,
    // //                           Visuals::sprite(MySprite::SIGN).with_text(String::new())))
    // // },
    // WORMHOLE =>  {
    //   (Interact::SingleOption(InteractSingleOption::Describe),
    //    name("wormhole"),
    //    SpaceObjectBundle::new( 4.0, false, Visuals::sprite(MySprite::WORMHOLE)))
    // },
    // ASTEROID =>  {
    //   (Interact::MultipleOptions(InteractMultipleOptions::ASTEROIDMiningMinigame{resources_left:5,tool_durability:5}),
    //    CanBeFollowedByNPC,
    //    SpaceObjectBundle::new(
    //                           asteroid_scale(),
    //                           false,
    //                           Visuals::sprite(MySprite::ASTEROID)))
    // },
    // SPACE_CAT =>  {
    //   (Name::new("space cat"),
    //    Interact::SingleOption(InteractSingleOption::Item(Item::SPACECAT)),
    //    SpaceObjectBundle::new( 1.3, true, Visuals::sprite(MySprite::SPACECAT)))
    // },
    // SPACEMAN =>  {
    //   (Name::new("spaceman"),
    //    Interact::SingleOption(InteractSingleOption::Item(Item::Person)),
    //    SpaceObjectBundle::new( 1.3, true, Visuals::sprite(MySprite::SPACEMAN)))
    // },
    // SPACE_COIN =>  {
    //   (Name::new("space coin"),
    //    Interact::SingleOption(InteractSingleOption::Item(Item::SpaceCOIN)),
    //    SpaceObjectBundle::new( 1.7, true, Visuals::sprite(MySprite::COIN)))
    // },
    // ICE_ASTEROID =>  {
    //   (Name::new("ice"),
    //    Interact::SingleOption(InteractSingleOption::Item(Item::DiHydrogenMonoxide)),
    //    SpaceObjectBundle::new( asteroid_scale(), true, Visuals::sprite(MySprite::ICEASTEROID)))
    // },
    // CRYSTAL_ASTEROID =>  {
    //   (Name::new("crystal asteroid"),
    //    Interact::SingleOption(InteractSingleOption::Item(Item::Crystal)),
    //    SpaceObjectBundle::new( asteroid_scale(), true, Visuals::sprite(MySprite::CRYSTALASTEROID)))
    // },
    // CRYSTAL_MONSTER =>  {
    //   (name("crystal monster"),
    //    SpaceObjectBundle::new( 2.1, true, Visuals::sprite(MySprite::CRYSTALMONSTER)))
    // },
    // CRYSTAL_MONSTER_2 =>  {
    //   (name("crystal monster"),
    //    Interact::SingleOption(InteractSingleOption::Describe),
    //    SpaceObjectBundle::new( 1.7, true, Visuals::sprite(MySprite::CRYSTALMONSTER)))
    // },
    // HP_BOX =>  {
    //   (name("hp box"),
    //    Interact::SingleOption(InteractSingleOption::HPBOX),
    //    SpaceObjectBundle::new( 0.9, true, Visuals::sprite(MySprite::HPBOX)))
    // },
    // TREASURE_CONTAINER =>  {
    //   (name("container"),
    //    Interact::SingleOption(InteractSingleOption::CONTAINER(vec![(Item::SpaceCOIN, 4), (Item::COFFEE, 1)])),
    //    SpaceObjectBundle::new( 2.1, true, Visuals::sprite(MySprite::CONTAINER)))
    // },
    // SPHERICAL_COW =>  {
    //   (name("spherical cow"),
    //    Interact::dialogue_tree_default_state(Dialogue::SPHERICAL_SPACE_COW),
    //    // Interact::MultipleOptions(InteractMultipleOptions::DialogueTREE(SPHERICAL_SPACE_COW)),
    //    SpaceObjectBundle::new( 1.7, true, Visuals::sprite(MySprite::SPHERICALCOW)))
    // },
    // // ZORP =>  {
    // //   (name("zorp"),
    // //    Interact::MultipleOptions(InteractMultipleOptions::DialogueTREE(ZORP)),
    // //    SpaceObjectBundle::new( 1.7, true, Visuals::sprite(MySprite::ZORP)))
    // // },
    // TRADE_STATION =>  {
    //   let (trade, text) = if prob(0.5) {
    //     let trade_buy = pick([Item::DiHydrogenMonoxide, Item::Crystal, Item::SPACECAT]).unwrap();
    //     (Interact::SingleOption(InteractSingleOption::Trade { inputs: (trade_buy, 1),
    //                                                           outputs: (Item::SpaceCOIN, 5) }),
    //      format!("space station\nbuys {:?}", trade_buy))
    //   } else {
    //     let trade_sell = pick([Item::Spice, Item::COFFEE, Item::Rock]).unwrap();
    //     (Interact::SingleOption(InteractSingleOption::Trade { inputs: (Item::SpaceCOIN, 5),
    //                                                           outputs: (trade_sell, 1) }),
    //      format!("space station\nsells {:?}", trade_sell))
    //   };
    //   (name("space station"),
    //    CanBeFollowedByNPC,
    //    trade,
    //    SpaceObjectBundle::new(
    //                           3.0,
    //                           false,
    //                           Visuals::sprite(MySprite::SPACESTATION).with_text(text)))
    // },
    // FLOATING_ISLAND =>  {
    //   (name("floating island"),
    //    Interact::SingleOption(InteractSingleOption::Describe),
    //    SpaceObjectBundle::new( 3.4, false, Visuals::sprite(MySprite::FLOATINGISLAND)))
    // },
    // ABANDONED_SHIP =>  {
    //   (name("abandoned ship"),
    //    Interact::MultipleOptions(InteractMultipleOptions::Salvage { how_much_loot: 3 }),
    //    SpaceObjectBundle::new(
    //                           2.0,
    //                           false,
    //                           Visuals::sprite(MySprite::SPACESHIPABANDONED)))
    // },
  }
}
type ASDF = fn(u32, u32, [u32; 123]) -> u32;
type ASAASDF = [bool; 123];

comment! {
  #[derive(Clone)]
  enum InteractMultipleOptions {
    Salvage {
      how_much_loot: u8
    },
    DialogueTREE(DialogueTree, &'static str),
    ASTEROIDMiningMinigame {
      resources_left: u8,
      tool_durability: u8
    }
  }
  impl InteractMultipleOptions {
    fn interact(self) -> (String, Vec<(String, MyCommand, Self)>) {
      match self {
        InteractMultipleOptions::ASTEROIDMiningMinigame { resources_left,
                                                          tool_durability } => {
          let msg =
            format!("You're mining an asteroid. Resources left: {}. Tool durability: {}.",
                    resources_left, tool_durability);
          let mut options = vec![];

          if resources_left > 0 && tool_durability > 0 {
            options.push((
              "Mine carefully".to_string(),
              MyCommand::multi([
                MyCommand::message_add("You mine carefully, preserving your tool."),
                MyCommand::give_item_to_player(Item::SpaceMinerals),
              ]),
              Self::ASTEROIDMiningMinigame { resources_left: resources_left - 1, tool_durability },
            ));
            options.push((
              "Mine aggressively".to_string(),
              MyCommand::multi([
                MyCommand::message_add("You mine aggressively, risking your tool for more resources."),
                MyCommand::give_item_to_player(Item::SpaceMinerals),
                MyCommand::give_item_to_player(Item::SpaceMinerals),
              ]),
              Self::ASTEROIDMiningMinigame { resources_left: resources_left - 1, tool_durability: tool_durability - 1 },
            ));
          }

          options.push(("Leave asteroid".to_string(),
                        MyCommand::end_object_interaction_mini_game(),
                        self.clone()));

          (msg, options)
        }
        InteractMultipleOptions::Salvage { how_much_loot } => {
          let msg = "It's a destroyed spaceship. Maybe you can find loot in it".to_string();
          let options = if how_much_loot > 0 {
            vec![("take some".to_string(),
                  MyCommand::multi([MyCommand::message_add("You found loot"),
                                    MyCommand::give_item_to_player(Item::SpaceCOIN)]),
                  Self::Salvage { how_much_loot: how_much_loot - 1 }),
                 ("don't take".to_string(), MyCommand::none(), self.clone()),
                 ("leave".to_string(),
                  MyCommand::end_object_interaction_mini_game(),
                  self.clone()),]
          } else {
            vec![("leave".to_string(),
                  MyCommand::end_object_interaction_mini_game(),
                  self.clone()),]
          };
          (msg, options)
        }
        InteractMultipleOptions::DialogueTREE(tree, node) => {
          let msg = "talking npc".to_string();
          if let Some((_, options)) = tree.iter().find(|(node2, options)| *node2 == node) {
            let options = options.iter().map(|(new_node, playersay, npcsay, effect)| {
              (playersay.to_string(),
               MyCommand::message_add(npcsay.to_string()),
               InteractMultipleOptions::DialogueTREE(tree,
                                                     *new_node))
            });
            (msg, options.collect())
          } else {
            (msg, default())
          }
        } // InteractMultipleOptions::SPHERICALCOWDialogueTREE { node } => {
        //   let msg = "It's a spherical cow in a vacuum".to_string();
        //   let options = node.options()
        //                     .into_iter()
        //                     .map(|(node, playersay, cowsay)| {
        //                       (playersay.to_string(),
        //                        MyCommand::message_add(cowsay),
        //                        InteractMultipleOptions::SPHERICALCOWDialogueTREE { node })
        //                     })
        //                     .collect();
        //   (msg, options)
        // }
      }
    }
  }

  #[derive(Clone)]
  enum InteractSingleOption {
    Message(String),
    // Salvage { how_much_loot: u8 },
    ASTEROID,
    HPBOX,
    Describe,
    Item(Item),
    Trade {
      inputs: (Item, u32),
      outputs: (Item, u32)
    },
    GATE(Vec3),
    CONTAINER(Vec<(Item, u32)>)
  }

  impl InteractSingleOption {
    fn interact(self,
                self_entity: Entity,
                self_name: String,
                player_inventory: &Inventory)
                -> (String, MyCommand) {
      match self {
        InteractSingleOption::Message(m) => ("examine".to_string(), MyCommand::message_add(m)),
        InteractSingleOption::ASTEROID => {
          (format!("examine {self_name}"),
           MyCommand::message_add("it's an asteroid"))
        }
        InteractSingleOption::HPBOX => {
          ("take hp box".to_string(),
           MyCommand::multi([MyCommand::update_player_component(|combat: Combat| {
             Combat { hp: combat.hp + 50,
                      ..combat }
           }),
                             MyCommand::despawn(self_entity)]))
        }
        InteractSingleOption::Describe => {
          (format!("examine {self_name}"), MyCommand::message_add(self_name))
        }
        InteractSingleOption::Item(item) => {
          (format!("take {self_name}"),
           MyCommand::multi([MyCommand::despawn(self_entity),
                             MyCommand::message_add(format!("You got a {}",debugfmt(item)) ),
                             MyCommand::give_item_to_player(item)]))
        }

        InteractSingleOption::Trade { inputs: (input_item, input_number),
                                      outputs: (output_item, output_number) } => {
          ("trade".to_string(),
           if let Some(&n) = player_inventory.0.get(&input_item)
           && n >= input_number
           {
             MyCommand::multi([
               MyCommand::mutate_player_component(move |mut inventory:&mut Inventory|{
                 inventory.trade([(input_item, input_number)],[(output_item, output_number)]);
               }),
               MyCommand::message_add(format!("You traded {:?} {:?} for {:?} {:?}s",
                                              input_number,
                                              input_item,
                                              output_number,
                                              output_item))
             ])
           } else {
             MyCommand::message_add("You don't have enough items")
           })
        }
        InteractSingleOption::GATE(destination_pos) => {
          ("interact".to_string(),
           MyCommand::update_player_component(move |transform| Transform { translation:
                                                                           destination_pos,
                                                                           ..transform }))
        }
        InteractSingleOption::CONTAINER(items) => {
          ("take container".to_string(),
           MyCommand::multi([MyCommand::despawn(self_entity),
                             MyCommand::message_add("you got things"),
                             MyCommand::mutate_player_component(|mut inventory: &mut Inventory| {
                               inventory.add_contents(items);
                             })]))
        }
      }
    }
  }

  #[derive(Component, Clone)]
  enum Interact {
    SingleOption(InteractSingleOption),
    MultipleOptions(InteractMultipleOptions)
  }

  impl Interact {
    fn dialogue_tree_default_state(tree: DialogueTREE) -> Self {
      let (node, _) = tree[0];
      Self::MultipleOptions(InteractMultipleOptions::DialogueTREE(tree, node))
    }

  }
}
#[bevy_main]
pub fn main() {
  let voxel_material = MyMaterial::BLOCKS.val();
  let address_mode = ImageAddressMode::ClampToBorder;
  // let default_sampler = ImageSamplerDescriptor { mag_filter: ImageFilterMode::Nearest,
  //                                                min_filter: ImageFilterMode::Linear,
  //                                                mipmap_filter: ImageFilterMode::Linear,
  //                                                // address_mode_u: address_mode,
  //                                                //                        address_mode_v: address_mode,
  //                                                //                        address_mode_w: address_mode,
  //                                                // compare:
  //                                                //   Some(ImageCompareFunction::Less),
  //                                                // lod_min_clamp: 10.0,
  //                                                // lod_max_clamp: 100.0,
  //                                                // border_color:
  //                                                //   Some(ImageSamplerBorderColor::TransparentBlack),
  //                                                // anisotropy_clamp: 1000,
  //                                                ..default() };

  App::new()
    .add_plugins((
      EmbeddedAssetPlugin::default(),
      // bevy::pbr::ScreenSpaceAmbientOcclusionPlugin
      DefaultPlugins
      // .set(bevy::render::RenderPlugin {

      //   render_creation: bevy::render::settings::RenderCreation::Automatic(bevy::render::settings::WgpuSettings {
      //     features: bevy::render::settings::WgpuFeatures::POLYGON_MODE_LINE,
      //     backends: Some(bevy::render::settings::Backends::DX12),
      //     ..default()
      //   }),
      //   ..default()
      // })
        // .set(ImagePlugin{default_sampler})
        .set(WindowPlugin {
          primary_window: Some(Window {
            // resolution: WindowResolution


            mode:WindowMode::Windowed,


            present_mode: bevy::window::PresentMode::AutoVsync,
            title: "bevy spooky game".to_string(),
            canvas: Some("#bevy".to_string()),
            ..default()
          }),
          ..default()
        }),

      VoxelWorldPlugin::with_config(MyMainWorld),
      // bevy_vox_scene::VoxScenePlugin,
      bevy_sprite3d::Sprite3dPlugin,
      // bevy_panorbit_camera::PanOrbitCameraPlugin,
      // bevy_mod_billboard::prelude::BillboardPlugin,
      // QuillPlugin,
      // QuillOverlaysPlugin,
    ))
    .init_state::<GameState>()
    .init_resource::<PressedKeys>()
    .init_resource::<UIData>()
    .init_resource::<TimeTicks>()

    // .init_resource::<WorldLocationMap>()
    .insert_resource(ClearColor(mycolor::CLEAR))
    .insert_resource(AMBIENT_LIGHT)
    // .insert_resource(Msaa::Sample4)
    .add_systems(Startup, (setup,
|world: &mut World| {
  ui_root().spawn(world);
                },
    ))

    .add_systems(Update,
                 ((
                   get_pressed_keys,
                   // timed_animation_system,
                 ).chain(),
                  (set_prev_loc,
                   player_movement,
                   random_movement,
                   try_movement,
                   movement).chain().run_if(is_sim_frame_condition),
                  position_sprite_billboards,
                  camera_follow_player,
                  visuals,
                  // ui,
                  sun_movement,
                  close_on_esc,
                  origin_time,
                  increment_time,
                 ).chain())
    // .add_systems(Update,((
    //   sync_locations_new,
    //   set_frame_timestamp,
    //   random_movement,
    // ).run_if(every_n_ticks::<TICK_TIME>),

    //   position_sprite_billboards,
    //   // proximity_system,
    //   visuals,
    //   ui,
    // ).chain())
    // .add_systems(Update,(
    //   position_sprite_billboards,
    //   // proximity_system,
    //   visuals,
    //   ui,
    // ).chain())
    .run();
}

// trunk build --release --public-url "bevyspookygame" --filehash false

// trunk serve

// cargo check --target wasm32-unknown-unknown
// cargo run --target x86_64-unknown-linux-gnu
// cargo check --target x86_64-unknown-linux-gnu
// cargo build --target x86_64-pc-windows-gnu --release
