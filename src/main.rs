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
// #![feature(impl_trait_existential_types)]
// #![feature(option_get_or_insert_default)]
#![feature(let_chains)]
// #![feature(const_mut_refs)]

// #![feature(int_roundings)]
// #![recursion_limit = "1024"]
// #![feature(const_fn_floating_point_arithmetic)]

// pub mod bundletree;
// pub mod ui;
mod dialogue;
mod mycommand;
use {dialogue::Dialogue, enum_assoc};

// use crate::dialogue::DialogueTree
pub use bevy::prelude::Name;
use {avian3d::prelude::*,
     bevy::{app::AppExit,
            asset::{AssetServer, Handle},
            core_pipeline::bloom::{BloomCompositeMode, BloomPrefilterSettings,
                                   BloomSettings},
            math::{primitives, vec3},
            pbr::StandardMaterial,
            prelude::*,
            render::texture::{ImageAddressMode, ImageFilterMode, ImageSamplerDescriptor},
            utils::{HashMap, HashSet},
            window::WindowMode},
     bevy_embedded_assets::*,
     bevy_panorbit_camera::PanOrbitCamera,
     bevy_quill::{prelude::*, QuillPlugin, ViewChild},
     bevy_quill_overlays::QuillOverlaysPlugin,
     bevy_voxel_world::prelude::*,
     dynamics::solver::SolverConfig,
     enum_assoc::Assoc,
     fancy_constructor::new,
     mycommand::MyCommand,
     rand::{random, thread_rng},
     rust_utils::*,
     std::{f32::consts::PI, mem::variant_count, sync::Arc}};
// ui::UIData

mod mycolor {
  use bevy::color::Color;

  pub const AMBIENT_LIGHT: Color = Color::hsv(301.0, 1.0, 1.0);
  pub const CLEAR: Color = Color::hsv(301.0, 1.0, 0.07);

  pub const GLOWY: Color = Color::srgb(13.99, 11.32, 50.0);
  pub const GLOWY_2: Color = Color::srgb(30.0, 20.7, 10.5);
  pub const GLOWY_3: Color = Color::srgb(0.0, 30.0, 0.0);
  pub const EXPLOSION: Color = Color::srgb(8.0, 3.0, 3.0);
  pub const LASER: Color = Color::hsv(60.0, 1.0, 4.0);
  // hsv(61, 100%, 100%)
}
// pub const AMBIENT_LIGHT_COLOR: Color = Color::hsv(301.0, 1.0, 1.0);
// pub const CLEAR_COLOR: Color = Color::hsv(301.0, 1.0, 0.07);

// pub const GLOWY_COLOR: Color = Color::srgb(13.99, 11.32, 50.0);
// pub const GLOWY_COLOR_2: Color = Color::srgb(30.0, 20.7, 10.5);
// pub const GLOWY_COLOR_3: Color = Color::srgb(0.0, 30.0, 0.0);
// pub const EXPLOSION_COLOR: Color = Color::srgb(8.0, 3.0, 3.0);
// pub const LASER_COLOR: Color = Color::hsv(60.0, 1.0, 4.0);
// hsv(61, 100%, 100%)
pub const BILLBOARD_REL_SCALE: f32 = 2.0;
pub const TEXT_SCALE: f32 = 0.013;
pub const ENABLE_SHADOWS_OTHER_THAN_SUN: bool = false;

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

fn array_range<const LEN: usize>() -> [usize; LEN] {
  let mut arr = [0; LEN];
  for i in 0..LEN {
    arr[i] = i;
  }
  arr
}
fn prob(p: f32) -> bool { p > rand::random::<f32>() }
pub fn cuboid_full_iter(lower_corner: IVec3,
                        side_lengths: IVec3)
                        -> impl Iterator<Item = IVec3> {
  let mut v = Vec::new();
  for x in 0..side_lengths.x {
    for y in 0..side_lengths.y {
      for z in 0..side_lengths.z {
        v.push(lower_corner + IVec3 { x, y, z })
      }
    }
  }
  v.into_iter()
}
pub fn sphere_full_iter(center: IVec3, radius: i32) -> impl Iterator<Item = IVec3> {
  cuboid_full_iter(center - IVec3::splat(radius),IVec3::splat(radius * 2)).filter(move |v: &IVec3| v.distance_squared(center) <= radius.pow(2))
}
#[derive(Component, Clone, PartialEq, Eq, Default)]
pub struct Visuals {
  text: Option<String>,
  material_mesh: Option<(MyMaterial, GenMesh)>,
  sprite: Option<MySprite>,
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

  let text_style = TextStyle { font_size: 30.0,
                               ..default() };
  let invisible_material = get_material_handle(MyMaterial::INVISIBLE);

  for (entity, mut visuals) in &mut visuals_q {
    if visuals.is_changed() || !visuals.done {
      visuals.done = true;
      *n += 1;
      if *n % 100 == 0 {
        println!("{}", *n);
      }

      let main_visual_child = *visual_child_entities.entry(entity).or_insert_with(|| {
                                                                    c.spawn((
                    PbrBundle {
                        material: invisible_material.clone(),
                        mesh: get_mesh_handle(GenMesh::SPHERE),
                        ..default()
                    },
                ))
                .set_parent(entity)
                .id()
                                                                  });

      c.entity(main_visual_child).despawn_descendants();

      if let Some(text) = visuals.text.clone() {
        c.spawn(Text2dBundle {
                    text: Text::from_section(text, text_style.clone()),
                    transform: Transform::from_xyz(0.0, 1.5, 0.0).with_scale(Vec3::splat(0.07)),
                    ..default()
                })
                .set_parent(main_visual_child);
      }

      if let Some(sprite) = visuals.sprite {
        let sprite_handle = get_sprite_handle(sprite);
        // sprite_3d_params.images.get(image_handle.clone())
        if let Some(image) = sprite_3d_params.images.get(&sprite_handle) {
          let image_height = image.height();
          c.spawn((VisualSprite,
                   bevy_sprite3d::Sprite3d { image: sprite_handle,
                                             pixels_per_metre: image_height as f32,
                                             double_sided: true,
                                             alpha_mode: AlphaMode::Blend,
                                             unlit: visuals.unlit,
                                             transform: Transform::from_xyz(0.0, 0.0,
                                                                            0.0),
                                             ..default() }.bundle(&mut sprite_3d_params)))
           .set_parent(main_visual_child);
        } else {
          visuals.done = false;
        }
      }

      if let Some((material, gen_mesh)) = visuals.material_mesh {
        let material = get_material_handle(material);
        let mesh = get_mesh_handle(gen_mesh);
        c.spawn(PbrBundle { material,
                            mesh,
                            ..default() })
         .set_parent(main_visual_child);
      }
    }
  }
}
#[derive(Component, Clone)]
struct FaceCamera;
pub fn face_camera(camq: Query<&GlobalTransform, With<Camera3d>>,
                   mut camera_facers_q: Query<(&mut Transform, &GlobalTransform),
                         (With<FaceCamera>, Without<Camera3d>)>) {
  if let Ok(cam_globaltransform) = camq.get_single() {
    for (mut transform, globaltransform) in &mut camera_facers_q {
      let dir = Vec3 { y: 0.0,
                       ..(globaltransform.translation()
                          - cam_globaltransform.translation()) };
      transform.look_to(dir, Vec3::Y);
    }
  }
}
#[derive(Component, Default, Clone, Copy, Debug)]
struct Pos(pub IVec3);

