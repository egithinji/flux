use std::env;
use flux::geojson::FeatureCollection;
use flux::geojson::OldFeatureCollection;
use flux::geojson::Feature;
use flux::geojson::OldFeature;
use flux::geojson::Properties;
use flux::file_operations::write_feature_collection_to_file;
use flux::statistics::Statistics;



fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        panic!("Please provide an argument");
    }

    let arg = args[1].as_str();
    match arg {
        "delete" => {
            if args.len() < 3 {
                panic!("Please provide the id of the feature to delete.");
            }
            let feature_id = args[2].parse::<usize>().unwrap();
            delete_feature_from_monthly(feature_id).unwrap();
            delete_feature_from_daily(feature_id).unwrap();
        },
        "addids" => {
            let fc = get_fc();
            generate_new_fc(fc);
            let today_fc = get_today_fc();
            generate_new_fc(today_fc);
        },
        _ => println!("Unrecognized argument."),
    }

}

fn delete_feature_from_monthly(feature_id: usize) -> Result<(),String>{
    //get the feature collection from the file
    let mut fc = FeatureCollection::from_file("locations.geojson").unwrap();

    //get vector of features from the feature collections
    let mut features = fc.features;

    let mut index = 0; //to hold the index of the feature we want to remove
    let mut found = false;


    //Iterate over the vector until we find a feature with the id matching the feature_id
    for f in features.iter() {
        if f.id == feature_id {
            found = true;
            break;
        }
        index = index +1;
    }

    //If the feature was found, remove it from the features
    if found == true {
        features.remove(index);
        fc.features = features;
        //write the adjusted feature collection to the locations.geojson file
        write_feature_collection_to_file(&fc,"locations.geojson");

        //update the statistics
        let stats = Statistics::new();
        stats.updateStats();

        println!("Feature successfully removed. New size of feature collection: {}", fc.features.len());
        Ok(())
    } else {
        Err("Feature not found".to_string())
    }

}

fn delete_feature_from_daily(feature_id: usize) -> Result<(),String>{
    //get the feature collection from the file
    let mut fc = FeatureCollection::from_file("today_locations.geojson").unwrap();

    //get vector of features from the feature collections
    let mut features = fc.features;

    let mut index = 0; //to hold the index of the feature we want to remove
    let mut found = false;


    //Iterate over the vector until we find a feature with the id matching the feature_id
    for f in features.iter() {
        if f.id == feature_id {
            found = true;
            break;
        }
        index = index +1;
    }

    //If the feature was found, remove it from the features
    if found == true {
        features.remove(index);
        fc.features = features;
        //write the adjusted feature collection to the locations.geojson file
        write_feature_collection_to_file(&fc,"today_locations.geojson");

        //update the statistics
        let stats = Statistics::new();
        stats.updateStats();

        println!("Feature successfully removed. New size of feature collection: {}", fc.features.len());
        Ok(())
    } else {
        Err("Feature not found".to_string())
    }

}


//temp function for adding user ids to features
fn get_fc() -> OldFeatureCollection {
    
    //get the feature collection from the file
    let mut fc = OldFeatureCollection::from_file("locations.geojson").unwrap();

    fc

}


//temp function
fn get_today_fc() -> OldFeatureCollection {

    //get the feature collection from the file
    let mut fc = OldFeatureCollection::from_file("today_locations.geojson").unwrap();

    fc

}

//temp function
fn generate_new_fc(oldfc: OldFeatureCollection) {
   
    
    //this is a vec of features without user ids
    let features = oldfc.features;
    
    //this is a vec of features with user ids added
    let mut newfeatures: Vec<Feature> = Vec::new();

    //loop through the vec
    for f in features {
        
        let p = Properties {
            text: f.properties.text,
            posted_on: f.properties.posted_on,
            area: f.properties.area,
            user_id: 0,
        };

        let newf = Feature {
            r#type: f.r#type,
            geometry: f.geometry,
            properties: p,
            id: f.id,
        };

        newfeatures.push(newf);
    }

    //the new feature collection should have the new features
    
    let newfc = FeatureCollection {
        r#type: oldfc.r#type,
        features: newfeatures,
        file_name: oldfc.file_name,
    };

    write_feature_collection_to_file(&newfc, &newfc.file_name);    

}
