
pub mod geojson {

    use serde::{Deserialize, Serialize};

    //Corresponds to geojson geometry key
    #[derive(Serialize, Deserialize)]
    pub struct Geometry {
        pub r#type: GeojsonType,
        pub coordinates: [f64;2],
    }

    //Corresponds to geojson properties key
    #[derive(Serialize, Deserialize)]
    pub struct Properties {
        pub title: String,
        pub description: String,
    }

    //Corresponds to geojson feature
    #[derive(Serialize, Deserialize)]
    pub struct Feature {
        pub r#type: GeojsonType,
        pub geometry: Geometry,
        pub properties: Properties,
    }

    //Corresponds to geojson feature collection
    #[derive(Serialize, Deserialize)]
    pub struct FeatureCollection {
        pub r#type: GeojsonType,
        pub features: Vec<Feature>,
    }

    //Creates a new feature collection with no features
    impl FeatureCollection {
        pub fn new() -> FeatureCollection {
            FeatureCollection {
                r#type: GeojsonType::FeatureCollection,
                features: Vec::<Feature>::new(),
            }
        }
    }

    //Used when specifying a type as a point, feature, or feature collection
    #[derive(Serialize, Deserialize)]
    pub enum GeojsonType {
        FeatureCollection,
        Feature,
        Point
    }

    pub fn add_feature_to_collection(mut fc: FeatureCollection, feature: Feature) -> FeatureCollection{
       //given a featurecollection and a feature,
       //adds the feature to the feature collection, and returns the feature collection.

       //add the feature to the feature collection
       fc.features.push(feature);

       fc

    }

}

pub mod tweets {

    use crate::geojson::GeojsonType;
    use crate::geojson::Feature;
    use crate::geojson::Geometry;
    use crate::geojson::Properties;
    
    //To hold relevant data pertaining to a tweet
    pub struct Tweet {
        pub location: [f64;2],
        pub title: String,
        pub description: String,
    }

    pub fn convert_tweet_to_feature(tweet: Tweet) -> Feature {
       //given a tweet, creates a new feature with the tweet's data and returns it.

       //create a geometry struct (needed by 'feature' struct)
       let g = Geometry {
            r#type: GeojsonType::Point,
            coordinates: tweet.location,
       };

       //create a properties struct (needed by 'feature' struct)
       let p = Properties {
            title: tweet.title,
            description: tweet.description,
       };

       //create the feature
       let f = Feature {
            r#type: GeojsonType::Feature,
            geometry: g,
            properties: p,
       };

       f

    }

}


pub mod file_operations {
    use crate::geojson::Feature;
    use crate::geojson::FeatureCollection;
    use std::fs::File;
    use serde_json::Result;

    //write a single feature to the web server
     pub fn write_feature_to_file(f: &Feature) -> std::io::Result<()> {
        let file = File::create("location.geojson")?;
        serde_json::to_writer(file, f)?;
        Ok(())
    }

    //write a feature collection to the web server
    pub fn write_feature_collection_to_file(f: FeatureCollection) -> std::io::Result<()> {
        let file = File::create("locations.geojson")?;
        serde_json::to_writer(file, &f)?;
        Ok(())
    }   
}
