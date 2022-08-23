use std::marker::PhantomData;

use bevy::prelude::*;

/// 移動可能なエンティティに属するコンポーネント
#[derive(Component, Default)]
pub struct Mover<T> {
    pub speed: f32,
    pub dir: Vec3,
    _marker: PhantomData<T>,
}

impl<T> Mover<T> {
    pub fn new(speed: f32, dir: Vec3) -> Self {
        Self {
            speed,
            dir,
            _marker: PhantomData,
        }
    }
}

/// 移動方法の実装
pub trait Movable: Component {
    fn movement(&self, time: &Time, transform: &mut Transform);
}

/// 移動を行うシステム
pub fn movement_system<T: Movable>(time: Res<Time>, mut movers: Query<(&mut Transform, &T)>) {
    for (mut transform, mover) in movers.iter_mut() {
        mover.movement(time.as_ref(), transform.as_mut());
    }
}