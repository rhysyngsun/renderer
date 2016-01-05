use core::{Light, Scene};
use lights::PointLight;
use linalg::{Point3, Transform, Vector3};
use primative::TransformedPrimative;

use nom::{Consumer, ConsumerState, Input, Move, ErrorKind};
use nom::{is_digit, digit, eof, eol, multispace, space};
use nom::Err::*;
use nom::IResult;
use nom::IResult::*;

use std::str;
use std::str::FromStr;

pub enum PbrtSceneConsumerState {
    Options,
    World,
    Done,
}

pub struct PbrtSceneConsumer {
    pub scene: Scene,
    pub state: PbrtSceneConsumerState,

    c_state: ConsumerState<usize, (), Move>,

    t: Transform,
}

#[derive(PartialEq)]
enum FloatRepType {
    Integer,
    Decimal,
}

fn decimal(input: &[u8]) -> IResult<&[u8], &[u8]> {
    let mut rep_type = FloatRepType::Integer;
    for (idx, item) in input.iter().enumerate() {
        if !is_digit(*item) {
            if idx == 0 && *item != '-' as u8 {
                return Error(Position(ErrorKind::Custom(0), input));
            } else if rep_type == FloatRepType::Integer {
                rep_type = FloatRepType::Decimal;
            } else {
                return Done(&input[idx..], &input[0..idx]);
            }
        }
    }
    Done(b"", input)
}

named!(str_f64<&[u8], f64>,
       map_res!(
           map_res!(decimal, str::from_utf8)
        , FromStr::from_str)
);

named!(vector3<&[u8], Vector3>,
    dbg_dmp!(chain!(
        x: str_f64 ~ space ~
        y: str_f64 ~ space ~
        z: str_f64,
        || { Vector3::new(x, y, z)}
       )));

named!(look_at<&[u8], Transform>,
    chain!(
        tag!("LookAt") ~ space ~
            eye: vector3 ~ space ~
            look: vector3 ~ space ~
            up: vector3 ~ space ~ eol,
        || { Transform::look_at(eye, look, up) }
    )
);

named!(transformation<&[u8], Transform>,
    alt!(
        look_at
    )
);

impl PbrtSceneConsumer {
    pub fn new() -> PbrtSceneConsumer {
        PbrtSceneConsumer {
            scene: Scene::new(),
            state: PbrtSceneConsumerState::Options,

            c_state: ConsumerState::Continue(Move::Consume(0)),

            t: Transform::identity(),
        }

    }
}

impl<'a> Consumer<&'a [u8], usize, (), Move> for PbrtSceneConsumer {
    fn state(&self) -> &ConsumerState<usize, (), Move> {
        &self.c_state
    }

    fn handle(&mut self, input: Input<&[u8]>) -> &ConsumerState<usize, (), Move> {
        match self.state {
            PbrtSceneConsumerState::Options => {
                self.c_state = ConsumerState::Error(());
            }
            PbrtSceneConsumerState::World => {
                self.c_state = ConsumerState::Error(());
            }
            PbrtSceneConsumerState::Done => {
                self.c_state = ConsumerState::Error(());
            }
        }
        &self.c_state
    }
}

#[cfg(test)]
#[test]
fn test_str_f64() {
    assert_eq!(str_f64(&b"0"[..]), Done(&b""[..], 0.0));
    assert_eq!(str_f64(&b"4.23230"[..]), Done(&b""[..], 4.2323));
}

#[cfg(test)]
#[test]
fn test_vector3() {
    assert_eq!(vector3(&b"0.0 10.0 100.0"[..]),
               Done(&b""[..], Vector3::new(0.0, 10.0, 100.0)));
}

#[cfg(test)]
#[test]
fn test_look_at() {
    assert_eq!(look_at(&b"LookAt 0 10 100 0 -1 0 0 1 0"[..]),
               Done(&b""[..],
                    Transform::look_at(Vector3::new(0.0, 10.0, 100.0),
                                       Vector3::new(0.0, -1.0, 0.0),
                                       Vector3::new(0.0, 1.0, 0.0))));
}
