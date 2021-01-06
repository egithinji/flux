use flux::tweets::Tweet;
use flux::geojson::Feature;
use flux::geojson::FeatureCollection;
use flux::file_operations::get_polygon_from_file;
use flux::file_operations::save_location_description;
use flux::file_operations::get_hashmap_of_locations;
use flux::file_operations::write_unmatched_location;
use flux::file_operations::write_feature_collection_to_file;
use flux::file_operations::user_posted_today;
use flux::file_operations::write_user_id;
use flux::geojson::get_random_point_in_polygon;
use flux::statistics::Statistics;
use geo::Polygon;
use geo::Point;
use geo::prelude::Contains;
use flux::pattern_matching;
use egg_mode::error::Result;
use egg_mode::search::{self, ResultType};
use futures::TryStreamExt;
use egg_mode::stream::StreamMessage;
use egg_mode;
use egg_mode::user::TwitterUser;
use flux::pattern_matching::match_location_description;
use chrono::prelude::*;
use std::env;
use flux::config::Config;
use convert_case::{Case, Casing};


#[tokio::main]
async fn main() {

    let stats = Statistics::new();

    //the main feature collection containing all complaints in the month
    let mut fc = FeatureCollection::new("locations.geojson".to_string());
    //the feature collection containing only complaints for today
    let mut fc2 = FeatureCollection::new("today_locations.geojson".to_string());

    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        //Normal operation of the program. Read the feature collections from the file
        fc = FeatureCollection::from_file("locations.geojson").unwrap();
        fc2 = FeatureCollection::from_file("today_locations.geojson").unwrap();
        stats.updateStats();
    } else {
        //Read the argument
        let arg = args[1].as_str();

        match arg {
            "refresh" => {
                //Refresh the locations.geojson and today_locations.geojson files by writing blank fc to the file.
                write_feature_collection_to_file(&fc,"locations.geojson");
                write_feature_collection_to_file(&fc2,"today_locations.geojson");
                stats.updateStats();
            },
            _ => {
                panic!("Unrecognized argument");
            }

        }

    }

    
    let config = Config::new();
    let token = egg_mode::auth::Token::Access{consumer: config.con_token, access: config.acc_token};

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

                       //store the user's id in the antispam file 
                        write_user_id(&f.properties.user_id);
                        //add the feature to the both feature collections (for the month and for
                        //today)
                        fc.add_feature(f.clone());
                        fc2.add_feature(f);
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
    
    //First, strip the text of non-ascii characters.
    let text = pattern_matching::convert_to_ascii(&tweet.text);
    
    //We're not interested in retweets
    if pattern_matching::is_retweet(&text) {
        return None
    }

    //Get a hashmap of location descriptions to polygons
    let locations = get_hashmap_of_locations();

    //Attempt to find a location description in the tweet's text
    let description = match_location_description(&text, &locations);

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
            let lt_formatted = local_time.format("%R%P on %a %b %d %Y").to_string();

            //Get the id of the user who tweeted
            let mut uid = 0; 
            match tweet.user {
            
                Some(b) => {
                        uid = b.id;
                        match b.location {
                            Some(l) => {
                                println!("******\nThe location of the tweeter is {:?}\n*****************",l);
                            },
                            None => {}
                        }
                },
                None => {}
            }

            //At this point, check if this user is a spammer. If so return none and don't proceed

            if user_posted_today(uid) {
                    return None
            }

            //Then create a flux tweet with all this information
            //If the area has an underscore replace it with a space
            let a = v.replace("_"," ");
            let new_tweet = Tweet {
                location: [random_point.0, random_point.1],
                posted_on: lt_formatted,
                text: tweet.text.to_owned(),
                user_id: uid,
                area: a.to_case(Case::Title).to_lowercase(),
            };

            //Convert the tweet to a Feature
            let new_feature = new_tweet.to_feature();

            Some(new_feature)

        },
        None => {
            println!("No location matched.");
            write_unmatched_location(&text);
            None
        }
    }


}


