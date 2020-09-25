    use serde::{Deserialize, Serialize};
    use crate::file_operations::write_feature_to_file;
    use crate::file_operations::write_feature_collection_to_file;

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

        pub fn add_feature(mut self, feature: Feature) -> std::io::Result<()> {
           //add a feature to this feature collection
           //and update the locations.geojson and new_location.geojson files on the webserver.
           
           //update the new_location.geojson file
           write_feature_to_file(&feature)?;

           //add the feature to this feature collection
           self.features.push(feature);

           //update the locations.geojson file
           write_feature_collection_to_file(self)?;
        
           Ok(())
        }
    }

    //Used when specifying a type as a point, feature, or feature collection
    #[derive(Serialize, Deserialize)]
    pub enum GeojsonType {
        FeatureCollection,
        Feature,
        Point
    }

    pub fn add_feature_to_collection(mut fc: FeatureCollection, feature: Feature) -> std::io::Result<()> {
       //given a featurecollection and a feature,
       //adds the feature to the feature collection, and updates the locations.geojson and
       //new_location.geojson files on the webserver.

       //update the new_location.geojson file
       write_feature_to_file(&feature)?;

       //add the feature to the feature collection
       fc.features.push(feature);

       //update the locations.geojson file
       write_feature_collection_to_file(fc)?;
       
       Ok(())
    }

