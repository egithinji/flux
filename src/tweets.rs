use crate::geojson::GeojsonType;
use crate::geojson::Feature;
use crate::geojson::Geometry;
use crate::geojson::Properties;
use std::fs;
use std::fs::File;
use std::io::prelude::*;

//To hold relevant data pertaining to a tweet
pub struct Tweet {
    pub location: [f64;2],
    pub posted_on: String,
    pub text: String,
    pub user_id: u64,
    pub area: String,
}

impl Tweet {
    //return a Feature that corresponds to this tweet
    pub fn to_feature(self) -> Feature {
        //create a geometry struct (needed by 'feature' struct)
        let g = Geometry {
            r#type: GeojsonType::Point,
            coordinates: self.location,
        };

        //create a properties struct (needed by 'feature' struct)
        let p = Properties {
            posted_on: self.posted_on,
            text: self.text,
            area: self.area,
            user_id: self.user_id,
        };

        //get the current tweet count, increment by one to get
        //this feature's id.
        let content = fs::read_to_string("tweet_count.txt").expect("Something went wrong reading tweet_count.txt file.");
        let trimmed_content = content.trim(); 
        let new_id = trimmed_content.parse::<usize>().unwrap() + 1;

        //write the new id to the file
        let mut f = File::create("tweet_count.txt").expect("Something went wrong writing to tweet_count.txt");
        f.write_all(new_id.to_string().as_bytes()).expect("Something went wrong writing to tweet_count.txt");

        //create the feature
        let f = Feature {
            r#type: GeojsonType::Feature,
            geometry: g,
            properties: p,
            id: new_id,
        };

        f
    }
}



