mod pbrt;

use nom::{
    FileProducer,
    Consumer,
};

use core::Scene;

use self::pbrt::{
    PbrtSceneConsumer,
    PbrtSceneConsumerState,
};


pub fn parse(file: &String) -> Result<Scene, String> {
    match FileProducer::new(file, 500) {
        Result::Ok(mut fp) => {
            let mut sc = PbrtSceneConsumer::new();
            sc.run(&mut fp);

            match sc.state {
                PbrtSceneConsumerState::Done => {
                    return Result::Ok(sc.scene);
                },
                _ => {
                    return Result::Err(String::from("Error"));
                }
            }
        },
        Result::Err(_) => {
            return Result::Err(format!("Error loading file: {}", file));
        }
    }
}
