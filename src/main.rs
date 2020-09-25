use flux::tweets::Tweet;
use flux::geojson::FeatureCollection;

fn main() -> std::io::Result<()> {

    let tweet1 = Tweet {
        location: [36.802, -1.261],
        title: "Westlands".to_owned(),
        description: "Sarit Centre".to_owned(),
    };

    let tweet2 = Tweet {
        location: [36.822, -1.289],
        title: "CBD".to_owned(),
        description: "KICC Building".to_owned(),
    };

    let tweet3 = Tweet {
        location: [36.753, -1.289],
        title: "Kawangware".to_owned(),
        description: "Congo".to_owned(),
    };

    let tweet4 = Tweet {
        location: [36.796, -1.292],
        title: "Hurlingham".to_owned(),
        description: "1414 Rose Avenue".to_owned(),
    };

    let fc = FeatureCollection::new();
    let new_feature = tweet3.to_feature();
    fc.add_feature(new_feature)?;
    Ok(())

}
