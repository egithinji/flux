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

    impl Tweet {
        //return a Feature that corresponds to this tweet
        pub fn to_feature(self) -> Feature {
            //create a geometry struct (needed by 'feature' struct)
            let g = Geometry {
                r#type: GeojsonType::Point,
                coordinates: self.location,
            };

            //create a properties struct (needed by 'feature' struct)
            let p = Properties {
                title: self.title,
                description: self.description,
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

