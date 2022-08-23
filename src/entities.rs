use crate::prelude::*;

/// エンティティのコマンド処理に関するプラグイン
pub struct EntitiesPlugin;

impl Plugin for EntitiesPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(despawn_entities::<DespawnType>)
            .add_system(advance_despawn_timer);
    }
}

/// 破棄可能状態の実装
pub trait Despawnable: Component {
    /// 破棄すべき状態にあるか
    fn should_despawn(&self) -> bool;
    /// 破棄直前に行うアクション
    fn on_before_despawn(&self) {}
}

/// エンティティの破棄タイプ
#[derive(Component, Default)]
pub enum DespawnType {
    Time(Timer),
    #[default]
    None,
}

impl Despawnable for DespawnType {
    fn should_despawn(&self) -> bool {
        match self {
            DespawnType::Time(t) => t.finished(),
            _ => false,
        }
    }
}

fn advance_despawn_timer(time: Res<Time>, mut timers: Query<&mut DespawnType>) {
    for mut t in timers.iter_mut() {
        match t.as_mut() {
            DespawnType::Time(ref mut t) => {
                t.tick(time.delta());
            }
            DespawnType::None => {}
        }
    }
}

/// 破棄可能判定を行い、必要であればエンティティの子階層を含め破棄する
fn despawn_entities<T: Despawnable>(mut commands: Commands, entities: Query<(Entity, &T)>) {
    for (e, despawnable) in entities.iter() {
        if despawnable.should_despawn() {
            despawnable.on_before_despawn();
            commands.entity(e).despawn_recursive();
        }
    }
}