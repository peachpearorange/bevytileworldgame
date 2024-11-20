
use {crate::{cuboid_coords, BlockType, Location, MySprite, RandomMovement, Visuals},
     bevy::prelude::{Name, *},
     noise::{NoiseFn, Perlin},
     rand::random};

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

fn generate_tile(noise: &Perlin, pos: IVec3) -> WorldTile {
  let loc = Location::from(pos);
  let wanderer =
    bundle_spawn((Name::new("Wanderer"), RandomMovement, Visuals::sprite(MySprite::PLAYER)));
  let tree = bundle_spawn((Name::new("tree"), Visuals::sprite(MySprite::TREE)));

  let prob = |p| rand::random::<f32>() < p;
  let IVec3 { x, y, z } = pos;
  let surface_height = 5;
  let cave = noise.get([x as f64 / 20.0, y as f64 / 20.0, z as f64 / 20.0]) > 0.6;
  let height = surface_height;

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

pub fn spawn_world(mut c: &mut Commands) {
  let noise = Perlin::new(5);
  let bounds = IVec3::new(40, 20, 40);
  let coords = cuboid_coords(bounds);
  for (pos, tile) in coords.map(move |pos| (pos, generate_tile(&noise, pos))) {
    let loc = Location::from(pos);
    if let Some(block) = tile.block {
      c.spawn((loc, block));
    }
    for spawn in tile.entities {
      spawn(&mut c, loc);
    }
    for spawn in tile.entities_above {
      spawn(&mut c, loc.above());
    }
  }
}
