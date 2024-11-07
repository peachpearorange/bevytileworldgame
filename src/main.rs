#![allow(clippy::unnecessary_cast)]
#![allow(unused_imports)]
#![allow(dead_code)]
// #![feature(const_trait_impl)]
#![feature(type_alias_impl_trait)]
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
pub const AMBIENT_LIGHT_COLOR: Color = Color::hsv(301.0, 1.0, 1.0);
pub const CLEAR_COLOR: Color = Color::hsv(301.0, 1.0, 0.07);

pub const GLOWY_COLOR: Color = Color::srgb(13.99, 11.32, 50.0);
pub const GLOWY_COLOR_2: Color = Color::srgb(30.0, 20.7, 10.5);
pub const GLOWY_COLOR_3: Color = Color::srgb(0.0, 30.0, 0.0);
pub const EXPLOSION_COLOR: Color = Color::srgb(8.0, 3.0, 3.0);
pub const LASER_COLOR: Color = Color::hsv(60.0, 1.0, 4.0);
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
  const WHITE: Self = Self::new("white_corners.png");
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
                                                      ..GLOWY_COLOR.into() });
  const GLOWY_2: Self = Self::new(|| StandardMaterial { unlit: true,
                                                        alpha_mode: AlphaMode::Mask(0.0),
                                                        ..GLOWY_COLOR_2.into() });
  const GLOWY_3: Self = Self::new(|| StandardMaterial { unlit: true,
                                                        alpha_mode: AlphaMode::Mask(0.0),
                                                        ..GLOWY_COLOR_3.into() });
  const EXPLOSION: Self = Self::new(|| StandardMaterial { unlit: true,
                                                          alpha_mode:
                                                            AlphaMode::Mask(0.0001),
                                                          ..EXPLOSION_COLOR.into() });
  const LASER: Self = Self::new(|| StandardMaterial { unlit: true,
                                                      alpha_mode:
                                                        AlphaMode::Mask(0.0001),
                                                      ..LASER_COLOR.into() });
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

#[derive(Component, Clone)]
enum Tile {
  Air,
  Wall(Material),
  Floor(Material),
  Furniture(Furniture, Material)
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
#[derive(Component)]
struct Pos(pub IVec3);

fn block_entity(pos: Pos, block_type: BlockType) -> impl Bundle { (pos, block_type) }
const NORMAL_NPC_SCALE: f32 = 1.9;
const NORMAL_NPC_THRUST: f32 = 400.0;
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

// fn basic_tile(pos: IVec3, tile: Tile) -> impl Bundle {
//   // let tile = Tile { walkable,
//   //                   color: color.to_string() };
//   let base_bundle = (tile, name(name));
//   base_bundle
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

mod spawnables {
  use {crate::{basic_animal, Pos},
       bevy::prelude::{Bundle, Commands}};

  type Spawnable = fn() -> Box<dyn FnOnce(&mut Commands, Pos)>;
  // impl<B: Bundle> From<B> for Box<dyn FnOnce(&mut Commands, Pos)> {
  //   fn from(b: B) -> Self {
  //     Box::new(|commands, pos| {
  //       commands.spawn((pos, b));
  //     })
  //   }
  // }