impl From<Pos> for Vec3 {
  fn from(Pos(iv): Pos) -> Self { iv.as_vec3() }
}

fn block_entity(pos: Pos, block_type: BlockType) -> impl Bundle { (pos, block_type) }
// const NORMAL_NPC_SCALE: f32 = 1.9;
// const NORMAL_NPC_THRUST: f32 = 400.0;
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
enum Dir {
  North,
  Northeast,
  East,
  Southeast,
  South,
  Southwest,
  West,
  Northwest,
  Here
}
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

// #[derive(Bundle, Clone)]
// pub struct SpaceObjectBundle((SpaceObject,
//                                Visuals,
//                                LockedAxes,
//                                ColliderMassProperties,
//                                Collider,
//                                RigidBody,
//                                LinearDamping,
//                                AngularDamping,
//                                LinearVelocity,
//                                AngularVelocity,
//                                ExternalForce,
//                                ExternalImpulse,
//                                SpatialBundle));
// impl SpaceObjectBundle {
//   fn new(translation: IVec3, scale: f32, can_move: bool, visuals: Visuals) -> Self {
//     let collider = Collider::sphere(1.0);
//     Self((SpaceObject { scale, ..default() },
//           visuals,
//           LockedAxes::ROTATION_LOCKED,
//           ColliderMassProperties::new(&collider, 1.0),
//           collider,
//           if can_move {
//             RigidBody::Dynamic
//           } else {
//             RigidBody::Static
//           },
//           LinearDamping(1.6),
//           AngularDamping(1.2),
//           LinearVelocity::default(),
//           AngularVelocity::default(),
//           ExternalForce::default().with_persistence(false),
//           ExternalImpulse::default(),
//           SpatialBundle { transform: Transform { translation,
//                                                  rotation: default(),
//                                                  scale: Vec3::splat(scale) },
//                           ..default() }))
//   }
//   fn sprite(translation: Vec3, scale: f32, can_move: bool, sprite: MySprite) -> Self {
//     Self::new(translation, scale, can_move, Visuals::sprite(sprite))
//   }
// }

const INTERACTION_RANGE: f32 = 8.0;
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
struct Spawnable(fn(&mut Commands, Pos));
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
struct Minable {
  item: Item,
  labor: Labor
}
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
fn make_tool(mat: Material, kind: Tool) -> Item { Item::Tool { mat, kind } }

