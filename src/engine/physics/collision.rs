use cgmath::Zero;
use ncollide3d::{
  na::{self, Isometry3, Translation3, UnitQuaternion},
  pipeline::{CollisionGroups, CollisionObjectSlabHandle, GeometricQueryType},
  query::{self, Contact, DefaultTOIDispatcher},
  shape::{Cuboid, ShapeHandle},
  world::CollisionWorld,
};
use std::{collections::HashMap, f32::consts::PI};

use crate::engine::{GameObject, Transform};

#[derive(Clone, Copy, Debug)]
pub enum EventStatus {
  Enter,
  Stay,
  Leave,
  None,
}

#[derive(Clone, Copy)]
pub struct CollisionEvent {
  pub status: EventStatus,
  pub depth: f32,
  pub normal: cgmath::Vector3<f32>,
  pub other_handle: CollisionObjectSlabHandle,
  pub other_tag: Tag,
}

#[derive(Copy, Clone)]
pub enum Tag {
  Player,
  Platform,
  None,
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

  pub fn update_object(&mut self, object: &mut GameObject) {
    let mut events =
      HashMap::<CollisionObjectSlabHandle, (CollisionObjectSlabHandle, Contact<f32>)>::new();

    let collision_object = self.world.get_mut(object.collision_handle).unwrap();
    collision_object.set_position(get_isometry(&object.transform));

    self.world.update();

    for pair in self.world.contact_pairs(true) {
      events.insert(pair.0, (pair.1, pair.3.deepest_contact().unwrap().contact));
      events.insert(pair.1, (pair.0, pair.3.deepest_contact().unwrap().contact));
    }

    if let Some((other_handle, contact)) = events.get(&object.collision_handle) {
      let status = match object.collision.status {
        EventStatus::None => EventStatus::Enter,
        EventStatus::Enter => EventStatus::Stay,
        EventStatus::Stay => EventStatus::Stay,
        EventStatus::Leave => EventStatus::Enter,
      };
      object.collision = CollisionEvent {
        status,
        depth: contact.depth,
        normal: cgmath::Vector3 {
          x: contact.normal.x,
          y: contact.normal.y,
          z: contact.normal.z,
        },
        other_handle: *other_handle,
        other_tag: Tag::Platform,
      };
    } else {
      if let EventStatus::Enter | EventStatus::Stay = object.collision.status {
        object.collision = CollisionEvent {
          status: EventStatus::Leave,
          ..object.collision
        };
      } else {
        object.collision = CollisionEvent {
          status: EventStatus::None,
          depth: 0.0,
          normal: cgmath::Vector3::zero(),
          other_handle: CollisionObjectSlabHandle(0),
          other_tag: Tag::None,
        };
      }
    }
  }

  pub fn get_toi(
    &mut self,
    object: &mut Transform,
    vel: cgmath::Vector3<f32>,
    other_handle: CollisionObjectSlabHandle,
    dist: f32,
  ) -> f32 {
    let cgmath::Vector3 { x, y, z } = object.scale;
    let collider = Cuboid::new(na::Vector3::<f32>::new(x, y, z));
    let cgmath::Vector3 { x, y, z } = vel;
    let velocity = na::Vector3::<f32>::new(x, y, z);

    let collision_object = self.world.get_mut(other_handle).unwrap();
    let shape = collision_object.shape().as_shape::<Cuboid<f32>>().unwrap();
    if let Ok(Some(toi)) = query::time_of_impact::<f32>(
      &DefaultTOIDispatcher,
      &get_isometry(object),
      &velocity,
      &collider,
      collision_object.position(),
      &na::Vector3::<f32>::new(0.0, 0.0, 0.0),
      shape,
      1.0,
      dist,
    ) {
      return toi.toi;
    }
    1.0
  }

  pub fn add_collider(&mut self, object: &Transform, tag: &Tag) -> CollisionObjectSlabHandle {
    let position = get_isometry(object);

    let cgmath::Vector3 { x, y, z } = object.scale;
    let collider = ShapeHandle::new(Cuboid::new(na::Vector3::<f32>::new(x, y, z)));

    let collision_group = match tag {
      Tag::Player => self.player_group,
      Tag::Platform => self.platform_group,
      _ => self.platform_group,
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

fn get_isometry(obj: &Transform) -> Isometry3<f32> {
  let cgmath::Vector3 { x, y, z } = obj.position;
  let position = Translation3::from(na::Vector3::<f32>::new(x, y, z));
  let cgmath::Vector3 { x, y, z } = obj.rotation;
  let rotation = UnitQuaternion::from_euler_angles(x * PI / 180.0, y * PI / 180.0, z * PI / 180.0);

  Isometry3::from_parts(position, rotation)
}
