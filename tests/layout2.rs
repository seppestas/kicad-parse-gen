// (c) 2016-2017 Productize SPRL <joost@productize.be>

extern crate kicad_parse_gen as kicad;
#[macro_use]
extern crate log;
extern crate difference;
extern crate env_logger;

use kicad::BoundingBox;
use std::path::PathBuf;
use std::env;
use difference::Changeset;

#[test]
fn parse_and_compare() {
    env::set_var("RUST_LOG","trace");
    env_logger::init().unwrap(); 
    let mut file_name = String::new();
    file_name.push_str(env!("CARGO_MANIFEST_DIR"));
    file_name.push_str("/tests/data/");
    file_name.push_str("layout2.kicad_pcb");
    let file_name = PathBuf::from(file_name);
    
    let content = kicad::read_file(&file_name).unwrap();
    
    let layout = kicad::read_layout(&file_name).unwrap();
    let s = kicad::layout::layout_to_string(&layout, 0).unwrap();

    kicad::write_file("/tmp/dump.kicad_pcb", &s).unwrap();

    let changeset = Changeset::new(&content, &s, "\n");
    if changeset.distance > 1 {
        println!("{}", changeset);
        assert_eq!(changeset.distance, 1);
    }

    let b = layout.bounding_box();
    assert_eq!(b.x1, 99.1875);
    assert_eq!(b.y1, 99.6);
    assert_eq!(b.x2, 101.9);
    assert_eq!(b.y2, 100.9);
    info!("bound: {:?}", b);
}
