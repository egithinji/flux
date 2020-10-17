use std::env;
use flux::geojson::FeatureCollection;
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
            delete_feature(feature_id).unwrap();
        },
        _ => println!("Unrecognized argument."),
    }
    
   


}

fn delete_feature(feature_id: usize) -> Result<(),String>{
    //get the feature collection from the file
    let mut fc = FeatureCollection::from_file("locations.geojson").unwrap();
    //get the vector of features from the feature collection
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
        write_feature_collection_to_file(&fc);

        //update the statistics
        let stats = Statistics::new();
        stats.updateStats();

        println!("Feature successfully removed. New size of feature collection: {}", fc.features.len());
        Ok(())
    } else {
        Err("Feature not found".to_string())
    }

}
