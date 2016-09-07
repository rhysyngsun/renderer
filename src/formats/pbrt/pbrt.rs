use core::Scene;
use linalg::{Transform, Vector3};

use nom::{Consumer, ConsumerState, Input, Move};
use nom::{digit, space};
use nom::IResult::Done;

use std::str::from_utf8;
use std::str::FromStr;

use formats::param_set::{ParamSet, ParamType};


/// TODO: replace this nom impl with a better parser engine

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


// float parsers
named!(flt_no_dot<&[u8], (&str, &str)>,
       chain!(
           b: map_res!(digit, from_utf8),
           || { return (b, "") }));

named!(flt_w_dot<&[u8], (&str, &str)>,
       chain!(
           b: map_res!(digit, from_utf8) ~
           tag!(".") ~
           a: map_res!(digit, from_utf8),
           || { return (b, a) }));

named!(flt<&[u8], (&str, &str)>,
      alt_complete!(flt_w_dot | flt_no_dot));

named!(signed_f64<&[u8], Result<f64, <f64 as FromStr>::Err> >,
       dbg_dmp!(chain!(
           s: opt!(map_res!(tag!("-"), from_utf8)) ~
           d: flt,
           || {
                let sign = s.unwrap_or("");
                let (before, after) = d;
                (&format!("{}{}.{}", sign, before, after)[..]).parse::<f64>() })));

named!(flt64<&[u8], f64>,
       map_res!(signed_f64,
                |r: Result<f64, <f64 as FromStr>::Err>| {
                    r
                }));


named!(vector3<&[u8], Vector3>,
       chain!(
           x: flt64 ~ space ~
           y: flt64 ~ space ~
           z: flt64,
           || { Vector3::new(x, y, z) }));

named!(look_at<&[u8], Transform>,
       chain!(
           tag!("LookAt") ~ space ~
           eye: vector3 ~ space ~
           look: vector3 ~ space ~
           up: vector3,
           || { Transform::look_at(eye, look, up) }));

named!(transformation<&[u8], Transform>,
       alt!(look_at));

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

    #[allow(unused_variables)]
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
fn test_signed_f64() {
    assert_eq!(signed_f64(&b"1"[..]), Done(&b""[..], Result::Ok(1.0)));
    assert_eq!(signed_f64(&b"0"[..]), Done(&b""[..], Result::Ok(0.0)));
    assert_eq!(signed_f64(&b"0.145"[..]), Done(&b""[..], Result::Ok(0.145)));
    assert_eq!(signed_f64(&b"2.45"[..]), Done(&b""[..], Result::Ok(2.45)));
}

#[cfg(test)]
#[test]
fn test_vector3() {
    assert_eq!(vector3(&b"0.0 10.0 100.0"[..]),
               Done(&b""[..], Vector3::new(0.0, 10.0, 100.0)));

    assert_eq!(vector3(&b"0 10 100"[..]),
               Done(&b""[..], Vector3::new(0.0, 10.0, 100.0)));

    assert_eq!(vector3(&b"0 -10 100"[..]),
               Done(&b""[..], Vector3::new(0.0, -10.0, 100.0)));
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
