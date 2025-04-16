use std::mem;

#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
struct Point {
    x: f64,
    y: f64,
}
struct ThreedPoint{
    x: f64,
    y: f64,
    z: f64
}

// A Rectangle can be specified by where its top left and bottom right 
// corners are in space
#[allow(dead_code)]
struct Rectangle {
    top_left: Point,
    bottom_right: Point,
}

struct Cube{
    top_left_front: ThreedPoint,
  //  top_right_front: ThreedPoint,
 //   bottom_left_front: ThreedPoint,
    bottom_right_front: ThreedPoint,
    
    top_left_back: ThreedPoint,
 //   top_right_back:ThreedPoint,
 //   bottom_left_back: ThreedPoint,
    bottom_right_back: ThreedPoint,
}

fn origin() -> Point {
    Point { x: 0.0, y: 0.0 }
}
fn three_d_origin() -> ThreedPoint{
    ThreedPoint { x: 0.0, y: 0.0, z: 0.0}
}
fn boxed_origin() -> Box<Point> {
    // Allocate this point on the heap, and return a pointer to it
    Box::new(Point { x: 0.0, y: 0.0 })
}
fn boxed_three_d_origin() -> Box<ThreedPoint>{
    Box::new(ThreedPoint {x: 0.0, y: 0.0, z: 0.0})
}
fn main() {
    // (all the type annotations are superfluous)
    // Stack allocated variables
    let point: Point = origin();
    let threedpoint: ThreedPoint = three_d_origin();
    
    let rectangle: Rectangle = Rectangle {
        top_left: origin(),
        bottom_right: Point { x: 3.0, y: -4.0 }
    };
    
    let cube: Cube = Cube{
        top_left_front: three_d_origin(),
        bottom_right_front: ThreedPoint { x: 3.0, y: -4.0, z: 0.0},
        top_left_back: ThreedPoint { x: 0.0, y:0.0, z:-3.0},
        bottom_right_back: ThreedPoint { x: 3.0, y: -4.0, z: -3.0}
    };

    // Heap allocated rectangle
    let boxed_rectangle: Box<Rectangle> = Box::new(Rectangle {
        top_left: origin(),
        bottom_right: Point { x: 3.0, y: -4.0 },
    });

    let boxed_cube: Box<Cube> = Box::new(Cube{
        top_left_front: three_d_origin(),
        bottom_right_front: ThreedPoint{x:3.0, y: -4.0, z: 0.0},
        top_left_back: ThreedPoint{x:0.0, y: 0.0, z: -3.0},
        bottom_right_back: ThreedPoint{x: 3.0, y: -4.0, z: -3.0}
    });

    // The output of functions can be boxed
    let boxed_point: Box<Point> = Box::new(origin());
    let boxed_threed_point: Box<Point>  = Box::new(origin());

    // Double indirection
    let box_in_a_box: Box<Box<Point>> = Box::new(boxed_origin());
    let cube_in_a_box: Box<Box<ThreedPoint>> = Box::new(boxed_three_d_origin());

    println!("Point occupies {} bytes on the stack",
             mem::size_of_val(&point));
    println!("Rectangle occupies {} bytes on the stack",
             mem::size_of_val(&rectangle));

    println!("3d point occupies {} bytes on the stack", mem::size_of_val(&threedpoint));
    println!("Cube occupies {} bytes on the stack", mem::size_of_val(&cube));
    

    // box size == pointer size
    println!("Boxed point occupies {} bytes on the stack",
             mem::size_of_val(&boxed_point));
    println!("Boxed rectangle occupies {} bytes on the stack",
             mem::size_of_val(&boxed_rectangle));
    println!("Boxed box occupies {} bytes on the stack",
             mem::size_of_val(&box_in_a_box));

    //cube size == pointenr size
    println!("Boxed point occupies {} bytes on the stack", mem::size_of_val(&boxed_threed_point));
    println!("Boxed cube occupies {} bytes on the stack", mem::size_of_val(&boxed_cube));
    println!("Cube in a box occupies {} bytes on the stack", mem::size_of_val(&cube_in_a_box));

    // Copy the data contained in `boxed_point` into `unboxed_point`
    let unboxed_point: Point = *boxed_point;
    println!("Unboxed point occupies {} bytes on the stack",
             mem::size_of_val(&unboxed_point));

    let unboxed_point: Point = *boxed_threed_point;
    println!("Unboxed point occupies {} bytes on the stack", mem::size_of_val(&unboxed_point)); 
}