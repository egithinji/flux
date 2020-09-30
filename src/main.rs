use flux::tweets::Tweet;
use flux::geojson::FeatureCollection;
use flux::file_operations::get_polygon_from_file;
use flux::geojson::triangulate_polygon;
use geo::Polygon;
use geo::Point;
use geo::prelude::Contains;

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

    //let fc = FeatureCollection::new();
    //let new_feature = tweet4.to_feature();
    //fc.add_feature(new_feature)?;
    
    let poly = get_polygon_from_file("./polygons/pangani.txt",0.0,0.0);
    
    println!("The polygon is: \n\n {:?}", poly);
    
    triangulate_polygon(poly);


    //let p1: Point<f64> = (36.83966875076294,-1.2708122539392026).into(); 
   
    //println!("\nChecking if it contains: {:?}", (36.83966875076294,-1.2708122539392026));

    //println!("\nThe answer is {}",poly.contains(&p1));

    Ok(())

}
