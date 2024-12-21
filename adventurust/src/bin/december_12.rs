use core::num;
use std::{array, collections::{HashMap, HashSet}, fs, iter::zip};
use clap::Parser;
use log::{self, debug, info};
use itertools::enumerate;

/*
Thinking through the problem:
- For each spot, grow the plot around it, keeping a list of all the points that are the same and are reachable
- Once all areas are grown, and each square has a list of the points that are reachable then:
    - area = size of plot
    - perimeter = Sum(4-#connections) for each point


*/

#[derive(Parser, Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn add(&self, other: &Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
    fn distance(&self, other: &Point) -> i32 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }
}

#[derive(Clone)]
struct GardenPlot {
    location: Point,
    value: char,
    connected_plots: Vec<Point>,
}

impl GardenPlot {
    fn point_in_connected(&self, point: &Point) -> bool {
        for plot in &self.connected_plots {
            if plot == point {
                return true;
            }
        }
        false
    }
    fn str(&self) -> String{
        format!("Value: {}, Location: ({},{}), Connected: {}", self.value, self.location.x, self.location.y, self.connected_plots.len())
    }
}

struct GardenArea {
    plots: Vec<GardenPlot>,
}

#[derive(Parser, Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum DIRECTION {
    NORTH,
    SOUTH,
    EAST,
    WEST,
}

const MOVEMENT: [Point; 4]= [Point { x: 0, y: 1 }, 
Point { x: 0, y: -1 }, 
Point { x: 1, y: 0 },
Point { x: -1, y: 0 }] ;

impl GardenArea {
    fn point_in_area(&self, point: &Point) -> bool {
        for plot in &self.plots {
            if plot.location == *point {
                return true;
            }
        }
        false
    }

    fn area(&self) -> i32 {
        self.plots.len() as i32
    }
    fn perimeter(&self) -> i32 {
        let mut perimeter = 0;
        for plot in &self.plots {
            perimeter += 4 - plot.connected_plots.len() as i32;
        }
        perimeter
    }
    fn score(&self) -> i32 {
        self.area() * self.perimeter()
    }

    fn str(&self) -> String{
        format!("Char: {}, Area: {}, Perimeter: {}", self.plots[0].value, self.area(), self.perimeter())
    }

    fn num_sides(&self) -> i32 {
        let mut sides = HashMap::new();
        sides.insert(DIRECTION::SOUTH, Vec::new());
        sides.insert(DIRECTION::NORTH, Vec::new());
        sides.insert(DIRECTION::EAST, Vec::new());
        sides.insert(DIRECTION::WEST, Vec::new());
        for plot in &self.plots {
            for movement in MOVEMENT {
                match movement {
                    Point{x: 0, y: 1} => {
                        if ! plot.point_in_connected(&plot.location.add(&movement)){
                            sides.get_mut(&DIRECTION::SOUTH).unwrap().push(plot.location.add(&movement).clone());
                        }
                    },
                    Point{x: 0, y: -1} => {
                        if ! plot.point_in_connected(&plot.location.add(&movement)){
                            sides.get_mut(&DIRECTION::NORTH).unwrap().push(plot.location.add(&movement).clone());
                        }
                    },
                    Point{x: 1, y: 0} => {
                        if ! plot.point_in_connected(&plot.location.add(&movement)){
                            sides.get_mut(&DIRECTION::WEST).unwrap().push(plot.location.add(&movement).clone());
                        }
                    },
                    Point{x: -1, y: 0} => {
                        if ! plot.point_in_connected(&plot.location.add(&movement)){
                            sides.get_mut(&DIRECTION::EAST).unwrap().push(plot.location.add(&movement).clone());
                        }
                    },
                    _ => {
                        panic!("Invalid movement");
                    }
                }
            }
        }
        let mut num_sides = 0;
        for side in sides.keys() {
            let mut points_sorted = sides.get(side).unwrap().clone();
            if *side == DIRECTION::NORTH || *side == DIRECTION::SOUTH {
                points_sorted.sort_by(|a, b| if a.y == b.y {
                    a.x.cmp(&b.x)
                } else {
                    a.y.cmp(&b.y)
                });
            }
            if *side == DIRECTION::EAST || *side == DIRECTION::WEST {
                points_sorted.sort_by(|a, b| if a.x == b.x {
                    a.y.cmp(&b.y)
                } else {
                    a.x.cmp(&b.x)
                });
                
            }

            // num side is the number of points that are greater than 1 away from the previous point
            if points_sorted.len() == 0{
                continue;
            }
            if points_sorted.len() == 1{
                num_sides += 1;
                continue;
            }
            let mut new_side = true;
            // TODO: It's over or under counting....
            for (first , second )in zip(points_sorted.iter().take(points_sorted.len() - 1), points_sorted.iter().skip(1)) {
                 if new_side {  
                    num_sides += 1;
                    new_side = false;
                } 
                if first.distance(&second) > 1 {
                    new_side = true;
                    // if last iteration
                    if second == points_sorted.last().unwrap() {
                        num_sides += 1;
                    }
                }
            }
            debug!("Value: {}, Direction: {:?},  Num sides: {}", self.str(), side ,  num_sides);

            
        }

        num_sides
    }