fn make_weapon(mat: Material, kind: Weapon) -> Item { Item::Weapon { mat, kind } }
#[derive(Clone, Copy)]
enum Labor {
  Mine,
  Woodcut,
  Mason,
  Smith
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
fn granite() -> impl Bundle { block(Material::Stone(Stone::Granite)) }
fn marble() -> impl Bundle { block(Material::Stone(Stone::Marble)) }
fn gold_ore() -> impl Bundle { block(Material::Metal(Metal::Gold)) }

fn granite_table() -> impl Bundle {
  furniture(Material::Stone(Stone::Granite), Furniture::Table)
}

fn oak_bed() -> impl Bundle { furniture(Material::Wood(Wood::Oak), Furniture::Bed) }

fn basic_animal(nameval: &'static str, ch: char) -> impl Bundle {
  (name(nameval), RandomMovement, Visuals::character(chardisplay))
}
fn snowman() -> impl Bundle { basic_animal("snowman", '⛄') }
fn sheep() -> impl Bundle { basic_animal("sheep", '🐑') }
fn duck() -> impl Bundle { basic_animal("duck", '🦆') }
fn rabbit() -> impl Bundle { basic_animal("rabbit", '🐇') }

fn player() -> impl Bundle {
  (name("You"),
   Player {},
   EnemyMovement,
   AttackPlayer,
   Combat { hp: 30,
            damage: 1,
            is_hostile: true },
   Visuals::
   // SpaceObjectBundle::new(NORMAL_NPC_SCALE, true, Visuals::character('👿'))
  )
}
fn enemy() -> impl Bundle {
  (name("enemy"),
   EnemyMovement,
   AttackPlayer,
   Combat { hp: 30,
            damage: 1,
            is_hostile: true },
   SpaceObjectBundle::new(NORMAL_NPC_SCALE, true, Visuals::character('👿')))
}

fn spider() -> impl Bundle {
  (name("spider"),
   EnemyMovement,
   AttackPlayer,
   Combat { hp: 40,
            damage: 1,
            is_hostile: true },
   SpaceObjectBundle::new(NORMAL_NPC_SCALE, true, Visuals::character('🕷')))
}

fn fire() -> impl Bundle {
  (name("fire"),
   Fire { dir: Dir::East },
   SpaceObjectBundle::new(1.0, false, Visuals::character('🔥')))
}
fn monster(name: &'static str, ch: char, hp: i32, dmg: i32) -> impl Bundle {
  (Name(name), Char(ch), Combat { hp, dmg })
}
fn block(name: &'static str, color: &str) -> impl Bundle {
  (Name(name), Tile { bg: Color::hex(color).unwrap() })
}
fn craftsman(name: &'static str, ch: char, job: &'static str) -> impl Bundle {
  (entity(name, ch), Job(job), Container)
}
fn mage(name: &'static str, ch: char, mana: i32) -> impl Bundle {
  (monster(name, ch, 50, 5), Magic { mana })
}
fn gold_ore() -> impl Bundle { (block("gold ore", "FFD700"), Ore { value: 100 }) }
fn farmland() -> impl Bundle { (block("farmland", "5A3A1A"), Grows(WHEAT)) }
fn magma() -> impl Bundle {
  (block("magma", "FF4500"), Fluid { flow: 0.5 }, Burns { temp: 2000 }, Light(8.0))
}

// Creature Templates
fn dwarf() -> impl Bundle {
  (craftsman("dwarf", '🧔', "miner"), Combat { hp: 100, dmg: 8 }, Digger { speed: 1.0 })
}

fn wizard() -> impl Bundle { mage("wizard", '🧙', 100) }
fn dragon() -> impl Bundle {
  (name("dragon"),
   EnemyMovement,
   DragonAttack,
   AttackPlayer,
   Combat { hp: 60,
            damage: 1,
            is_hostile: true },
   SpaceObjectBundle::new(NORMAL_NPC_SCALE, true, Visuals::character('🐉')))
}
fn troll() -> impl Bundle { (monster("troll", '👹', 200, 30), Burns { temp: -10 }) }
fn goblin() -> impl Bundle { (monster("goblin", '👺', 40, 8), Container) }

// Item Templates
fn pickaxe() -> impl Bundle {
  (entity("pickaxe", '⛏'), Tool { durability: 100 }, Craftable { skill: 5 })
}
fn wheat() -> impl Bundle { (entity("wheat", '🌾'), Grows(WHEAT_MATURE)) }
fn wheat_mature() -> impl Bundle { (entity("mature wheat", '🌾'), Craftable { skill: 1 }) }
fn potion() -> impl Bundle {
  (entity("potion", '🧪'), Magic { mana: 50 }, Craftable { skill: 8 })
}
fn ring() -> impl Bundle { (entity("ring", '💍'), Magic { mana: 100 }, Light(3.0)) }

// Machines/Workshops
fn brewing_stand() -> impl Bundle {
  (block("brewing stand", "8B4513"), Container, Craftable { skill: 3 })
}
fn anvil() -> impl Bundle {
  (block("anvil", "4A4A4A"), Tool { durability: 1000 }, Craftable { skill: 10 })
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u8)]
pub enum BlockTexture {
  Bricks,
  Grass,
  Rocks,
  Snow,
  Stone,
  Sand,
  Dirt
}
impl BlockTexture {
  pub const NUM: usize = variant_count::<Self>();
}
impl From<u8> for BlockTexture {
  fn from(index: u8) -> Self {
    unsafe { std::mem::transmute(index) }
  }
}
impl From<BlockTexture> for u8 {
  fn from(block_texture: BlockTexture) -> u8 { block_texture as u8 }
}
impl From<BlockTexture> for u32 {
  fn from(block_texture: BlockTexture) -> u32 { block_texture as u32 }
}
#[derive(Clone, Copy, PartialEq, Eq, Debug, Assoc, Component)]
#[func(pub const fn textures(&self) -> [BlockTexture; 3])]
#[repr(u8)]
pub enum BlockType {
  #[assoc(textures = [BlockTexture::Bricks; 3])]
  Bricks,
  #[assoc(textures = [BlockTexture::Grass, BlockTexture::Grass, BlockTexture::Dirt])]
  Grass,
  #[assoc(textures = [BlockTexture::Rocks; 3])]
  Rocks,
  #[assoc(textures = [BlockTexture::Snow, BlockTexture::Snow, BlockTexture::Dirt])]
  Snow,
  #[assoc(textures = [BlockTexture::Stone; 3])]
  Stone,
  #[assoc(textures = [BlockTexture::Sand; 3])]
  Sand,
  #[assoc(textures = [BlockTexture::Dirt; 3])]
  Dirt
}
impl BlockType {
  pub const NUM: usize = variant_count::<Self>();
}
impl From<u8> for BlockType {
  fn from(index: u8) -> Self {
    unsafe { std::mem::transmute(index) }
  }
}
impl From<BlockType> for u8 {
  fn from(block_type: BlockType) -> u8 { block_type as u8 }
}
impl From<BlockType> for u32 {
  fn from(block_type: BlockType) -> u32 { block_type as u32 }
}

impl BlockType {
    pub fn is_solid(&self) -> bool {
        match self {
            BlockType::Air => false,
            _ => true,
        }
    }
    pub fn is_mineable(&self) -> bool {
        matches!(self, BlockType::Stone | BlockType::Dirt | BlockType::Sand)
    }
}
#[derive(Resource)]
pub struct GameWorld {
    blocks: HashMap<IVec3, BlockType>,
    entities: HashMap<IVec3, Vec<Entity>>,
}

impl Default for GameWorld {
    fn default() -> Self {
        Self {
            blocks: HashMap::new(),
            entities: HashMap::new(),
        }
    }
}

impl GameWorld {
    pub fn set_block(&mut self, pos: IVec3, block: BlockType) {
        match block {
            BlockType::Air => { self.blocks.remove(&pos); }
            _ => { self.blocks.insert(pos, block); }
        }
    }

    pub fn get_block(&self, pos: IVec3) -> BlockType {
        self.blocks.get(&pos).copied().unwrap_or(BlockType::Air)
    }

    pub fn is_walkable(&self, pos: IVec3) -> bool {
        !self.get_block(pos).is_solid()
    }

    // Entity position tracking
    pub fn move_entity(&mut self, entity: Entity, from: IVec3, to: IVec3) {
        // Remove from old position
        if let Some(entities) = self.entities.get_mut(&from) {
            entities.retain(|&e| e != entity);
            if entities.is_empty() {
                self.entities.remove(&from);
            }
        }

        // Add to new position
        self.entities.entry(to)
            .or_insert_with(Vec::new)
            .push(entity);
    }

    pub fn get_entities_at(&self, pos: IVec3) -> &[Entity] {
        self.entities.get(&pos).map(|v| v.as_slice()).unwrap_or(&[])
    }
}

// Simple component for moving entities
#[derive(Component)]
pub struct Position(pub IVec3);

// System to sync positions with world state
pub fn sync_positions(
    mut world: ResMut<GameWorld>,
    query: Query<(Entity, &Position), Changed<Position>>,
) {
    for (entity, pos) in query.iter() {
        // We only track the current position, no need to store old one
        world.move_entity(entity, pos.0, pos.0);
    }
}

// Simple movement system example
pub fn move_entities(
    world: Res<GameWorld>,
    mut query: Query<(&mut Position, &Movement)>,
    time: Res<Time>,
) {
    for (mut pos, movement) in query.iter_mut() {
        let target = pos.0 + movement.direction;
        if world.is_walkable(target) {
            pos.0 = target;
        }
    }
}

