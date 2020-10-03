use crate::geojson::Feature;
use crate::geojson::FeatureCollection;
use std::fs;
use std::fs::File;
use serde_json::Result;
use serde::Deserialize;
use std::error::Error;
use std::io::BufReader;
use std::path::Path;
use geo::LineString;
use geo::Polygon;
use geo::Coordinate;
use std::io::{self, BufRead};
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::collections::HashMap;


//write a single feature to the web server
 pub fn write_feature_to_file(f: &Feature) -> std::io::Result<()> {
    let file = File::create("new_location.geojson")?;
    serde_json::to_writer(file, f)?;
    Ok(())
}

//write a feature collection to the web server
pub fn write_feature_collection_to_file(f: FeatureCollection) -> std::io::Result<()> {
    let file = File::create("locations.geojson")?;
    serde_json::to_writer(file, &f)?;
    Ok(())
}

//Reads a file that contains a series of coordinates in the format [lat,long],[lat,long],[lat,long]... and returns a Polygon object
pub fn get_polygon_from_file<T>(filename: &str, default_long: T, default_lat: T) -> Polygon<T>
where
    T: geo::CoordinateType,
    T: std::str::FromStr,
    T: std::fmt::Debug,
    <T as std::str::FromStr>::Err: std::fmt::Debug,
{
   
    //let mut vec: Vec<_> = vec!([default_long,default_lat]);
    let mut vec: Vec<_> = Vec::new();
    let mut coor: [T;2] = [default_long, default_lat];
    

    // I'm using the read_liines method described here:
    // https://doc.rust-lang.org/stable/rust-by-example/std_misc/file/read_lines.html
    if let Ok(lines) = read_lines(filename) {
                
        //Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(v) = line {
                //Skip the lines that contain '[', '],', and ']' 
                if (v != "[" && v != "]," && v != "]"){
                   //If the line ends with a ',' then it's a longitude coordinate
                   if (v.contains(",")) {
                        //Remove the ',' and then parse the string into a T (which is a float)
                        let long = v.replace(",","").parse::<T>().unwrap();
                        coor[0] = long;
                   } else { //else this is a latitude coordinate
                       let lat = v.parse::<T>().unwrap();
                       coor[1] = lat;
                       vec.push(coor);
                   }
                }
            }
        }
    }
    

    let ls: LineString<T> = vec.into(); 
    let p = Polygon::new(ls, vec![]); 
    p
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
// see https://doc.rust-lang.org/stable/rust-by-example/std_misc/file/read_lines.html
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn get_coordinate<T: geo::CoordinateType>(long: T, lat: T) -> geo::Coordinate<T> {
   let c = geo::Coordinate {
        x: long,
        y: lat,
   };

   c
}

pub fn save_location_description(description: &str, poly_file: &str) {
    //Writes a location description to the location_descriptions.txt file.
    //On each line, the file will contain a file description followed by a 
    //space followed by a polygon file.
    //e.g. buru buru_buru.txt
    
    let mut f = OpenOptions::new()
        .append(true)
        .open("location_descriptions.txt")
        .unwrap();

    if let Err(e) = writeln!(f, "{} => polygons/{}", description, poly_file) {
        eprintln!("Couldn't write to file: {}", e);
    }


}

pub fn get_hashmap_of_locations() -> HashMap<String, Polygon<f64>> {
    //Reads the location_descriptions file line by line
    //and returns a HashMap where the keys are location descriptions
    //and the values are polygons. e.g. Key: roysambu Value: 'a polygon of roysambu'


    let mut locations = HashMap::new();

    if let Ok(lines) = read_lines("location_descriptions.txt") {
       

        for line in lines {
            
            if let Ok(text) = line {
               
                //The part before '=>' is the description, the part
                //after is the polygon file
                
                let vec = text.split("=>").collect::<Vec<_>>();
                let description = vec[0].trim().to_string();
                let polygon_file = vec[1].trim();
                let polygon = get_polygon_from_file(polygon_file,0.0,0.0);
                locations.insert(description, polygon);

            }

        }

    }

    locations

}
