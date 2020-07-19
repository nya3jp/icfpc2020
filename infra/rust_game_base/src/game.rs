use self::super::value::*;

pub const THRUST_COMMAND: i128 = 0;
pub const SELF_DESTRUCT_COMMAND: i128 = 1;
pub const BEAM_COMMAND: i128 = 2;

#[derive(Debug)]
pub enum Command {
    // Thrust(ShipNum, X, Y)
    Thrust(i8, i8, i8),
    // SelfDestruct(ShipNum)
    SelfDestruct(i8),
    // Beam(ShipNum, X, Y, Power)
    Beam(i8, i16, i16, i8),
}

impl Command {
    pub fn to_value(&self) -> Value {
        match self {
            //   send: [0, SHIP_NUM, (X . Y)]
            &Command::Thrust(ship_num, x, y) => Value::Cons(
                Box::new(Value::Int(THRUST_COMMAND)),
                Box::new(Value::Cons(
                    Box::new(Value::Int(ship_num as i128)),
                    Box::new(Value::Cons(
                        Box::new(Value::Cons(
                            Box::new(Value::Int(x as i128)),
                            Box::new(Value::Int(y as i128)),
                        )),
                        Box::new(Value::Nil),
                    )),
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
            &Command::Beam(ship_num, x, y, power) => Value::Cons(
                Box::new(Value::Int(BEAM_COMMAND)),
                Box::new(Value::Cons(
                    Box::new(Value::Int(ship_num as i128)),
                    Box::new(Value::Cons(
                        Box::new(Value::Cons(
                            Box::new(Value::Int(x as i128)),
                            Box::new(Value::Int(y as i128)),
                        )),
                        Box::new(Value::Cons(
                            Box::new(Value::Int(power as i128)),
                            Box::new(Value::Nil),
                        )),
                    )),
                )),
            ),
        }
    }
}

#[derive(Debug)]
pub struct GameState {
    pub state1: Value,
    pub state2: Value,
}
