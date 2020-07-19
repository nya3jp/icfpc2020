use self::super::value::*;

pub const THRUST_COMMAND: i128 = 0;
pub const SELF_DESTRUCT_COMMAND: i128 = 1;
pub const BEAM_COMMAND: i128 = 2;
pub const SPLIT_COMMAND: i128 = 3;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Point {
    pub x: isize,
    pub y: isize,
}

impl Point {
    pub fn to_value(&self) -> Value {
        Value::Cons(
            Box::new(Value::Int(self.x as i128)),
            Box::new(Value::Int(self.y as i128)),
        )
    }
    pub fn sub(&self, p: Point) -> Point {
        Point{ x: self.x - p.x, y: self.y - p.y }
    }
    pub fn add(&self, p: Point) -> Point {
        Point{ x: self.x + p.x, y: self.y + p.y }
    }
    pub fn l0_distance(&self) -> isize {
        std::cmp::max(self.x.abs(), self.y.abs())
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Command {
    // Thrust(ShipNum, Point{x, y})
    Thrust(i8, Point),
    // SelfDestruct(ShipNum)
    SelfDestruct(i8),
    // Beam(ShipNum, X, Y, Power)
    Beam(i8, Point, i8),
    // Split
    Split(Param),
}

impl Command {
    pub fn to_value(&self) -> Value {
        match self {
            //   send: [0, SHIP_NUM, (X . Y)]
            &Command::Thrust(ship_num, ref pos) => Value::Cons(
                Box::new(Value::Int(THRUST_COMMAND)),
                Box::new(Value::Cons(
                    Box::new(Value::Int(ship_num as i128)),
                    Box::new(Value::Cons(Box::new(pos.to_value()), Box::new(Value::Nil))),
                )),
            ),
            // send [1, SHIP_NUM]
            &Command::SelfDestruct(ship_num) => Value::Cons(
                Box::new(Value::Int(SELF_DESTRUCT_COMMAND)),
                Box::new(Value::Cons(
                    Box::new(Value::Int(ship_num as i128)),
                    Box::new(Value::Nil),
                )),
            ),
            // send: [2, SHIP_NUM, ( X . Y ), POWER] =>
            &Command::Beam(ship_num, point, power) => Value::Cons(
                Box::new(Value::Int(BEAM_COMMAND)),
                Box::new(Value::Cons(
                    Box::new(Value::Int(ship_num as i128)),
                    Box::new(Value::Cons(
                        Box::new(point.to_value()),
                        Box::new(Value::Cons(
                            Box::new(Value::Int(power as i128)),
                            Box::new(Value::Nil),
                        )),
                    )),
                )),
            ),
            &Command::Split(ref param) => Value::Cons(
                Box::new(Value::Int(SPLIT_COMMAND)),
                Box::new(Value::Cons(
                    Box::new(Value::Int(param.energy as i128)),
                    Box::new(Value::Cons(
                        Box::new(Value::Int(param.laser_power as i128)),
                        Box::new(Value::Cons(
                            Box::new(Value::Int(param.cool_down_per_turn as i128)),
                            Box::new(Value::Cons(
                                Box::new(Value::Int(param.life as i128)),
                                Box::new(Value::Nil),
                            )),
                        )),
                    )),
                )),
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
pub struct Obstacle {
    // 重力源の半径 (|x| と |y| がともにこれ以下になると死. 移動中にかすめてもセーフ),
    pub gravity_radius: usize,
    // ステージの半径 (|x| か |y| どちらかがこれを超えると死)
    pub stage_half_size: usize,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct StageData {
    pub total_turns: usize,
    pub role: isize, // 0: you are attacker. 1: defender.
    pub _2: (isize, isize, isize),
    pub obstacle: Option<Obstacle>,
    pub _3: Vec<isize>,
}

// deserialized response.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Response {
    pub current_game_state: CurrentGameState,
    pub stage_data: StageData,
    pub current_state: Option<CurrentState>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
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

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Machine {
    // 0 attacker, 1 diffender
    pub team_id: isize,
    // 機体 ID. 多分自陣営/敵陣営通して unique.
    pub machine_id: isize,
    pub position: Point,
    pub velocity: Point,
    pub params: Param,
    // 0-64
    pub heat: usize,
    pub _1: isize,
    pub _2: isize,

    pub generated_heat: isize,
    pub attack_heat: isize
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
        // TODO
    },
    // 3
    Split {
        params: Param,
    },
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CurrentState {
    pub turn: usize, // 現在のターン数
    pub obstacle: Option<Obstacle>,
    pub machines: Vec<(Machine, Option<ActionResult>)>,
}
