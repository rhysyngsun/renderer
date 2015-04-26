extern crate rustc_serialize;
extern crate docopt;

use docopt::Docopt;

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
    // TODO: cleanup
}
