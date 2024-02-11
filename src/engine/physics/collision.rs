use ncollide3d::{
  na::{self, Isometry3, Translation3, UnitQuaternion},
  pipeline::{CollisionGroups, CollisionObjectSlabHandle, ContactEvent, GeometricQueryType},
  shape::{Cuboid, ShapeHandle},
  world::CollisionWorld,
};
use std::{collections::HashMap, f32::consts::PI};

use crate::engine::{GameObject, Transform};

#[derive(Clone, Copy)]
pub struct CollisionEvent {
  pub colliding: bool,
  pub depth: f32,
  pub normal: cgmath::Vector3<f32>,
  handle: CollisionObjectSlabHandle,
  other_handle: CollisionObjectSlabHandle,
  pub other_tag: Tag,
}

#[derive(Copy, Clone)]
pub enum Tag {
  Player,
  Platform,
}

pub struct Collision {
  world: CollisionWorld<f32, Tag>,
  player_group: CollisionGroups,
  platform_group: CollisionGroups,
  contacts_query: GeometricQueryType<f32>,
}

impl Collision {
  pub fn new() -> Self {
    let mut player_group = CollisionGroups::new();
    player_group.set_membership(&[1]);
    let mut platform_group = CollisionGroups::new();
    platform_group.set_membership(&[2]);
    platform_group.set_whitelist(&[1]);

    let world = CollisionWorld::new(0.02);

    let contacts_query = GeometricQueryType::Contacts(0.0, 0.0);

    Self {
      world,
      player_group,
      platform_group,
      contacts_query,
    }
  }

  pub fn update(&mut self, objects: Vec<&mut GameObject>) {
    let mut events = HashMap::<CollisionObjectSlabHandle, CollisionEvent>::new();

    for object in &objects {
      let collision_object = self.world.get_mut(object.collision_handle).unwrap();
      collision_object.set_position(get_isometry(&object.transform));
    }

    self.world.update();

    for event in self.world.contact_events() {
      let collision_event = handle_contact_event(&self.world, event);
      if let Some(e) = collision_event {
        events.insert(e.handle, e);
      }
    }

    for object in objects {
      if let Some(event) = events.get(&object.collision_handle) {
        object.collision = Some(*event);
      } else {
        object.collision = None;
      }
    }
  }

  pub fn add_collider(&mut self, object: &Transform, tag: &Tag) -> CollisionObjectSlabHandle {
    let position = get_isometry(object);

    let cgmath::Vector3 { x, y, z } = object.scale;
    let collider = ShapeHandle::new(Cuboid::new(na::Vector3::<f32>::new(x, y, z)));

    let collision_group = match tag {
      Tag::Player => self.player_group,
      Tag::Platform => self.platform_group,
    };

    let collision_data = *tag;

    let (handle, _) = self.world.add(
      position,
      collider,
      collision_group,
      self.contacts_query,
      collision_data,
    );

    handle
  }
}

fn handle_contact_event(
  world: &CollisionWorld<f32, Tag>,
  event: &ContactEvent<CollisionObjectSlabHandle>,
) -> Option<CollisionEvent> {
  if let ContactEvent::Started(collider1, collider2) = event {
    let pair = match world.contact_pair(*collider1, *collider2, true) {
      None => return None,
      Some(pair) => pair,
    };

    let co1 = world.collision_object(*collider1).unwrap();
    let co2 = world.collision_object(*collider2).unwrap();

    if let Tag::Player = co1.data() {
      let contact = pair.3.deepest_contact().unwrap().contact;

      return Some(CollisionEvent {
        colliding: contact.depth > 0.0,
        depth: contact.depth,
        normal: cgmath::Vector3 {
          x: contact.normal.x,
          y: contact.normal.y,
          z: contact.normal.z,
        },
        handle: pair.0,
        other_handle: pair.1,
        other_tag: *co2.data(),
      });
    }
    if let Tag::Player = co2.data() {
      let contact = pair.3.deepest_contact().unwrap().contact;

      return Some(CollisionEvent {
        colliding: contact.depth > 0.0,
        depth: contact.depth,
        normal: cgmath::Vector3 {
          x: contact.normal.x,
          y: contact.normal.y,
          z: contact.normal.z,
        },
        handle: pair.1,
        other_handle: pair.0,
        other_tag: *co1.data(),
      });
    }
  }
  return None;
}

fn get_isometry(obj: &Transform) -> Isometry3<f32> {
  let cgmath::Vector3 { x, y, z } = obj.position;
  let position = Translation3::from(na::Vector3::<f32>::new(x, y, z));
  let cgmath::Vector3 { x, y, z } = obj.rotation;
  let rotation = UnitQuaternion::from_euler_angles(x * PI / 180.0, y * PI / 180.0, z * PI / 180.0);

  Isometry3::from_parts(position, rotation)
}
