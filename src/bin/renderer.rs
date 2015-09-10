extern crate rustc_serialize;
extern crate docopt;
extern crate renderer;

use docopt::Docopt;

use renderer::formats::parse;

// Write the Docopt usage string.
static USAGE: &'static str = "
Usage: renderer <scene-file>
";

#[derive(RustcDecodable, Debug)]
struct Args {
    arg_scene_file: String,
}

fn main() {
    let args: Args = Docopt::new(USAGE)
                            .and_then(|d| d.decode())
                            .unwrap_or_else(|e| e.exit());
    println!("{:?}", args);
    // TODO: init
    // TODO: parse scene
    match parse(&args.arg_scene_file) {
        Ok(scene) => println!("got scene"),
        Err(msg) => println!("Error parsing scene file: {:?}", msg),
    };
    // TODO: cleanup
}
