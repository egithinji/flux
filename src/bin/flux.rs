use flux::tweets::Tweet;
use flux::geojson::Feature;
use flux::geojson::FeatureCollection;
use flux::file_operations::get_polygon_from_file;
use flux::file_operations::save_location_description;
use flux::file_operations::get_hashmap_of_locations;
use flux::file_operations::write_unmatched_location;
use flux::file_operations::write_feature_collection_to_file;
use flux::geojson::get_random_point_in_polygon;
use geo::Polygon;
use geo::Point;
use geo::prelude::Contains;
use flux::pattern_matching;
use egg_mode::error::Result;
use egg_mode::search::{self, ResultType};
use futures::TryStreamExt;
use egg_mode::stream::StreamMessage;
use egg_mode;
use flux::pattern_matching::match_location_description;
use chrono::prelude::*;
use std::env;

const CONSUMER_SECRET: &str = "KPe3VuA3sddUEcE4BzsWGOG5s7B2VoerkN7CjJP3drHNqfAuyV";
const CONSUMER_KEY: &str = "RLWbvzksHLHAZYDBhVc26SAEm";
const ACCESS_TOKEN_SECRET: &str = "5xbJrYXKer89YvsuHIWzJzSpW45wlz8JwSL9X5uuTuWxW";
const ACCESS_TOKEN: &str = "1253810304-jdIZtGSSvT6ngkUT4zIGo3Lfuvcad6l8wd387In";

#[tokio::main]
async fn main() {

//Create a new feature collection and write it to the web server
//let mut fc = FeatureCollection::new();

let mut fc = FeatureCollection::new();

let args: Vec<String> = env::args().collect();

if args.len() == 1 {
    //Normal operation of the program. Read the feature collection from the file
    fc = FeatureCollection::from_file("locations.geojson").unwrap();
} else {
    //Read the argument
    let arg = args[1].as_str();

    match arg {
        "refresh" => {
            //Refresh the locations.geojson file by writing blank fc to the file.
            write_feature_collection_to_file(&fc);
        },
        _ => {
            panic!("Unrecognized argument");
        }

    }

}

    
let con_token = egg_mode::KeyPair::new(CONSUMER_KEY, CONSUMER_SECRET);
let acc_token = egg_mode::KeyPair::new(ACCESS_TOKEN, ACCESS_TOKEN_SECRET);

    let token = egg_mode::auth::Token::Access{consumer: con_token, access: acc_token};
    println!("Live streaming tweets...");

    println!("Ctrl-C to quit\n");

    let stream = egg_mode::stream::filter()
        .track(&["KenyaPower_Care"])
        .start(&token)
        .try_for_each(|m| {
            if let StreamMessage::Tweet(tweet) = m {
                println!("\n-----------------------------------");
                println!("Tweeted at: {}\n{}",tweet.created_at,tweet.text);
                println!("-----------------------------------");
                match process_tweet(tweet) {
                    Some(f) => {
                        fc.add_feature(f);
                    },
                    None => {}
                }
            } else {
                println!("{:?}",m);
            }
            futures::future::ok(())
        });
    if let Err(e) = stream.await {
        println!("Stream error: {}", e);
        println!("Disconnected")

    }

}

fn process_tweet(tweet: egg_mode::tweet::Tweet) -> Option<Feature> {
    //Please note the tweet here is an egg_mode tweet not the same as a flux tweet.
    //We want to check if the text of the tweet contains any of the location descriptions we have on
    //file.
    
    //First of all, we're not interested in retweets
    if pattern_matching::is_retweet(&tweet.text) {
        return None
    }
   
    //Get a hashmap of location descriptions to polygons
    let locations = get_hashmap_of_locations();

    //Attempt to find a location description in the tweet's text
    let description = match_location_description(&tweet.text, &locations);

    match description {
        Some(v) => {
            //If a location_description is matched, get a random point in the corresponding polygon
            println!("Matched {}!",v);

            //Get the polygon corresponding to the location
            let poly = locations.get(&v).unwrap().to_owned();

            //Then get the random point
            let random_point = get_random_point_in_polygon(poly);

            //Convert the created_at to local time
            let local_time: DateTime<Local> = DateTime::from(tweet.created_at);
            let lt_formatted = local_time.format("%R%P on %a %b %e").to_string();
            //Then create a flux tweet with all this information
            let new_tweet = Tweet {
                location: [random_point.0, random_point.1],
                posted_on: lt_formatted,
                text: tweet.text.to_owned(),
                area: v,
            };

            //Convert the tweet to a Feature
            let new_feature = new_tweet.to_feature();

            Some(new_feature)

        },
        None => {
            println!("No location matched.");
            write_unmatched_location(&tweet.text);
            None
        }
    }


}


