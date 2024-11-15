use {crate::{BlockType, Location, MySprite, RandomMovement, Visuals},
     bevy::prelude::{Name, *},
     noise::{NoiseFn, Perlin},
     rand::random};

// Dimensions of the world grid
const WIDTH: usize = 256;
const HEIGHT: usize = 64;
const LENGTH: usize = 256;

struct WorldGenTile(Option<BlockType>, Box<dyn FnOnce(&mut Commands)>);

type SpawnFn = Box<dyn FnOnce(&mut Commands)>;
// type SpawnFn = fn(&mut Commands);

enum WorldTile {
  Block(BlockType),
  BlockWithEntitiesOnTop(BlockType, Vec<SpawnFn>),
  Entities(Vec<SpawnFn>),
  Empty
}

macro_rules! bundle_spawn {
    ($($bundle:tt)*) => {{
        Box::new(move |commands: &mut Commands| {
            commands.spawn($($bundle)*);
        })
    }};
}

fn generate_tile(noise: &Perlin, pos: IVec3) -> WorldTile {
  let loc = Location::from(pos);
  let wanderer = bundle_spawn!((Name::new("Wanderer"),
                                loc,
                                RandomMovement,
                                Visuals::sprite(MySprite::PLAYER)));
  let tree = bundle_spawn!((Name::new("tree"), loc, Visuals::sprite(MySprite::TREE)));
  // Perlin::get()
  let prob = |p| rand::random::<f32>() < p;
  let IVec3 { x, y, z } = pos;
  let surface_height = 5;
  let cave = noise.get([x as f64 / 20.0, y as f64 / 20.0, z as f64 / 20.0]) > 0.6;

  let height = surface_height;
  // let water_level = 4;

  if y > height {
    WorldTile::Empty
  } else if y == height {
    if prob(0.03) {
      WorldTile::BlockWithEntitiesOnTop(BlockType::Grass, vec![tree])
    } else if prob(0.01) {
      WorldTile::BlockWithEntitiesOnTop(BlockType::Grass, vec![wanderer])
    } else {
      WorldTile::Block(BlockType::Grass)
    }
  } else if cave {
    WorldTile::Empty
  } else if y > height - 3 {
    WorldTile::Block(BlockType::Dirt)
  } else {
    WorldTile::Block(BlockType::Stone)
  }
}

pub fn world_coords(bounds: IVec3) -> impl Iterator<Item = IVec3> {
  let (w, h, l) = (bounds.x, bounds.y, bounds.z);
  (-w..w).flat_map(move |x| {
           (-h..h).flat_map(move |y| (-l..l).map(move |z| IVec3 { x, y, z }))
         })
}

pub fn spawn_world(mut commands: Commands) {
  let noise = Perlin::new(5);
  let bounds = IVec3::new(128, 64, 128);
  let coords = world_coords(bounds);
  for (pos, tile) in coords.map(|pos| (pos, generate_tile(&noise, pos))) {
    match tile {
      WorldTile::Block(block) => {
        commands.spawn((Location(pos), block));
      }
      WorldTile::BlockWithEntitiesOnTop(block, spawns) => {
        commands.spawn((Location(pos), block));
        for spawn in spawns {
          spawn(&mut commands);
        }
      }
      WorldTile::Entities(spawns) => {
        for spawn in spawns {
          spawn(&mut commands);
        }
      }
      WorldTile::Empty => {}
    }
  }
}