// Plugin to organize everything
pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<GameWorld>()
           .add_systems(Update, (
               sync_positions,
               move_entities,
           ));
    }
}


#[derive(Component, Clone, Copy, Debug, Eq, PartialEq)]
pub struct Location(pub IVec3);
impl Location {
  pub fn new(x: i32, y: i32, z: i32) -> Self { Self(IVec3::new(x, y, z)) }

  pub fn manhattan_distance(&self, other: &Location) -> i32 {
    let diff = self.0 - other.0;
    diff.x.abs() + diff.y.abs() + diff.z.abs()
  }
}

#[derive(Clone)]
enum Tile {
  Wall(Material),
  // Floor(Material),
  Open {
    floor: Option<Material>,
    entities: Vec<Entity>
  }
}
#[derive(Default, Resource)]
struct Blocks(HashMap<IVec3, BlockType>);
#[derive(Resource)]
pub struct BlockWorld {
  blocks: HashMap<IVec3, Tile>,
  // Cache of entities by position for quick lookups
  position_cache: HashMap<Entity, IVec3>
}

// System to keep Location components in sync with WorldMap
pub fn sync_entity_locations(mut commands: Commands,
                             mut world_map: ResMut<WorldMap>,
                             mut moved_query: Query<(Entity, &mut Location),
                                   Changed<Location>>) {
  for (entity, mut location) in moved_query.iter_mut() {
    let old_pos = location.0;
    // Remove from old position
    world_map.remove_entity(old_pos, entity);
    // Add to new position if it's walkable
    if world_map.is_walkable(location.0) {
      world_map.add_entity(location.0, entity);
    } else {
      // If the new position isn't walkable, revert the location
      location.0 = old_pos;
      world_map.add_entity(old_pos, entity);
    }
  }
}

// System to cleanup entities when they're despawned
pub fn cleanup_despawned_entities(mut world_map: ResMut<WorldMap>,
                                  removed_entities: RemovedComponents<Location>,
                                  query: Query<&Location>) {
  for entity in removed_entities.iter() {
    // We only need to cleanup if the entity had a Location
    if let Ok(location) = query.get(entity) {
      world_map.remove_entity(location.0, entity);
    }
  }
}

// Helper system to ensure Transform and Location stay in sync
pub fn sync_transforms(mut query: Query<(&Transform, &mut Location), Changed<Transform>>) {
  for (transform, mut location) in query.iter_mut() {
    location.0 = IVec3::new(transform.translation.x as i32,
                            transform.translation.y as i32,
                            transform.translation.z as i32);
  }
}

// Plugin to organize all the systems
#[derive(Default)]
pub struct WorldPlugin;

impl Plugin for WorldPlugin {
  fn build(&self, app: &mut App) {
    app.init_resource::<WorldMap>()
           .add_systems(Update, (
               sync_entity_locations,
               cleanup_despawned_entities,
               sync_transforms,
           ).chain());
  }
}

// Example usage in game code:
fn spawn_entity(commands: &mut Commands, pos: IVec3, world_map: &mut WorldMap) -> Entity {
  // Only spawn if the position is walkable
  if !world_map.is_walkable(pos) {
    panic!("Attempted to spawn entity in non-walkable position");
  }

  let entity = commands.spawn((
    Location(pos),
    Transform::from_translation(Vec3::new(pos.x as f32, pos.y as f32, pos.z as f32)) // ... other components
  ))
                       .id();

  world_map.add_entity(pos, entity);
  entity
}

#[derive(Component)]
struct Furniture;
fn furniture(mat: Material, kind: Furniture) -> impl Bundle {
  (Tile::Furniture(kind, mat), Furniture)
}

fn forge(mat: Material) -> impl Bundle {
  (name("forge"), Tile::Furniture(Furniture::Forge, mat), Container(vec![]), Quality(0))
}

impl Default for BlockWorld {
  fn default() -> Self {
    Self { blocks: HashMap::new(),
           position_cache: HashMap::new() }
  }
}

// Spatial relationship queries
impl BlockWorld {
  pub fn new() -> Self { Self { blocks: HashMap::new() } }
  pub fn set_block(&mut self, pos: IVec3, material: Material) {
    self.blocks.insert(pos, Tile::Solid(material));
  }
  pub fn set_furniture(&mut self, pos: IVec3, furniture: Furniture, material: Material) {
    self.blocks
        .insert(pos, Tile::Furniture(furniture, material));
  }
  pub fn remove_block(&mut self, pos: IVec3) {
    if let Some(Tile::Open(entities)) = self.blocks.get(&pos) {
      // If it was an open tile with entities, preserve the entities
      self.blocks.insert(pos, Tile::Open(entities.clone()));
    } else {
      self.blocks.insert(pos, Tile::Open(Vec::new()));
    }
  }
  pub fn add_entity(&mut self, pos: IVec3, entity: Entity) {
    match self.blocks.get_mut(&pos) {
      Some(Tile::Open(entities)) => {
        if !entities.contains(&entity) {
          entities.push(entity);
        }
      }
      Some(_) => {
        warn!("Attempted to add entity to non-open tile at {:?}", pos);
      }
      None => {
        self.blocks.insert(pos, Tile::Open(vec![entity]));
      }
    }
  }
  pub fn remove_entity(&mut self, pos: IVec3, entity: Entity) {
    if let Some(Tile::Open(entities)) = self.blocks.get_mut(&pos) {
      entities.retain(|&e| e != entity);
    }
  }
  pub fn get_tile(&self, pos: IVec3) -> Option<&Tile> { self.blocks.get(&pos) }
  pub fn is_walkable(&self, pos: IVec3) -> bool {
    match self.blocks.get(&pos) {
      Some(Tile::Open(_)) => true,
      _ => false
    }
  }

  pub fn get_entities_at(&self, pos: IVec3) -> Vec<Entity> {
    match self.blocks.get(&pos) {
      Some(Tile::Open(entities)) => entities.clone(),
      _ => Vec::new()
    }
  }
  // Core position queries
  pub fn get_position(&self, entity: Entity) -> Option<IVec3> {
    self.position_cache.get(&entity).copied()
  }

  pub fn get_entities_at(&self, pos: IVec3) -> Vec<Entity> {
    match self.blocks.get(&pos) {
      Some(Tile::Open(entities)) => entities.clone(),
      _ => Vec::new()
    }
  }

  pub fn is_walkable(&self, pos: IVec3) -> bool {
    matches!(self.blocks.get(&pos), Some(Tile::Open(_)))
  }

