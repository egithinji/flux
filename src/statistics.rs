use crate::geojson::FeatureCollection;
use crate::geojson::Feature;
use crate::file_operations::write_feature_collection_to_file;
use std::collections::HashMap;
use chrono::prelude::*;
use serde::{Serialize};
use serde_json::Result;
use std::fs::OpenOptions;
use std::fs::File;
use std::io::Write;
use convert_case::{Case, Casing};

#[derive(Serialize)]
pub struct Statistics {
   //Struct to hold statistics of interest
   //The various statistics are obtained by processing the
   //feature collection in locations.geojson.

    pub top_today: String,
    pub top_3_this_month: Vec<String>,
    pub top_10_this_month: Vec<String>,
    pub total_complaints_today: u32,
    pub total_complaints_this_month: u32,
    
    #[serde(skip_serializing)]
    features: Vec<Feature>,


}

impl Statistics {

    pub fn new() -> Statistics {
        
        Statistics {
            top_today: String::from("Test"),
            top_3_this_month: Vec::new(),
            top_10_this_month: Vec::new(),
            total_complaints_today: 0,
            total_complaints_this_month: 0,
            features: Vec::new(),
        }

    }

    //Called whenever there is a need to update the statistics.
    //Updates the various statistics in the Statistic struct.
    //I'm chaining a bunch of private methods to accomplish this.
    pub fn updateStats(mut self) {

        self.get_features()
        .updateTopToday()
        .updateTop3Month()
        .updateTop10Month()
        .updateTotalToday()
        .updateTotalMonth();

    }

    fn get_features(mut self) -> Self {
        
        //get the feature collection from the file
        let fc = FeatureCollection::from_file("locations.geojson").unwrap();
        //get the vector of features from the feature collection
        let the_features = fc.features;

        self.features = the_features;

        self

    }


    fn updateTopToday(mut self) -> Self {
        //Iterate through the features.
        //For each feature that was posted today, add the area to 
        //a hashmap where the keys are area names and the value
        //is the number of times the area has been found.
        
        //Using the method here: https://doc.rust-lang.org/std/collections/struct.HashMap.html#examples-20

        let mut areas = HashMap::new();
        
        for feature in self.features.iter() {
            if is_today(&feature.properties.posted_on.trim()) {
                let counter = areas.entry(&feature.properties.area).or_insert(0);
                *counter += 1;
            }
        }

        let mut top_today = String::new();
        
        //Get the area with the highest occurunces.
        //See https://doc.rust-lang.org/std/iter/trait.Iterator.html
        match areas.iter().max_by(|x,y| x.1.cmp(y.1)) {
            Some(v) => {
                top_today = v.0.to_string();        
            },
            None => {
                    
            }
        }

        println!("The top area today is {}", top_today);

        self.top_today = top_today.to_case(Case::Title);

        self
    }

    fn updateTop3Month(mut self) -> Self {
        //The top 3 areas this month
        //Use a similar method to top_today.
        //Collect into a vector, sort, and take the top 3 values.

        //Create a hashmap of <area,number of occurunces>
        let mut areas = HashMap::new();
        
        for feature in self.features.iter() {
            let counter = areas.entry(&feature.properties.area).or_insert(0);
            *counter += 1;
        }

        //Collect the hashmap into a vector
        let mut v = areas.iter().collect::<Vec<_>>();
        
        //Sort the vector by occurunces
        v.sort_by(|a,b| b.1.cmp(&a.1));
        
        //Get the first three values
        let mut top_3 = Vec::new();
        let mut count = 0;
        for entry in v {
            if count == 3 {
                break;
            }
           //TODO: This compiles but can't be the correct way of doing this. 
            top_3.push(entry.0.to_owned().to_owned().to_case(Case::Title));
            count = count + 1;
        }

        println!("The top 3 areas this month are:");
        
        for value in top_3.clone() {
            println!("{}",value);
        }

        self.top_3_this_month = top_3;
        
        self 
    }

    fn updateTop10Month(mut self) -> Self {
        //update the top10thismonth vector
        
        //Create a hashmap of <area,number of occurunces>
        let mut areas: HashMap<&str, u32> = HashMap::new();
        
        for feature in self.features.iter() {
            let counter = areas.entry(&feature.properties.area).or_insert(0);
            *counter += 1;
        }

        //Collect the hashmap into a vector
        let mut v = areas.iter().collect::<Vec<_>>();
        
        //Sort the vector by occurunces
        v.sort_by(|a,b| b.1.cmp(&a.1));
        
        //Get the first three values
        let mut top_10 = Vec::new();
        let mut count: u32 = 0;
        for entry in v {
            if count == 10 {
                break;
            }
           //TODO: This compiles but can't be the correct way of doing this. 
            top_10.push(entry.0.to_owned().to_owned().to_case(Case::Title) + " (" + &entry.1.to_owned().to_string() + ")");
            count = count + 1;
        }

        println!("The top 10 areas this month are:");
        
        for value in top_10.clone() {
            println!("{:?}",value);
        }

        self.top_10_this_month = top_10;
        
        self 



    }


    fn updateTotalToday(mut self) -> Self {
        //Total number of complaints today
        
        let mut total = 0;

        for feature in self.features.iter() {
            if is_today(&feature.properties.posted_on.trim()) {
                total += 1;
            }
        }


        println!("Total complaints today: {}", &total);

        self.total_complaints_today = total;

        
        self        
    }

    fn updateTotalMonth(mut self) {
        //Total number of complaints this month
        let mut total = 0;

        for feature in self.features.iter() {
                total += 1;
        }


        println!("Total complaints this month: {}", &total);

        self.total_complaints_this_month = total;

        self.publish();
    }

    fn publish(self) {
        //Print the stats to file in json format
        //from where it will be retrieved by the webpage via ajax.

        let s = serde_json::to_string_pretty(&self);

        match s {
            Ok((v)) => {
                println!("The stats look like this:\n {}", v);
                //write the stats to the stats.txt file
                let mut file = File::create("stats.txt").expect("Something went wrong opening the stats file for writing.");
                file.write_all(v.as_bytes()).expect("Something went wrong writing the stats.");
            },
            Err(_) => {
                println!("Something went wrong serializing stats.");
            }
        }
    }

}

fn is_today(date: &str) -> bool {
    //The sting is in the format '22:25pm on Mon Oct 12 2020'
    //Get the last four words.

    let mut words: Vec<&str> = date.split_whitespace().collect();

    let expression = words[3].to_string() + " " + words[4] + " " + words[5];

    //Format today's date into the above expression's format
    let today = Local::today().format("%b %d %Y").to_string();

    if today == expression.to_string() {
        true
    } else {
        false
    }

}
