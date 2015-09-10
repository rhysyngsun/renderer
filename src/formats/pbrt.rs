use core::{
    Light,
    Scene,
};
use lights::{
    PointLight,
};
use linalg::Point3f64;
use primative::{
    TransformedPrimative,
};

use nom::{
    Consumer,
    ConsumerState,
};
use nom::{
    be_f64,
    eof,
    multispace,
};
use nom::IResult::*;

pub enum PbrtSceneConsumerState {
    Options,
    World,
    Done,
}

pub struct PbrtSceneConsumer {
    pub scene: Scene,
    pub state: PbrtSceneConsumerState,
}

impl PbrtSceneConsumer {
    pub fn new() -> PbrtSceneConsumer {
        PbrtSceneConsumer {
            scene: Scene::new(),
            state: PbrtSceneConsumerState::Options,
        }
    }
}

impl Consumer for PbrtSceneConsumer {
    fn consume(&mut self, input: &[u8]) -> ConsumerState {
        match self.state {
            PbrtSceneConsumerState::Options => {
                return ConsumerState::ConsumerError(0);
            },
            PbrtSceneConsumerState::World => {
                return ConsumerState::ConsumerError(0);
            },
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
