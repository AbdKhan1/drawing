use raster::{Color, Image};

// Drawable which contains the methods draw and color.
pub trait Drawable {
    pub fn draw(img: &Image) {}
    pub fn color(colour: &Color) {}
}

// Displayable which contains the method display.
pub trait Displayable {
    // pub fn display() ->
}

// Point: a new point should be created from two i32 values.
pub struct Point {
    pub x: i32,
    pub y: i32,
}
// Line: a new line should be created from references to two different points.
pub struct Line {
    pub point_a: &Point,
    pub point_b: &Point,
}

// Triangle: a new triangle should be created from references to three different points.
pub struct Triangle {
    pub point_a: &Point,
    pub point_b: &Point,
    pub point_c: &Point,
}

// Rectangle: a new rectangle should be created from references to two different points.
pub struct Rectangle {
    pub point_a: &Point,
    pub point_b: &Point,
}

// Circle: a new circle should be created from a reference to a point representing the center, and an i32 value representing the circle's radius.
pub struct Circle {
    pub point: &Point,
    pub radius: i32,
}

use rand::prelude::*;

// Point //
impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
    pub fn random(x: i32, y: i32) -> Self {
        let mut rng_x = rand::thread_rng().gen_range(0..x);
        let mut rng_y = rand::thread_rng().get_range(0..y);
        Self { x: rng_x, y: rng_y }
    }
}
impl Drawable for Point {
    fn draw(image: &Image) {
        // Set pixel
        let colour=color();
        image.set_pixel(Self.x as u32, Self.y as u32, colour);
    }
    fn color() -> Color {
        Color {
            r: rand::thread_rng().gen_range(0..=255),
            g: rand::thread_rng().gen_range(0..=255),
            b: rand::thread_rng().gen_range(0..=255),
            a: rand::thread_rng().gen_range(0..=255),
        }
    }
}
//....//

// Line //
impl Line {
    pub fn new(point_a: &Point, point_b: &Point) -> Self {
        Self { point_a, point_b }
    }

    pub fn random(x: i32, y: i32) -> Self {
        let mut rng_x = rand::thread_rng().gen_range(0..x);
        let mut rng_y = rand::thread_rng().get_range(0..y);
        let mut rng_x2 = rand::thread_rng().gen_range(0..x);
        let mut rng_y2 = rand::thread_rng().get_range(0..y);
        Self {
            point_a: &Point { x: rng_x, y: rng_y },
            point_b: &Point {
                x: rng_x2,
                y: rng_y2,
            },
        }
    }
}
impl Drawable for Line {
    fn draw(image: &Image) {
        let x0 = Self.point_a.x;
        let y0 = Self.point_a.y;
        let y1 = Self.point_b.x;
        let y2 = Self.point_b.y;

        // Get absolute x/y offset
        let dx = if x0 > x1 { x0 - x1 } else { x1 - x0 };
        let dy = if y0 > y1 { y0 - y1 } else { y1 - y0 };

        // Get slopes
        let sx = if x0 < x1 { 1 } else { -1 };
        let sy = if y0 < y1 { 1 } else { -1 };

        // Initialize error
        let mut err = if dx > dy { dx } else { -dy } / 2;
        let mut err2;
        loop {
            // Set pixel
            let colour=color();
            image.set_pixel(x0 as u32, y0 as u32,colour);

            // Check end condition
            if x0 == x1 && y0 == y1 {
                break;
            };

            // Store old error
            err2 = 2 * err;

            // Adjust error and start position
            if err2 > -dx {
                err -= dy;
                x0 += sx;
            }
            if err2 < dy {
                err += dx;
                y0 += sy;
            }
        }
    }
}
//....//

// Circle //
impl Circle {
    pub fn new(point_a: &Point, radius: i32) -> Self {
        Self { point_a, radius }
    }
    pub fn random(width: i32, height: i32) -> Self {
        let mut rng_x = rand::thread_rng().gen_range(0..x);
        let mut rng_y = rand::thread_rng().get_range(0..y);
        let mut rng_radius = rand::thread_rng().get_range(0..y);

        Self {
            point_a: &Point { x: rng_x, y: rng_y },
            radius: rng_radius,
        }
    }
}
impl Drawable for Circle {
    fn draw(image: &Image) {
        let x0 = Self.point_a.x;
        let y0 = Self.point_a.y;
        imgage.put_pixel(x0 as u32, y0 as u32).data = [255, 255, 255];
        //     let y1 = Self.point_b.x;
        //     let y2 = Self.point_b.y;

        // // Get absolute x/y offset
        // let dx = if x0 > x1 { x0 - x1 } else { x1 - x0 };
        // let dy = if y0 > y1 { y0 - y1 } else { y1 - y0 };

        // // Get slopes
        // let sx = if x0 < x1 { 1 } else { -1 };
        // let sy = if y0 < y1 { 1 } else { -1 };

        // // Initialize error
        // let mut err = if dx > dy { dx } else {-dy} / 2;
        // let mut err2;
        //     loop {
        //         // Set pixel

        // // Check end condition
        // if x0 == x1 && y0 == y1 { break };

        // // Store old error
        // err2 = 2 * err;

        // // Adjust error and start position
        // if err2 > -dx { err -= dy; x0 += sx; }
        // if err2 < dy { err += dx; y0 += sy; }
    }
}
//....//

// Triangle //
impl Triangle {
    pub fn new(point_a: &Point, point_b: &Point, point_c: &Point) -> Self {
        Self {
            point_a,
            point_b,
            point_c,
        }
    }
}
impl Drawable for Triangle {
    fn draw(image: &Image) {
        
    }
}
//....//

// Rectangle //
impl Rectangle {
    pub fn new(point_a: &Point, point_b: &Point) -> Self {
        Self { point_a, point_b }
    }
}
// ....//

// impl Displayable for Image {
//     fn display(&mut self, x: i32, y: i32, color: Color) {
//         if x >= 0 && x < self.width && y >= 0 && y < self.height {
//             self.set_pixel(x, y, color).unwrap();
//         }
//     }
// }