    fn value(&self) -> char {
        self.plots[0].value
    }

    fn score_2(&self) -> i32 {
        self.area() * self.num_sides()
    }
}


#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    file_path: String,
    target_number: Option<i64>,
}


fn get_reachable_points(current_point: &Point, file_map: &Vec<Vec<char>>) -> Vec<Point> {
    let mut reachable_points = Vec::new();
    let current_value = file_map[current_point.y as usize][current_point.x as usize];
    for movement in MOVEMENT{
        let new_point = current_point.add(&movement);
        if new_point.x < 0 || new_point.y < 0 || new_point.x >= file_map[0].len() as i32 || new_point.y >= file_map.len() as i32 {
            continue;
        }
        if let Some(line) = file_map.get(new_point.y as usize) {
            if let Some(new_value) = line.get(new_point.x as usize) {
                if *new_value == current_value {
                    reachable_points.push(new_point);
                }
            }
        }
    }
    reachable_points
}

fn grow_garden(plot: &GardenPlot, file_map: &Vec<Vec<char>>, visited_points: &mut Vec<GardenPlot>)  {
    // For each reachable point, check if it's the same value
    // If so add it to the garden plots reachable spots, visited_points, and grow_garden
    for reachable_point in &plot.connected_plots {
        if visited_points.iter().any(|p| p.location == *reachable_point) {
            continue;
        }
        let new_plot = GardenPlot {
            location: *reachable_point,
            value: file_map[reachable_point.y as usize][reachable_point.x as usize],
            connected_plots: get_reachable_points(reachable_point, file_map),
        };
        visited_points.push(new_plot.clone());
        grow_garden(&new_plot, file_map, visited_points);
    }
    
}

fn build_gardens(file_map: &Vec<Vec<char>>) -> Vec<GardenArea> {
    let mut areas: Vec<GardenArea> = Vec::new();
    for (y, line) in file_map.iter().enumerate() {
        for (x, value) in line.iter().enumerate() {
            let point = Point { x: x as i32, y: y as i32 };
            let mut skip = false;
            for area in areas.iter() {
                if area.point_in_area(&point) {
                    skip = true;
                    break;
                }
            }
            if skip {
                continue;
            }
            let plot = GardenPlot {
                location: point,
                value: *value,
                connected_plots: get_reachable_points(&point, file_map),
            };
            let mut garden_plots = Vec::new();
            garden_plots.push(plot.clone());
            grow_garden(&plot, file_map, &mut garden_plots);
            let new_area = GardenArea {
                plots: garden_plots.clone(),
            };
            areas.push(new_area);
        }
    }
    areas
}

fn get_garden_areas(file_path: String) -> Vec<GardenArea> {
   let lines = fs::read_to_string(file_path).expect("Failed to read file");
   let mut file_map = Vec::new();
   for line in lines.lines() {
       let mut line_vec = Vec::new();
       for c in line.chars() {
           line_vec.push(c);
       }
       file_map.push(line_vec);
   }

   build_gardens(&file_map)

}


fn main(){
    env_logger::init();

    let args = Cli::parse();
    let file_path = args.file_path;
    let file_map = get_garden_areas(file_path);

    let mut sum = 0;
    for area in file_map.iter() {
        debug!("Area: {}", area.str());
        sum += area.score();
    }
    info!("Answer 1: {}", sum);

    let mut sum = 0;
    for area in file_map.iter() {
        debug!("Area: {}, sides: {}", area.str(), area.num_sides());
        sum += area.score_2();
    }
    info!("Answer 2: {}", sum);
    
}