  pub fn get_material_at(&self, pos: IVec3) -> Option<Material> {
    match self.blocks.get(&pos) {
      Some(Tile::Solid(mat)) => Some(*mat),
      Some(Tile::Furniture(_, mat)) => Some(*mat),
      _ => None
    }
  }

  // Adjacency helpers
  pub fn get_adjacent_positions(&self, pos: IVec3) -> Vec<IVec3> {
    const DIRECTIONS: [IVec3; 6] = [IVec3::X,
                                    -IVec3::X,
                                    IVec3::Y,
                                    -IVec3::Y,
                                    IVec3::Z,
                                    -IVec3::Z];
    DIRECTIONS.iter().map(|dir| pos + *dir).collect()
  }

  pub fn get_adjacent_entities(&self, pos: IVec3) -> Vec<Entity> {
    self.get_adjacent_positions(pos)
        .iter()
        .flat_map(|adj_pos| self.get_entities_at(*adj_pos))
        .collect()
  }

  // Advanced spatial queries
  pub fn find_entities_in_radius(&self, center: IVec3, radius: i32) -> Vec<Entity> {
    let mut result = Vec::new();
    for x in -radius..=radius {
      for y in -radius..=radius {
        for z in -radius..=radius {
          let pos = center + IVec3::new(x, y, z);
          if Location(center).manhattan_distance(&Location(pos)) <= radius {
            result.extend(self.get_entities_at(pos));
          }
        }
      }
    }
    result
  }

  pub fn find_reachable_positions(&self, start: IVec3, max_distance: i32) -> HashSet<IVec3> {
    let mut visited = HashSet::new();
    let mut queue = vec![start];
    visited.insert(start);

    while let Some(pos) = queue.pop() {
      if Location(start).manhattan_distance(&Location(pos)) >= max_distance {
        continue;
      }

      for adj_pos in self.get_adjacent_positions(pos) {
        if self.is_walkable(adj_pos) && !visited.contains(&adj_pos) {
          visited.insert(adj_pos);
          queue.push(adj_pos);
        }
      }
    }
    visited
  }
}

// High-level spatial relationship queries
pub struct SpatialQueries;

impl SpatialQueries {
  // Find entities with components that are adjacent to a position
  pub fn find_adjacent<T: Component>(world: &BlockWorld,
                                     pos: IVec3,
                                     query: &Query<(Entity, &Location, &T)>)
                                     -> Vec<(Entity, &T)> {
    let adjacent_positions: HashSet<_> =
      world.get_adjacent_positions(pos).into_iter().collect();
    query.iter()
         .filter(|(_, loc, _)| adjacent_positions.contains(&loc.0))
         .map(|(entity, _, component)| (entity, component))
         .collect()
  }

  // Find the nearest entity with a specific component
  pub fn find_nearest<T: Component>(world: &BlockWorld,
                                    from: IVec3,
                                    query: &Query<(Entity, &Location, &T)>)
                                    -> Option<(Entity, &T, i32)> {
    query.iter()
         .map(|(entity, loc, component)| {
           (entity, component, Location(from).manhattan_distance(&loc))
         })
         .min_by_key(|&(_, _, dist)| dist)
  }

  // Check if two entities are adjacent
  pub fn are_adjacent(world: &BlockWorld, entity1: Entity, entity2: Entity) -> bool {
    let pos1 = world.get_position(entity1)?;
    let pos2 = world.get_position(entity2)?;
    Location(pos1).manhattan_distance(&Location(pos2)) == 1
  }
}

// Systems to maintain spatial relationships
pub fn sync_locations(mut world: ResMut<BlockWorld>,
                      mut commands: Commands,
                      mut moved_query: Query<(Entity, &Location), Changed<Location>>) {
  for (entity, location) in moved_query.iter() {
    if let Some(old_pos) = world.position_cache.get(&entity) {
      world.remove_entity(*old_pos, entity);
    }

    if world.is_walkable(location.0) {
      world.add_entity(location.0, entity);
      world.position_cache.insert(entity, location.0);
    }
  }
}

// Example spatial relationship components
#[derive(Component)]
pub struct RequiresAdjacent<T: Component> {
  _phantom: std::marker::PhantomData<T>
}

#[derive(Component)]
pub struct RequiresNearby<T: Component> {
  radius: i32,
  _phantom: std::marker::PhantomData<T>
}

// Example system using spatial relationships
pub fn check_spatial_requirements(world: Res<BlockWorld>,
                                  query_requires_adjacent: Query<(Entity,
                                         &Location,
                                         &RequiresAdjacent<T>)>,
                                  query_component: Query<(Entity, &Location, &T)>) {
  for (entity, location, _requirement) in query_requires_adjacent.iter() {
    let adjacent = SpatialQueries::find_adjacent(&world, location.0, &query_component);
    if adjacent.is_empty() {
      // Handle case where required adjacent entity is missing
      // e.g., disable crafting, stop growth, etc.
    }
  }
}

// Example usage:
fn spawn_with_spatial_requirement(commands: &mut Commands,
                                  pos: IVec3,
                                  world: &mut BlockWorld) {
  // Spawn an entity that requires an adjacent Craftable
  let entity = commands.spawn((Location(pos),
                       RequiresAdjacent::<Craftable> { _phantom:
                                                         std::marker::PhantomData }))
               .id();

  world.add_entity(pos, entity);
}

// Plugin to organize all spatial systems
#[derive(Default)]
pub struct SpatialPlugin;

