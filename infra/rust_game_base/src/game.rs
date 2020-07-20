use self::super::value::*;
use std::ops::{Add, AddAssign, Neg, Sub, SubAssign};

pub const THRUST_COMMAND: i128 = 0;
pub const SELF_DESTRUCT_COMMAND: i128 = 1;
pub const BEAM_COMMAND: i128 = 2;
pub const SPLIT_COMMAND: i128 = 3;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Default, Hash)]
pub struct Point {
    pub x: isize,
    pub y: isize,
}

impl Point {
    pub const fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }

    pub fn to_value(&self) -> Value {
        Value::Cons(
            Box::new(Value::Int(self.x as i128)),
            Box::new(Value::Int(self.y as i128)),
        )
    }
    pub fn l0_distance(&self) -> isize {
        std::cmp::max(self.x.abs(), self.y.abs())
    }
    pub fn dist(self, p: Point) -> f64 {
        (self.dist2(p) as f64).sqrt()
    }
    pub fn dist2(self, p: Point) -> isize {
        (self - p).norm2()
    }
    pub fn norm2(self) -> isize {
        let (x, y) = (self.x, self.y);
        x * x + y * y
    }
    pub fn norm(self) -> f64 {
        Point::new(0, 0).dist(self)
    }
}

impl Add for Point {
    type Output = Point;
    fn add(self, p: Point) -> Point {
        Point {
            x: self.x + p.x,
            y: self.y + p.y,
        }
    }
}

impl AddAssign for Point {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl Sub for Point {
    type Output = Point;
    fn sub(self, p: Point) -> Point {
        Point {
            x: self.x - p.x,
            y: self.y - p.y,
        }
    }
}

impl Neg for Point {
    type Output = Point;
    fn neg(self) -> Self::Output {
        return Point {
            x: -self.x,
            y: -self.y,
        };
    }
}

impl SubAssign for Point {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Command {
    // Thrust(ShipNum, Point{x, y})
    Thrust(isize, Point),
    // SelfDestruct(ShipNum)
    Bomb(isize),
    // Beam(ShipNum, X, Y, Power)
    Beam(isize, Point, isize),
    // Split(ShipNum, Param)
    Split(isize, Param),
}

impl Command {
    pub fn to_value(&self) -> Value {
        use crate::dsl::*;

        match self {
            //   send: [0, SHIP_NUM, (X . Y)]
            &Command::Thrust(ship_num, ref pos) => {
                list!(int(THRUST_COMMAND), int(ship_num), pos.to_value())
            }
            // send [1, SHIP_NUM]
            &Command::Bomb(ship_num) => list!(int(SELF_DESTRUCT_COMMAND), int(ship_num)),
            // send: [2, SHIP_NUM, ( X . Y ), POWER] =>
            &Command::Beam(ship_num, point, power) => list!(
                int(BEAM_COMMAND),
                int(ship_num),
                point.to_value(),
                int(power)
            ),
            &Command::Split(ship_num, ref param) => list!(
                int(SPLIT_COMMAND),
                int(ship_num),
                list!(
                    int(param.energy),
                    int(param.laser_power),
                    int(param.cool_down_per_turn),
                    int(param.life)
                )
            ),
        }
    }
}

// 0/START, 1/PLAYING, 2/END (cf: 公式)
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CurrentGameState {
    START,
    PLAYING,
    END,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Role {
    ATTACKER,
    DEFENDER,
}

impl Default for Role {
    fn default() -> Self {
        Role::ATTACKER
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Obstacle {
    // 重力源の半径 (|x| と |y| がともにこれ以下になると死. 移動中にかすめてもセーフ),
    pub gravity_radius: usize,
    // ステージの半径 (|x| か |y| どちらかがこれを超えると死)
    pub stage_half_size: usize,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct StageData {
    pub total_turns: usize,
    pub self_role: Role, // whether you're an attacker or a defender.
    pub initialize_param: InitializeParam,
    pub obstacle: Option<Obstacle>,
    pub defender: Option<Param>, // Attacker can receive this.
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct InitializeParam {
    pub total_cost: usize,
    pub _2: isize,
    pub _3: isize,
}

// deserialized response.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Response {
    pub current_game_state: CurrentGameState,
    pub stage_data: StageData,
    pub current_state: Option<CurrentState>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Default)]
pub struct Param {
    // コレがなくなると、 Thruster が吹けない
    pub energy: usize,
    // 0 だとそもそも撃てない
    pub laser_power: usize,
    // 毎ターンHeatが減少
    pub cool_down_per_turn: usize,
    // 分裂可能だと2, 分裂ソース? (死んだ時に 0)
    pub life: usize,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Default)]
pub struct Machine {
    pub role: Role,
    // 機体 ID. 多分自陣営/敵陣営通して unique.
    pub machine_id: isize,
    pub position: Point,
    pub velocity: Point,
    pub params: Param,
    // 0-64
    pub heat: usize,
    pub _1: isize,
    pub _2: isize,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ActionResult {
    // tag = 0
    Thruster {
        // 加速度
        a: Point,
    },
    // 1
    Bomb {
        power: usize,
        area: usize,
    },
    // 2
    Laser {
        opponent: Point,
        power: usize,     // The cost the player paid.
        intensity: usize, // The instensity of the laser at the target.
        _3: isize,        // what...?
    },
    // 3
    Split {
        params: Param,
    },
}

#[derive(Clone, Debug, Eq, PartialEq, Default)]
pub struct CurrentState {
    pub turn: usize, // 現在のターン数
    pub obstacle: Option<Obstacle>,
    pub machines: Vec<(Machine, Vec<ActionResult>)>,
}

// Utilities.

// Returns machine ids of the given role.
pub fn get_roled_machine_ids(state: &CurrentState, role: Role) -> Vec<isize> {
    state
        .machines
        .iter()
        .filter_map(|(m, _)| {
            if m.role == role {
                Some(m.machine_id)
            } else {
                None
            }
        })
        .collect::<Vec<_>>()
}

pub fn get_machine_by_id(state: &CurrentState, machine_id: isize) -> Option<&Machine> {
    state
        .machines
        .iter()
        .find(|(m, _)| m.machine_id == machine_id)
        .map(|(m, _)| m)
}
