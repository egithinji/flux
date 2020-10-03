use serde::{Deserialize, Serialize};
use crate::file_operations::write_feature_to_file;
use crate::file_operations::write_feature_collection_to_file;
use geo::LineString;
use geo::Polygon;
use geo::MultiPolygon;
use geo::CoordinateType;
use delaunator::Point as DelPoint;
use delaunator::triangulate;
use rand::Rng;

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

pub fn get_random_point_in_polygon(poly: Polygon<f64>) -> (f64, f64)  
//First triangulates the polygon using delaunator triangulation.
//Then uses formula to generate random point in one of the triangles

{
    
    //get a reference of the exterior linestring of the polygon
    let ls = poly.exterior();

   
    //get a vector of delaunator points from the linestring
    let del_points: Vec<DelPoint> = linestring_to_delaunator(ls.clone());

    //triangulate the delaunator points
    let result = triangulate(&del_points).expect("No triangulation exists.");

    println!("Triangles are: {:?}",result.triangles);

    let multipolygon = get_polygons_from_triangles(result.triangles, ls);

    let mut points: Vec<_> = Vec::new();

    //Iterate through the multipolygon of triangles and generate a random
    //point within each
    for p in multipolygon.into_iter() {
        points.push(generate_random_point_in_triangle(p));
    }

    //select one of the random points and return it
    let mut rng = rand::thread_rng();
    let num = rng.gen_range(0,points.len());
    let random_point: (f64, f64) = points[num];

    random_point

}

fn linestring_to_delaunator(ls: LineString<f64>) -> Vec<DelPoint> 

{
    let mut del_points: Vec<DelPoint> = Vec::new();
    for point in ls.points_iter() {
        del_points.push(DelPoint{x: point.x(), y: point.y()}); 
    }

    del_points
}

fn get_polygons_from_triangles(triangles: Vec<usize>, linestring: &LineString<f64>) -> MultiPolygon<f64> {
    let mut inner_polys: Vec<Polygon<f64>> = Vec::new();
    let mut coord: Vec<_> = Vec::new();
    let mut count = 0;
    
    for index in triangles.iter() {
        if count == 3 {
            inner_polys.push(Polygon::new(LineString::from(coord.clone()), vec![],));
            count = 0;
            coord.clear();
        }
        coord.push(linestring[*index].x_y()); 
        count = count + 1;    
    }

    MultiPolygon::<f64>(inner_polys)
   
}

fn generate_random_point_in_triangle(triangle: Polygon<f64>) -> (f64, f64) {
    //Using formula from section 4.2 in the following book:
    //https://www.cs.princeton.edu/~funk/tog02.pdf
    //See also https://math.stackexchange.com/questions/18686/uniform-random-point-in-triangle
    
    let mut rng = rand::thread_rng();
    let r1: f64 = rng.gen_range(0.0, 1.0);
    let r2: f64 = rng.gen_range(0.0, 1.0);

    let ls = triangle.exterior().clone();

    //get the three vertices of the triangle
    let a = ls[0];
    let b = ls[1];
    let c = ls[2];
    
    let x = (1.0-r1.sqrt())*a.x + r1.sqrt()*(1.0-r2)*b.x + r1.sqrt()*r2*c.x;

    let y = (1.0-r1.sqrt())*a.y + r1.sqrt()*(1.0-r2)*b.y + r1.sqrt()*r2*c.y;
        
        

    (x, y)
}
