use crate::prelude::*;

/// 弾に関するプラグイン
pub struct BulletPlugin;

impl Plugin for BulletPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup)
            .add_system(movement_system::<Mover<Bullet>>);
    }
}

/// 初期設定を行う
fn setup(mut commands: Commands) {
    let mover = Mover::<Bullet>::new(50., Vec3::X);
    let despawn_type = DespawnType::Time(Timer::from_seconds(3., false));
    commands
        .spawn_bundle(BulletBundle::new(mover, despawn_type))
        .insert_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::WHITE,
                custom_size: Some(Vec2::new(16., 8.)),
                ..default()
            },
            ..default()
        });
}

#[derive(Component, Default)]
struct Bullet;

/// 弾バンドル
#[derive(Bundle, Default)]
struct BulletBundle {
    bullet: Bullet,
    mover: Mover<Bullet>,
    despawn_type: DespawnType,
}

impl BulletBundle {
    pub fn new(mover: Mover<Bullet>, despawn_type: DespawnType) -> Self {
        Self {
            bullet: Bullet,
            mover,
            despawn_type,
        }
    }
}

/// 弾の移動処理の実装
impl Movable for Mover<Bullet> {
    fn movement(&self, time: &Time, transform: &mut Transform) {
        if let Some(normalized_dir) = self.dir.try_normalize() {
            transform.translation += normalized_dir * self.speed * time.delta_seconds();
        }
    }
}