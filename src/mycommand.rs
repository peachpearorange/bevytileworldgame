use {crate::{Combat, Player},
     bevy::{ecs::world::Command, prelude::*},
     rust_utils::*};

pub fn insert_component<C: Component>(world: &mut World, entity: Entity, component: C) {
  if let Some(mut entity_mut) = world.get_entity_mut(entity) {
    entity_mut.insert(component);
  }
}

pub fn update_component<C: Component + Clone>(world: &mut World,
                                              entity: Entity,
                                              f: impl FnOnce(C) -> C) {
  if let Some(mut entity_mut) = world.get_entity_mut(entity) {
    if let Some(mut component) = entity_mut.get_mut::<C>() {
      let updated = f((*component).clone());
      *component = updated;
    }
  }
}

pub fn mutate_component<C: Component>(world: &mut World,
                                      entity: Entity,
                                      f: impl FnOnce(&mut C)) {
  if let Some(mut entity_mut) = world.get_entity_mut(entity) {
    if let Some(mut component) = entity_mut.get_mut::<C>() {
      f(&mut component);
    }
  }
}

pub fn get_player(world: &mut World) -> Option<Entity> {
  world.query_filtered::<Entity, With<Player>>()
       .iter(world)
       .next()
}

pub struct MyCommand(pub Box<dyn FnOnce(&mut World) + 'static + Send + Sync>);

// impl From<Box<dyn FnOnce(&mut World) + 'static + Send + Sync>> for MyCommand {
//   fn from(f: Box<dyn FnOnce(&mut World) + 'static + Send + Sync>) -> Self { MyCommand(f) }
// }

// impl<F> From<F> for MyCommand where F: FnOnce(&mut World) + 'static + Send + Sync {
//   fn from(f: F) -> Self { MyCommand(Box::new(f)) }
// }

impl<F> From<F> for MyCommand where F: FnOnce(&mut World) + 'static + Send + Sync {
  fn from(f: F) -> Self { MyCommand(Box::new(f)) }
}
impl MyCommand {
  pub fn none() -> Self { (|_world: &mut World| {}).into() }

  pub fn multi(commands: impl IntoIterator<Item = MyCommand>) -> Self {
    let v = vec(commands);
    (move |world: &mut World| {
      for command in v {
        command.0(world);
      }
    }).into()
  }

  // pub fn spawn(b: impl Bundle) -> Self {
  //   let spawnable: Spawnable = b.into();
  //   (move |world: &mut World| {
  //     let mut commands = world.commands();
  //     spawnable.spawn(&mut commands);
  //   }).into()
  // }

  // pub fn give_item_to_player(item: Item) -> Self {
  //   (move |world: &mut World| {
  //     if let Some(player_entity) = get_player(world) {
  //       mutate_component(world, player_entity, |inventory: &mut Inventory| {
  //         inventory.add_contents([(item.clone(), 1)]);
  //       });
  //     }
  //   }).into()
  // }

  // pub fn end_object_interaction_mini_game() -> Self {
  //   (|_world: &mut World| {
  //     // Implement mini-game ending logic here
  //   }).into()
  // }

  pub fn damage_entity(entity: Entity, amount: u32) -> Self {
    (move |world: &mut World| {
      if let Some(mut combat) = world.get_mut::<Combat>(entity) {
        combat.hp = combat.hp.saturating_sub(amount);
      }
    }).into()
  }

  // pub fn message_add(message: impl ToString + Send + Sync + 'static) -> Self {
  //   (move |world: &mut World| {
  //     if let Some(mut ui_data) = world.get_resource_mut::<UIData>() {
  //       ui_data.message_add(message.to_string().clone());
  //     }
  //   }).into()
  // }

  pub fn despawn_entity(entity: Entity) -> Self {
    (move |world: &mut World| {
      world.commands().entity(entity).despawn_recursive();
    }).into()
  }
  pub fn despawn(entity: Entity) -> Self {
    (move |world: &mut World| {
      world.commands().entity(entity).despawn_recursive();
    }).into()
  }

  pub fn insert_component<C: Component + 'static>(entity: Entity, component: C) -> Self {
    (move |world: &mut World| insert_component(world, entity, component)).into()
  }

  pub fn update_component<C: Component + Clone + 'static>(entity: Entity,
                                                          f: impl FnOnce(C) -> C
                                                            + 'static
                                                            + Send
                                                            + Sync)
                                                          -> Self {
    (move |world: &mut World| update_component(world, entity, f)).into()
  }

  pub fn mutate_component<C: Component + 'static>(entity: Entity,
                                                  f: impl FnOnce(&mut C)
                                                    + 'static
                                                    + Send
                                                    + Sync)
                                                  -> Self {
    (move |world: &mut World| mutate_component(world, entity, f)).into()
  }

  pub fn insert_player_component<C: Component + 'static>(component: C) -> Self {
    (move |world: &mut World| {
      if let Some(player_entity) = get_player(world) {
        insert_component(world, player_entity, component);
      }
    }).into()
  }

  pub fn update_player_component<C: Component + Clone + 'static>(f: impl FnOnce(C) -> C
                                                                   + 'static
                                                                   + Send
                                                                   + Sync)
                                                                 -> Self {
    (move |world: &mut World| {
      if let Some(player_entity) = get_player(world) {
        update_component(world, player_entity, f);
      }
    }).into()
  }

  pub fn mutate_player_component<C: Component + Clone + 'static>(f: impl FnOnce(&mut C)
                                                                   + 'static
                                                                   + Send
                                                                   + Sync)
                                                                 -> Self {
    (move |world: &mut World| {
      if let Some(player_entity) = get_player(world) {
        mutate_component(world, player_entity, f);
      }
    }).into()
  }
}

impl Command for MyCommand {
  fn apply(self, world: &mut World) { (self.0)(world); }
}
