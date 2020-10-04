use regex::Regex;
use std::collections::HashMap;
use geo::Polygon;

pub fn match_location_description(text: &str, locations: &HashMap<String, Polygon<f64>>) -> Option<String> {
    //Checks whether any of the keys of the location_descriptions hashmap
    //are matched in the provided text.
    //Adds all the matches to a vector.
    //If there is more than one match, select the one with the most words
    //and return it.
    //The idea is that the match with more words is the more specific one.
    //For example 'buru buru' and 'buru buru phase 1' will both be in the vector
    //but 'buru buru phase 1' should be returned.

    let mut descriptions: Vec<&str> = Vec::new();

    for (description, polygon) in locations {
        
        let re = Regex::new(&description.to_lowercase()).unwrap();
        if re.is_match(&text.to_lowercase()) {
            println!("Found a match!");
            descriptions.push(description);
        }
        
    }

    if descriptions.len() == 0 {
        None
    } else if descriptions.len() == 1 {
        Some(descriptions[0].to_owned())
    }else {
        //return the longest description
        Some(longest(descriptions))
    }

}

fn longest(list: Vec<&str>) -> String {
    let mut longest = "";
    for d in list {
       if d.len() > longest.len() {
            longest = d;
       }
    }

    longest.to_owned()
}

pub fn is_retweet(text: &str) -> bool {
    //Checks if the tweet is a retweet (beginning with 'RT')
    let re = Regex::new(r"^RT").unwrap();
    re.is_match(text)
}
