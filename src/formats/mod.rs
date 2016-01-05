mod pbrt;

use nom::{Producer, FileProducer, Consumer, ConsumerState};

use core::Scene;

use self::pbrt::{PbrtSceneConsumer, PbrtSceneConsumerState};


pub fn parse(file: &String) -> Result<Scene, String> {
    match FileProducer::new(file, 500) {
        Result::Ok(mut fp) => {
            let mut sc = PbrtSceneConsumer::new();

            while let &ConsumerState::Continue(_) = fp.apply(&mut sc) {
            }

            match sc.state {
                PbrtSceneConsumerState::Done => {
                    return Result::Ok(sc.scene);
                }
                _ => {
                    return Result::Err(String::from("Error"));
                }
            }
        }
        Result::Err(_) => {
            return Result::Err(format!("Error loading file: {}", file));
        }
    }
}
