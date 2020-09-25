use flux::tweets::Tweet;
use flux::tweets::convert_tweet_to_feature;
use flux::file_operations::write_feature_to_file;
use flux::file_operations::write_feature_collection_to_file;
use flux::geojson::Feature;
use flux::geojson::FeatureCollection;
use flux::geojson::add_feature_to_collection;

fn main() -> std::io::Result<()> {

    let tweet1 = Tweet {
        location: [36.802, -1.261],
        title: "Westlands".to_owned(),
        description: "Sarit Centre".to_owned(),
    };

    let mut fc = FeatureCollection::new();
    let new_feature = convert_tweet_to_feature(tweet1);
    write_feature_to_file(&new_feature)?;
    fc = add_feature_to_collection(fc, new_feature);
    write_feature_collection_to_file(fc);

    Ok(())

}