impl Plugin for SpatialPlugin {
  fn build(&self, app: &mut App) {
    app.init_resource::<BlockWorld>()
       .add_systems(Update, (sync_locations, check_spatial_requirements));
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
// fn filter_least_map<O: Ord + Clone, T, R>(f: impl Fn(T) -> Option<(R, O)>,
//                                           coll: impl IntoIterator<Item = T>)
//                                           -> Option<R> {
//   coll.into_iter()
//       .filter_map(f)
//       .min_by_key(|(_, o)| o.clone())
//       .map(|(r, _)| r)
// }

// fn filter_least<O: Ord + Clone, T>(f: impl Fn(&T) -> Option<O>,
//                                    coll: impl IntoIterator<Item = T>)
//                                    -> Option<T> {
//   filter_least_map(|t| f(&t).map(|v| (t, v)), coll)
// }
// fn filter_most_map<O: Ord + Clone, T, R>(f: impl Fn(T) -> Option<(R, O)>,
//                                          coll: impl IntoIterator<Item = T>)
//                                          -> Option<R> {
//   coll.into_iter()
//       .filter_map(f)
//       .max_by_key(|(_, o)| o.clone())
//       .map(|(r, _)| r)
// }
// fn filter_most<O: Ord + Clone, T>(f: impl Fn(&T) -> Option<O>,
//                                   coll: impl IntoIterator<Item = T>)
//                                   -> Option<T> {
//   filter_most_map(|t| f(&t).map(|v| (t, v)), coll)
// }

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

fn camera_movement(mut camq: Query<&mut PanOrbitCamera>,
                   targetq: Query<&Pos, With<CameraTarget>>) {
  if let Ok(camera_target_pos) = targetq.get_single()
     && let Ok(mut cam) = camq.get_single_mut()
  {
    cam.target_focus = camera_target_pos.into();
  }
}
#[derive(Component, Debug, Clone, Copy, new)]
pub struct Navigation {
  max_speed: f32,
  #[new(default)]
  navigation_kind: NavigationKind
}

#[derive(Default, Debug, Clone, Copy)]
enum NavigationKind {
  #[default]
  None,
  // Dir2(Dir2),
  Vec2(Vec2),
  Pos(Vec2),
  Chase(Entity) // ChaseAtRange(Entity, f32)
}

#[derive(Default, Resource)]
pub struct TimeTicks(pub u32);
pub fn increment_time(mut time: ResMut<TimeTicks>) { time.0 += 1; }
pub fn timed_animation_system(time_ticks: Res<TimeTicks>,
                              mut q: Query<(&TimedAnimation, &mut TextureAtlas)>) {
  for (&TimedAnimation { num_frames,
                         time_per_frame_in_ticks },
       mut atlas) in &mut q
  {
    let time = time_ticks.0 as usize;
    let index = |time| (time / time_per_frame_in_ticks) % num_frames;
    let old_index = index(time.saturating_sub(1));
    let new_index = index(time);
    if new_index != old_index {
      atlas.index = new_index;
    }
  }
}

fn close_on_esc(mut exit: EventWriter<AppExit>, keyboard_input: Res<ButtonInput<KeyCode>>) {
  if keyboard_input.just_pressed(KeyCode::Escape) {
    exit.send(AppExit::Success);
  }
}

fn namefmt(oname: Option<&Name>) -> String {
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
    let player_light_on = player.light_on;
    let infobox_data =
      map(ToString::to_string,
          [format!("{:.1}", player_pos).as_str(),
           format!("you've found {} notes", player.notes_found.len()).as_str(),
           format!("light on: {player_light_on}",).as_str(),
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

pub fn string(t: impl ToString) -> String { t.to_string() }
#[derive(Component, Clone, Default)]
struct CanBeFollowedByNPC;

pub const BLOOM_SETTINGS: BloomSettings =
  BloomSettings { intensity: 0.5,
                  low_frequency_boost: 0.0,
                  prefilter_settings: BloomPrefilterSettings { threshold: 2.2,
                                                               threshold_softness: 0.0 },

                  composite_mode: BloomCompositeMode::Additive,
                  ..BloomSettings::NATURAL };

const TONEMAPPING: bevy::core_pipeline::tonemapping::Tonemapping =
  bevy::core_pipeline::tonemapping::Tonemapping::BlenderFilmic;

const FOG_SETTINGS: FogSettings =
  FogSettings { color: Color::srgb(0.25, 0.25, 0.25),
                falloff: FogFalloff::ExponentialSquared { density: 0.5 },
                directional_light_color: Color::NONE,
                directional_light_exponent: 8.0 };

pub const AMBIENT_LIGHT: AmbientLight = AmbientLight { color: mycolor::AMBIENT_LIGHT,
                                                       brightness: 14.0 };
const PLAYER_LIGHT_FLASHLIGHT: SpotLight =
  SpotLight { color: Color::WHITE,
              intensity: 600_000.0,
              range: 17.0,
              radius: 0.0,
              shadows_enabled: true,
              shadow_depth_bias: SpotLight::DEFAULT_SHADOW_DEPTH_BIAS,
              shadow_normal_bias: SpotLight::DEFAULT_SHADOW_NORMAL_BIAS,
              inner_angle: 0.3,
              outer_angle: 0.7 };
const TORCH_LIGHT: PointLight =
  PointLight { color: Color::hsv(33.0, 1.0, 0.5),
               intensity: 60_000.0,
               radius: 0.0,
               range: 8.0,
               shadows_enabled: true,
               shadow_depth_bias: PointLight::DEFAULT_SHADOW_DEPTH_BIAS / 10.0,
               shadow_normal_bias: PointLight::DEFAULT_SHADOW_NORMAL_BIAS / 10.0 };
const PLAYER_LIGHT_AMBIENT: PointLight =
  PointLight { color: Color::WHITE,
               intensity: 9_000.0,
               radius: 0.0,
               range: 16.0,
               shadows_enabled: false,
               shadow_depth_bias: PointLight::DEFAULT_SHADOW_DEPTH_BIAS / 10.0,
               shadow_normal_bias: PointLight::DEFAULT_SHADOW_NORMAL_BIAS / 10.0 };

#[derive(Component)]
struct Note(&'static str);
#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
enum GameState {
  Running,
  #[default]
  NotStarted,
  GameOver
}
// #[derive(Resource, Default)]
// struct GameOver(bool);

// const TILE_SIZE: f32 = 1.0;
// const CHARACTER_HEIGHT: f32 = TILE_SIZE;
// const CHARACTER_RADIUS: f32 = CHARACTER_HEIGHT * 0.3;
// #[derive(Bundle, Clone)]
// pub struct CharacterBundle((Visuals,
//                              FaceCamera,
//                              LockedAxes,
//                              ColliderMassProperties,
//                              Collider,
//                              RigidBody,
//                              Friction,
//                              // LinearDamping,
//                              // AngularDamping,
//                              LinearVelocity,
//                              AngularVelocity,
//                              ExternalForce,
//                              ExternalImpulse,
//                              SpatialBundle));
// impl CharacterBundle {
//   fn new(translation: Vec3, can_move: bool, visuals: Visuals) -> Self {
//     let cube_mesh = Cuboid::default().mesh().build();
//     let cube_collider = Cuboid::default().collider();
//     let cylinder_collider = Cylinder::new(CHARACTER_RADIUS, CHARACTER_HEIGHT).collider();
//     let sphere_collider = Sphere::new(1.0).collider();
//     // capsule_from_height_and_radius
//     let capsule_collider =
//       Capsule3d::new(CHARACTER_RADIUS, CHARACTER_HEIGHT - CHARACTER_RADIUS * 2.0).collider();
//     // Friction::ZERO
//     // let mesh = Capsule3d::new(CHARACTER_RADIUS, CHARACTER_RADIUS + CHARACTER_HEIGHT).collider()
//     // let mesh = Capsule3d::new(CHARACTER_RADIUS, CHARACTER_RADIUS + CHARACTER_HEIGHT).mesh()
//     //                                                                                 .build();
//     // let collider = Collider::convex_hull_from_mesh(&mesh).unwrap();
//     // let collider = Collider::convex_hull_from_mesh(&cube_mesh).unwrap();
//     let collider = capsule_collider;
//     // let collider = capsule_from_height_and_radius(CHARACTER_HEIGHT, CHARACTER_RADIUS);
//     // FogSettings
//     Self((visuals,
//           FaceCamera,
//           LockedAxes::ROTATION_LOCKED,
//           // LockedAxes::new().lock_rotation_x().lock_rotation_z(),
//           ColliderMassProperties::new(&collider, 1.0),
//           collider,
//           if can_move {
//             RigidBody::Dynamic
//           } else {
//             RigidBody::Static
//           },
//           Friction::ZERO,
//           // LinearDamping(1.6),
//           // AngularDamping(1.2),
//           LinearVelocity::default(),
//           AngularVelocity::default(),
//           ExternalForce::default().with_persistence(false),
//           ExternalImpulse::default(),
//           SpatialBundle { transform: Transform { translation,
//                                                  ..default() },
//                           ..default() }))
//   }
//   fn sprite(translation: Vec3, scale: f32, can_move: bool, sprite: MySprite) -> Self {
//     Self::new(translation, can_move, Visuals::sprite(sprite))
//   }
// }

fn rangerand(lo: f32, hi: f32) -> f32 { lo.lerp(hi, rand::random::<f32>()) }
fn random_normalized_vector() -> Vec3 { random::<Quat>() * Vec3::X }

const NOTE_FIND_RANGE: f32 = 1.8;
fn note(translation: Vec3, note_data: &'static str) -> impl Bundle {
  (Visuals::sprite(MySprite::NOTE),
   Note(note_data),
   Proximal { distance: NOTE_FIND_RANGE },
   SpatialBundle { transform: Transform { translation,
                                          rotation:
                                            (Quat::from_rotation_y(avian3d::math::PI * 0.2)
                                             * Quat::from_rotation_x(avian3d::math::PI
                                                                     * 0.5)).into(),
                                          scale: Vec3::splat(0.5) },
                   ..default() })
}
fn torch(pos: Vec3) -> impl Bundle {
  (Visuals::unlit_sprite(MySprite::TORCH),
   FaceCamera,
   PointLightBundle { transform: Transform::from_translation(pos),
                      point_light: TORCH_LIGHT,
                      ..default() })
}

// fn init(mut world: &mut World) { world.clear_entities() }
type NumberFunction = fn(i32) -> i32;

const INC: NumberFunction = |x| x + 1;

#[derive(Component, Debug)]
struct Player {}

#[derive(Component, Debug)]
pub struct CameraTarget;
pub fn setup(playerq: Query<&Transform, With<Player>>,
             serv: Res<AssetServer>,
             mut meshes: ResMut<Assets<Mesh>>,
             mut materials: ResMut<Assets<StandardMaterial>>,
             mut c: Commands) {
  c.spawn((Pos::default(), CameraTarget));
  let fov = std::f32::consts::PI / 4.0;

  let pitch_upper_limit_radians = 1.0;
  let pitch_lower_limit_radians = 0.2;
  let camera =
    (IsDefaultUiCamera,
     BLOOM_SETTINGS,
     // Skybox { image: skybox_handle.clone(),
     //          brightness: 600.0 },
     Camera2d,
     // FOG_SETTINGS,
     VoxelWorldCamera::<MyMainWorld>::default(),
     Camera3dBundle { camera: Camera { hdr: true,

                                       ..default() },
                      transform:
                        Transform::from_xyz(10.0, 10.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y),

                      tonemapping: TONEMAPPING,
                      projection:
                        Projection::Perspective(PerspectiveProjection { fov, ..default() }),
                      exposure: bevy::render::camera::Exposure { ev100: 10.0 },
                      // tonemapping:
                      //   bevy::core_pipeline::tonemapping::Tonemapping::Reinhard,
                      ..default() },
     PanOrbitCamera { // radius: Some(5.0),

                      // focus: todo!(),
                      // yaw: todo!(),
                      // pitch: todo!(),
                      // target_focus: todo!(),
                      // target_yaw: todo!(),
                      // target_pitch: todo!(),
                      // target_radius: todo!(),
                      // yaw_upper_limit: todo!(),
                      // yaw_lower_limit: todo!(),
                      pitch_upper_limit: Some(pitch_upper_limit_radians),
                      pitch_lower_limit: Some(pitch_lower_limit_radians),
                      zoom_upper_limit: Some(8.0),
                      zoom_lower_limit: Some(2.0),
                      // orbit_sensitivity: todo!(),
                      orbit_smoothness: 0.0,
                      pan_sensitivity: 0.0,
                      pan_smoothness: 0.5,
                      zoom_sensitivity: 2.5,
                      // zoom_smoothness: todo!(),
                      // button_orbit: todo!(),
                      // button_pan: todo!(),
                      // modifier_orbit: todo!(),
                      // modifier_pan: todo!(),
                      // touch_enabled: todo!(),
                      // touch_controls: todo!(),
                      // reversed_zoom: todo!(),
                      // is_upside_down: todo!(),
                      // allow_upside_down: todo!(),
                      // enabled: todo!(),
                      // initialized: todo!(),
                      // force_update: todo!(),
                      ..default() });
  c.spawn(camera);
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
  let cube_collider = Cuboid::default().collider();
  let ground_mesh =
    bevy::math::primitives::Plane3d::new(Vec3::Y, Vec2::new(40.0, 25.5)).mesh()
                                                                        .build();
  // let ground_mesh =
  //   bevy::math::primitives::Plane3d::new(Vec3::Y, Vec2::new(100.0, 100.0)).mesh()
  //                                                                         .build();
  let ground_collider = avian3d::prelude::Collider::trimesh_from_mesh(&ground_mesh).unwrap();
  let ground_texture = serv.load("embedded://ground.png");
  let ground_material = serv.add(StandardMaterial { perceptual_roughness: 0.8,
                                                    metallic: 0.0,
                                                    reflectance: 0.2,
                                                    base_color_texture:
                                                      Some(ground_texture),
                                                    ..default() });
  let ground = (ground_collider,
                RigidBody::Static,
                PbrBundle { mesh: serv.add(ground_mesh),
                            material: ground_material.clone(),
                            transform: Transform::from_xyz(39.5, 0.0, 42.5),
                            ..default() });
  c.spawn(ground);
  let small_ground_mesh =
    bevy::math::primitives::Plane3d::new(Vec3::Y, Vec2::new(3.5, 3.5)).mesh()
                                                                      .build();
  let small_ground_collider =
    avian3d::prelude::Collider::trimesh_from_mesh(&small_ground_mesh).unwrap();
  let small_ground = (small_ground_collider,
                      RigidBody::Static,
                      PbrBundle { mesh: serv.add(small_ground_mesh),
                                  material: ground_material.clone(),
                                  transform: Transform::from_xyz(37.0, 0.0, 3.0),
                                  ..default() });
  c.spawn(small_ground);
  let cube_mesh = Cuboid::default().mesh().build();
  let cube_collider = Collider::convex_hull_from_mesh(&cube_mesh).unwrap();
  let cube_mesh_handle = serv.add(cube_mesh);
  let position = Vec3::new(0.0, 4.0, 0.0);
  println("setup");
}
#[derive(Resource, Clone, Default)]
struct MyMainWorld;

impl VoxelWorldConfig for MyMainWorld {
  fn texture_index_mapper(&self) -> Arc<dyn Fn(u8) -> [u32; 3] + Send + Sync> {
    // WorldVoxel
    Arc::new(|vox_mat: u8| {
      let block_type = BlockType::from_index(vox_mat);
      let textures = block_type.textures();
      let texture_indexes = textures.map(BlockTexture::to_u32);
      texture_indexes
    })
  }
  fn voxel_texture(&self) -> Option<(String, u32)> {
    Some(("block_textures.png".into(), BlockTexture::NUM as u32))
  }
}

impl From<BlockType> for WorldVoxel {
  fn from(block_type: BlockType) -> Self { WorldVoxel::Solid(block_type.to_index()) }
}
fn create_voxel_scene(mut voxel_world: VoxelWorld<MyMainWorld>) {
  // Then we can use the `u8` consts to specify the type of voxel

  // 20 by 20 floor
  for x in -10..10 {
    for z in -10..10 {
      voxel_world.set_voxel(IVec3::new(x, -1, z), BlockType::Snow.into());
      // Grassy floor
    }
  }

  // Some bricks
  voxel_world.set_voxel(IVec3::new(0, 0, 0), BlockType::Snow.into());
  voxel_world.set_voxel(IVec3::new(0, 0, 0), BlockType::Snow.into());
  voxel_world.set_voxel(IVec3::new(1, 0, 0), BlockType::Snow.into());
  voxel_world.set_voxel(IVec3::new(0, 0, 1), BlockType::Snow.into());
  voxel_world.set_voxel(IVec3::new(0, 0, -1), BlockType::Stone.into());
  voxel_world.set_voxel(IVec3::new(-1, 0, 0), BlockType::Stone.into());
  voxel_world.set_voxel(IVec3::new(-2, 0, 0), BlockType::Sand.into());
  voxel_world.set_voxel(IVec3::new(-1, 1, 0), BlockType::Bricks.into());
  voxel_world.set_voxel(IVec3::new(-2, 1, 0), BlockType::Snow.into());
  voxel_world.set_voxel(IVec3::new(0, 1, 0), BlockType::Snow.into());
}

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
  let gravity = avian3d::dynamics::integrator::Gravity::default();
  let solver_config = SolverConfig { contact_damping_ratio: 0.5,
                                     // contact_frequency_factor: 1.5,
                                     // max_overlap_solve_speed: 4.0,
                                     // warm_start_coefficient: 1.0,
                                     // restitution_threshold: 1.0,
                                     // restitution_iterations: 1,
                                     ..default() };
  let address_mode = ImageAddressMode::ClampToBorder;
  let default_sampler = ImageSamplerDescriptor { // address_mode_u: address_mode,
                                                 //                        address_mode_v: address_mode,
                                                 //                        address_mode_w: address_mode,
                                                 mag_filter: ImageFilterMode::Nearest,
                                                 min_filter: ImageFilterMode::Linear,
                                                 mipmap_filter: ImageFilterMode::Linear,
                                                 // compare:
                                                 //   Some(ImageCompareFunction::Less),
                                                 // lod_min_clamp: 10.0,
                                                 // lod_max_clamp: 100.0,
                                                 // border_color:
                                                 //   Some(ImageSamplerBorderColor::TransparentBlack),
                                                 // anisotropy_clamp: 1000,
                                                 ..default() };
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
        .set(ImagePlugin{default_sampler})
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
      // bevy_debug_camera::DebugCameraPlugin::default(),
      bevy_panorbit_camera::PanOrbitCameraPlugin,
      bevy_mod_billboard::prelude::BillboardPlugin,
      // bevy_mod_picking::DefaultPickingPlugins,
      avian3d::PhysicsPlugins::default(),
      QuillPlugin,
      QuillOverlaysPlugin,
    ))// .add_plugins(add_global_highlight)
  // .add_event::<GuiInputEvent>()

    .init_state::<GameState>()
    .init_resource::<UIData>()
    .init_resource::<GameOver>()
    .init_resource::<TimeTicks>()
    .insert_resource(gravity)
    .insert_resource(solver_config)
    .insert_resource(ClearColor(mycolor::CLEAR))
    .insert_resource(AMBIENT_LIGHT)
    .insert_resource(Msaa::Sample4)
    .add_systems(Startup, (setup,create_voxel_scene
    ).chain())
    .add_systems(Update,(
      close_on_esc,
      // toggle_flashlight,
      // navigation,
      // player_movement,
      // monster_movement,
      camera_movement,
      increment_time,
      origin_time,
      timed_animation_system,
    ).chain())
    .add_systems(Update,(
      face_camera,
      // proximity_system,
      visuals,
      ui,
    ).chain())
    .run();
}

// trunk build --release --public-url "bevyspookygame" --filehash false

// trunk serve

// cargo check --target wasm32-unknown-unknown
// cargo run --target x86_64-unknown-linux-gnu
// cargo check --target x86_64-unknown-linux-gnu
// cargo build --target x86_64-pc-windows-gnu --release
