use crate::*;
// 各pluginで共通して使用するstruct等

// プレイヤー・キャラクター。死んでるときがあるので、enumを持つだけ
pub struct CentipedeContainer {
    pub centipede: Centipede,
}

impl Default for CentipedeContainer {
    fn default() -> Self {
        Self {
            centipede: Centipede::Dead(0.0),
        }
    }
}

impl CentipedeContainer {
    pub fn alive(&self) -> Option<&Alive> {
        match &self.centipede {
            Centipede::Alive(alive) => Some(alive),
            _ => None,
        }
    }

    pub fn alive_mut(&mut self) -> Option<&mut Alive> {
        match &mut self.centipede {
            Centipede::Alive(alive) => Some(alive),
            _ => None,
        }
    }

    pub fn head_entity(&self) -> Option<Entity> {
        match &self.centipede {
            Centipede::Alive(alive) => Some(alive.head_entity),
            _ => None,
        }
    }
}

// プレイヤー・キャラクターの状態
pub enum Centipede {
    Alive(Alive),
    Dead(f64),
}

// 生きてる場合
pub struct Alive {
    pub head_entity: Entity,
    pub speed: f32,
    pub movement: Movement,
    pub last_move: Vec2,
    pub tail_count: usize,
    pub position_history: Vec<Position>,
}

impl Alive {
    pub fn default(head_entity: Entity) -> Self {
        Self {
            head_entity,
            speed: DEFAULT_SPEED,
            movement: Movement::Linear(Vec2 { x: 1.0, y: 0.0 }),
            last_move: Vec2 { x: 1.0, y: 0.0 },
            tail_count: INITIAL_CENTIPEDE_LENGTH,
            position_history: vec![
                Position {
                    x: -1000.0,
                    y: 0.0,
                    visible: true,
                },
                Position {
                    x: 0.0,
                    y: 0.0,
                    visible: true,
                },
            ],
        }
    }
}

// プレイヤー・キャラクターの現在の動き方
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Movement {
    // 中心地点
    Circular(CircularMove),
    // ベクトル
    Linear(Vec2),
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct CircularMove {
    pub center: Position,
    pub clockwise: bool,
}

// X: 左右(右が大きい)、Y:上下(上が大きい)、中心が0の座標
#[derive(Default, Copy, Clone, PartialEq, Debug)]
pub struct Position {
    pub x: f32,
    pub y: f32,
    pub visible: bool,
}

impl Position {
    pub fn default(visible: bool) -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            visible,
        }
    }
}

impl Position {
    // 二点間の距離
    pub fn distance(self, other: &Position) -> f32 {
        ((other.x - self.x).powf(2.0) + (other.y - self.y).powf(2.0)).sqrt()
    }

    // otherに向かってdistance動いた場所の座標
    pub fn forward_to(&self, other: &Position, distance: f32) -> Self {
        let vec2 = Vec2::from(*self).lerp(Vec2::from(*other), distance / self.distance(other));
        Position {
            x: vec2.x,
            y: vec2.y,
            visible: self.visible,
        }
    }

    // directionの方向へdistance動かす
    pub fn move_to_with_distance(&mut self, direction: Vec2, distance: f32) {
        let ratio = distance / (direction.x.powf(2.0) + direction.y.powf(2.0)).sqrt();
        self.x += direction.x * ratio;
        self.y += direction.y * ratio;
    }

    // directionへdelta_seconds秒、speedで動かす
    pub fn move_to_with_sec(&mut self, direction: Vec2, speed: f32, delta_seconds: f32) {
        self.move_to_with_distance(direction, speed * delta_seconds);
    }
}

impl From<Position> for Vec2 {
    fn from(p: Position) -> Self {
        Vec2 { x: p.x, y: p.y }
    }
}

impl From<Vec3> for Position {
    fn from(v: Vec3) -> Self {
        Position {
            x: v.x,
            y: v.y,
            visible: v.z > 0.0,
        }
    }
}

// http://www5d.biglobe.ne.jp/~tomoya03/shtml/algorithm/Intersection.htm
// 線分が交差するかの判定
pub fn intersection(a1: &Position, a2: &Position, b1: &Position, b2: &Position) -> bool {
    intersect(a1, a2, b1, b2) && intersect(b1, b2, a1, a2)
}

fn intersect(x1: &Position, x2: &Position, y1: &Position, y2: &Position) -> bool {
    ((x1.x - x2.x) * (y1.y - x1.y) + (x1.y - x2.y) * (x1.x - y1.x))
        * ((x1.x - x2.x) * (y2.y - x1.y) + (x1.y - x2.y) * (x1.x - y2.x))
        < 0.0
}

// 子要素を持つだけのコンテナバンドル
#[derive(Bundle)]
pub struct ContainerBundle {
    pub transform: Transform,
    pub global_transform: GlobalTransform,
}

impl Default for ContainerBundle {
    fn default() -> Self {
        Self {
            transform: Transform::from_translation(INVISIBLE_POSITION),
            global_transform: GlobalTransform::from_translation(INVISIBLE_POSITION),
        }
    }
}

pub fn void(_: In<Option<()>>) {}
