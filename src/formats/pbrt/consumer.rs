use core::Scene;

use nom::{Consumer, ConsumerState, Input, Move};

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