  // const SNOWMAN: Spawnable = || basic_animal("snowman", '⛄').into();
}
mod template {}
fn basic_animal(nameval: &'static str, chardisplay: char) -> impl Bundle {
  (name(nameval),
   RandomMovement,
   // SpaceObjectBundle::new(pos, NORMAL_NPC_SCALE, true, Visuals::character(chardisplay))
  )
}
type ImplBundle = impl Bundle;
type BundleFn = fn() -> ImplBundle;
const SNOWMAN: BundleFn = || basic_animal("snowman", '⛄');
// const SNOWMAN: ImplBundle = basic_animal("snowman", '⛄');
type AAA = fn() -> ImplBundle;
struct Spawnable(fn(&mut Commands, Pos));
// impl Spawnable {
//   const SNOWMAN: Self = Self(|| basic_animal("snowman", '⛄').into());
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
enum Material {
  Stone(Stone),
  Metal(Metal),
  Wood(Wood),
  Gem(Gem),
  CrystalOre,
  MetalOre,
  Dirt
}

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

// Item System
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
#[derive(Clone, Copy)]
enum Labor {
  Mine,
  Woodcut,
  Mason,
  Smith
}

// type Template = impl Bundle;

type Auto = impl std::any::Any;
// const A: Auto = 5;
type UnaryFn<A, B> = fn(A) -> B;
fn block(mat: Material) -> impl Bundle {
  (Tile::Wall(mat),
   Minable { item: Item::Block(mat),
             labor: match mat {
               Material::Stone(_) => Labor::Mine,
               Material::Wood(_) => Labor::Woodcut,
               _ => Labor::Mine
             } })
}
const GRANITE: Auto = || block(Material::Stone(Stone::Granite));
const MARBLE: Auto = || block(Material::Stone(Stone::Marble));
const GOLD_ORE: Auto = || block(Material::Metal(Metal::Gold));
type ImplAdd = impl std::ops::Add;
const K: ImplAdd = 5;
// const J: Auto = K + K;

const fn furniture(mat: Material, kind: Furniture) -> impl Bundle {
  let k = K;
  (Tile::Furniture(kind, mat), Quality(0))
}

// Block Templates

// Furniture Templates
const GRANITE_TABLE: ImplBundle =
  furniture(Material::Stone(Stone::Granite), Furniture::Table);

const OAK_BED: ImplBundle = furniture(Material::Wood(Wood::Oak), Furniture::Bed);

// Workshop Templates
const FORGE: fn(Material) -> ImplBundle = |mat| {
  (name("forge"), Tile::Furniture(Furniture::Forge, mat), Container(vec![]), Quality(0))
};

// Item Creation Helpers
fn make_tool(mat: Material, kind: Tool) -> Item { Item::Tool { mat, kind } }

fn make_weapon(mat: Material, kind: Weapon) -> Item { Item::Weapon { mat, kind } }
// existential type Adder: Fn(usize) -> usize;
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

fn camera_follow_player(mut camq: Query<&mut PanOrbitCamera>,
                        playerq: Query<&Transform, With<Player>>) {
  if let Ok(player_transform) = playerq.get_single()
     && let Ok(mut cam) = camq.get_single_mut()
  {
    cam.target_focus = player_transform.translation;
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

pub const AMBIENT_LIGHT: AmbientLight = AmbientLight { color: AMBIENT_LIGHT_COLOR,
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

#[derive(Component, new)]
struct Proximal {
  distance: f32
}
#[derive(Component)]
struct Note(&'static str);
#[derive(Component, Clone, Debug, Default)]
pub struct Player {
  notes_found: HashSet<Entity>,
  light_on: bool
}
#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
enum GameState {
  Running,
  #[default]
  NotStarted,
  GameOver
}
#[derive(Resource, Default)]
struct GameOver(bool);

const TILE_SIZE: f32 = 1.0;
const CHARACTER_HEIGHT: f32 = TILE_SIZE;
const CHARACTER_RADIUS: f32 = CHARACTER_HEIGHT * 0.3;
#[derive(Bundle, Clone)]
pub struct CharacterBundle((Visuals,
                             FaceCamera,
                             LockedAxes,
                             ColliderMassProperties,
                             Collider,
                             RigidBody,
                             Friction,
                             // LinearDamping,
                             // AngularDamping,
                             LinearVelocity,
                             AngularVelocity,
                             ExternalForce,
                             ExternalImpulse,
                             SpatialBundle));
impl CharacterBundle {
  fn new(translation: Vec3, can_move: bool, visuals: Visuals) -> Self {
    let cube_mesh = Cuboid::default().mesh().build();
    let cube_collider = Cuboid::default().collider();
    let cylinder_collider = Cylinder::new(CHARACTER_RADIUS, CHARACTER_HEIGHT).collider();
    let sphere_collider = Sphere::new(1.0).collider();
    // capsule_from_height_and_radius
    let capsule_collider =
      Capsule3d::new(CHARACTER_RADIUS, CHARACTER_HEIGHT - CHARACTER_RADIUS * 2.0).collider();
    // Friction::ZERO
    // let mesh = Capsule3d::new(CHARACTER_RADIUS, CHARACTER_RADIUS + CHARACTER_HEIGHT).collider()
    // let mesh = Capsule3d::new(CHARACTER_RADIUS, CHARACTER_RADIUS + CHARACTER_HEIGHT).mesh()
    //                                                                                 .build();
    // let collider = Collider::convex_hull_from_mesh(&mesh).unwrap();
    // let collider = Collider::convex_hull_from_mesh(&cube_mesh).unwrap();
    let collider = capsule_collider;
    // let collider = capsule_from_height_and_radius(CHARACTER_HEIGHT, CHARACTER_RADIUS);
    // FogSettings
    Self((visuals,
          FaceCamera,
          LockedAxes::ROTATION_LOCKED,
          // LockedAxes::new().lock_rotation_x().lock_rotation_z(),
          ColliderMassProperties::new(&collider, 1.0),
          collider,
          if can_move {
            RigidBody::Dynamic
          } else {
            RigidBody::Static
          },
          Friction::ZERO,
          // LinearDamping(1.6),
          // AngularDamping(1.2),
          LinearVelocity::default(),
          AngularVelocity::default(),
          ExternalForce::default().with_persistence(false),
          ExternalImpulse::default(),
          SpatialBundle { transform: Transform { translation,
                                                 ..default() },
                          ..default() }))
  }
  fn sprite(translation: Vec3, scale: f32, can_move: bool, sprite: MySprite) -> Self {
    Self::new(translation, can_move, Visuals::sprite(sprite))
  }
}

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

pub fn setup(playerq: Query<&Transform, With<Player>>,
             serv: Res<AssetServer>,
             mut meshes: ResMut<Assets<Mesh>>,
             mut materials: ResMut<Assets<StandardMaterial>>,
             mut c: Commands) {
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
  pub fn to_u8(self) -> u8 { self as u8 }
  pub fn to_u32(self) -> u32 { self as u32 }
  pub fn from_index(index: u8) -> Self { unsafe { std::mem::transmute(index) } }
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
  pub fn to_index(self) -> u8 { self as u8 }
  pub fn to_u32(self) -> u32 { self as u32 }
  pub fn from_index(index: u8) -> Self { unsafe { std::mem::transmute(index) } }
}
impl From<BlockType> for u8 {
  fn from(block_type: BlockType) -> u8 { block_type as u8 }
}
#[derive(Default,Resource)]
struct Blocks(HashMap<IVec3,BlockType>);

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
    .insert_resource(ClearColor(CLEAR_COLOR))
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
      camera_follow_player,
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
