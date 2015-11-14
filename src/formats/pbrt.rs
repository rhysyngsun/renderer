use core::{Light, Scene};
use lights::PointLight;
use linalg::{Point3, Transform};
use primative::TransformedPrimative;

use nom::{Consumer, ConsumerState};
use nom::{be_f64, eof, eol, multispace, space};
use nom::IResult::*;

pub enum PbrtSceneConsumerState {
    Options,
    World,
    Done,
}

pub struct PbrtSceneConsumer {
    pub scene: Scene,
    pub state: PbrtSceneConsumerState,

    t: Transform,
}

enum PbrtTransformation {
    LookAt {
        ex: f64,
        ey: f64,
        ez: f64,
        lx: f64,
        ly: f64,
        lz: f64,
        ux: f64,
        uy: f64,
        uz: f64,
    },
}

named!(look_at<&[u8], PbrtTransformation>,
    chain!(
        tag!("LookAt") ~ space ~
            ex: be_f64 ~ space ~
            ey: be_f64 ~ space ~
            ez: be_f64 ~ space ~

            lx: be_f64 ~ space ~
            ly: be_f64 ~ space ~
            lz: be_f64 ~ space ~

            ux: be_f64 ~ space ~
            uy: be_f64 ~ space ~
            uz: be_f64 ~ space ~ eol,
        || { PbrtTransformation::LookAt {
                ex: ex, ey: ey, ez: ez,
                lx: lx, ly: ly, lz: lz,
                ux: ux, uy: uy, uz: uz,
            }
        }
    )
);

named!(transformation<&[u8], PbrtTransformation>,
    alt!(
        look_at
    )
);

impl PbrtSceneConsumer {
    pub fn new() -> PbrtSceneConsumer {
        PbrtSceneConsumer {
            scene: Scene::new(),
            state: PbrtSceneConsumerState::Options,

            t: Transform::identity(),
        }

    }
}

impl Consumer for PbrtSceneConsumer {
    fn consume(&mut self, input: &[u8]) -> ConsumerState {
        match self.state {
            PbrtSceneConsumerState::Options => {
                return ConsumerState::ConsumerError(0);
            }
            PbrtSceneConsumerState::World => {
                return ConsumerState::ConsumerError(0);
            }
            PbrtSceneConsumerState::Done => {
                return ConsumerState::ConsumerError(0);
            }
        }
    }
    fn end(&mut self) {
    }
    fn failed(&mut self, error_code: u32) {
    }
}
