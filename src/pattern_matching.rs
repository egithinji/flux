use regex::Regex;
use std::collections::HashMap;
use geo::Polygon;
use geo::algorithm::area::Area;
use convert_case::{Case, Casing};

pub fn match_location_description(text: &str, locations: &HashMap<String, Polygon<f64>>) -> Option<String> {
    //Checks whether any of the keys of the location_descriptions hashmap
    //are matched in the provided text.
    //If there is more than one match, return the one with the smaller area.
    //For example 'buru buru' and 'buru buru phase 1' might both be matched in a tweet
    //but 'buru buru phase 1' should be returned.

    //let mut descriptions: Vec<&str> = Vec::new();

    let mut smallest = None;
    let mut loc = None;

    for (description, polygon) in locations {
        //Match the description when it appears in the text of the tweet
        //preceded or followed by the provided punctuation or when it is at 
        //the beginning of the string.
    
        let desc = "(^|[ ,.?])".to_owned()+&description.to_case(Case::Lower)+"($|[ ,.!?])";
        let re = Regex::new(&desc).unwrap();
        if re.is_match(&text.to_case(Case::Lower)) {
            
            println!("Found a match!");
            
            match smallest {
                Some(v) => {
                    if polygon.signed_area() < v {
                        println!("Area of polygon is: {}",polygon.signed_area());
                        smallest = Some(polygon.signed_area());
                        loc = Some(description.to_owned());
                    }
                },
                None => {
                        println!("Area of polygon is: {}",polygon.signed_area());
                        smallest = Some(polygon.signed_area());
                        loc = Some(description.to_owned());
                }
            }
            
            //descriptions.push(description);
        }
        
    }

    //if descriptions.len() == 0 {
      //  None
    //} else if descriptions.len() == 1 {
      //  Some(descriptions[0].to_owned())
    //}else {
        //return the longest description
       // Some(longest(descriptions))
    //}

    loc

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
