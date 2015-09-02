extern crate geojson;
extern crate rustc_serialize;
extern crate rand;
extern crate docopt;

use docopt::Docopt;
use rand::Rng;
use std::env;
use rustc_serialize::json::Object;
use geojson::{Feature, FeatureCollection, GeoJson, Geometry, Value};

static USAGE: &'static str = "
Usage: rust-geojson-random <count>

Arguments:
    <count>  The number of features to generate.
";

#[derive(RustcDecodable)]
struct Args {
    count: usize
}

fn main() {

    let args: Args = Docopt::new(USAGE)
                  .and_then(|d| d.decode())
                  .unwrap_or_else(|e| e.exit());

    let mut features: Vec<geojson::Feature> = Vec::new();

    for x in 0..(args.count) {
        let geometry = Geometry::new(
            Value::Point(vec![
                (rand::random::<f64>() * 360.0) - 180.0,
                (rand::random::<f64>() * 180.0) - 90.0]));

        let mut properties = Object::new();

        let geojson = Feature {
            crs: None,
            bbox: None,
            id: None,
            geometry: geometry,
            properties: Some(properties),
        };
        features.push(geojson);
    }

    let collection = GeoJson::FeatureCollection(FeatureCollection {
        features: features,
        bbox: None,
        crs: None
    });

    print!("{}", collection.to_string());
}
