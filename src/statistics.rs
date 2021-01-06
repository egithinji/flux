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
use chrono::{DateTime,NaiveDate};
use chrono::format::ParseError;

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

#[derive(Debug)]
pub struct AreaStat {
        //Corresponds to one line in the csv file used to generate the barchart race
        date: String, //the date in YYYY-MM-DD format
        name: String, //the area
        category: String, //same as the area so that each area has a different color
        value: usize, //cumulative number of complaints that day

}

impl AreaStat {
        

        pub fn print_me (self) {
                //prints itself to the area_stats.csv file

                let mut file = OpenOptions::new()
                    .append(true)
                    .open("area_stats.csv")
                    .unwrap();

                //please note I'm intentionally not printing the category
                if let Err(e) = writeln!(file, "{},{},{}",self.date, self.name, self.value) {
                        eprintln!("Couldn't write area_stat to file: {}", e);
                }
                
                
        }
}

pub fn print_area_stat_column_headers() {
       //prints the column headers for the area stats (date, name, category, value) in the
       //area_stats.txt file

        let mut file = OpenOptions::new()
            .write(true)
            .open("area_stats.csv")
            .unwrap();

        //please note I'm intentionally not printing the category
        if let Err(e) = writeln!(file, "date,name,value") {
                eprintln!("Couldn't write the column headers in areastat file: {}", e);

        }


}


pub fn generate_area_stats(stats: HashMap<String,usize>, up_to_date: String, year: &str, month: &str) -> Vec<AreaStat>{
    //Given a hashmap containing <area:cumulative_complaints> values,
    //and a date as a string, return a vector of AreaStat objects representing
    //the data for each area up to that date.
   
    //This is the vector that will hold the areastats:
    let mut areaStats = Vec::new();


    //Collect the hashmap into a vector
    let mut v = stats.iter().collect::<Vec<_>>();

    for entry in v {

            let area_name = entry.0.to_owned();

            let mut d = String::new();
            d.push_str(year);
            d.push_str("-");
            d.push_str(month);
            d.push_str("-");
            if up_to_date.len() < 2 {
                    d.push_str("0");
                    d.push_str(&up_to_date);
            } else {
                    d.push_str(&up_to_date);
            }


            let a = AreaStat {
                    date: d,
                    name: area_name.clone(),
                    category: area_name.clone(),
                    value: entry.1.to_owned(),
            };

            areaStats.push(a);

    }

    areaStats

}

pub fn get_stats_up_to_date(features: &Vec<Feature>, up_to_day: usize) -> HashMap<String,usize>{
        //Given a vector of features (from the feature collection) and an end day (e.g. 01, 02 up to 31),
        //returns hashmap of area complaints up to that day with values
        //<area,count>
        
        //Create a hashmap of <area,number of occurunces>
        let mut areas = HashMap::new();

        for feature in features.iter() {
            let area_name = feature.properties.area.to_owned();
            let counter = areas.entry(area_name).or_insert(0);
            *counter += 1;
            //get the day of the month in the current feature
            let current_day = get_day_from_feature(feature);
            //if it's greater than the up_to_day, exit the loop
            if current_day > up_to_day {
                    break;
            }
        }

        areas

}

fn get_day_from_feature(feature: &Feature) -> usize {
        //Given a feature, returns the day of the month

        let text = &feature.properties.posted_on;
        //Date is in the format: '15:28pm on Wed Nov 25 2020'

        let vec: Vec<&str> = text.split(" ").collect();

        let day = vec[4].parse::<usize>().unwrap();

        day

}


pub fn get_vec_of_features() -> Vec<Feature> {
        //Returns a vector of features from the feature collection.

        //get the feature collection from the file
        let fc = FeatureCollection::from_file("locations.geojson").unwrap();
        //get the vector of features from the feature collection
        let the_features = fc.features;

       the_features 
}

fn get_formatted_date_from_string(text: String) -> String {
    //Given a date in the format: '15:28pm on Wed Nov 25 2020' returns 2020-11-25
        
    println!("Received {}",text);

    let vec: Vec<&str> = text.split(" ").collect();

    let mut new_string = String::new();

    new_string.push_str(vec[3]); //i.e. Nov in the example
    new_string.push_str(vec[4]); //i.e. 25 in the example
    new_string.push_str(vec[5]); //i.e. 2020 in the example

    println!("Parsing: ");
    println!("{}",new_string);

    
    let value = NaiveDate::parse_from_str(&new_string,"%b%d%Y");
    let mut formatted_date = String::new();
    match value {
            Ok(v) => {
                    formatted_date = v.to_string();
            },
            Err(e) => {
                    panic!("Couldn't parse the date.");
            }
    }

    formatted_date
}
