    use crate::geojson::Feature;
    use crate::geojson::FeatureCollection;
    use std::fs::File;
    use serde_json::Result;

    //write a single feature to the web server
     pub fn write_feature_to_file(f: &Feature) -> std::io::Result<()> {
        let file = File::create("new_location.geojson")?;
        serde_json::to_writer(file, f)?;
        Ok(())
    }

    //write a feature collection to the web server
    pub fn write_feature_collection_to_file(f: FeatureCollection) -> std::io::Result<()> {
        let file = File::create("locations.geojson")?;
        serde_json::to_writer(file, &f)?;
        Ok(())
    } 
