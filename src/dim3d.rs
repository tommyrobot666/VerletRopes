use std::ops;
use crate::verletcore::{Line};

// Clone and Copy are same.
// Copy is done automatically, Clone needs you to use .clone()
// Copy is "bitwise", Clone can use any computations
#[derive(Clone, Copy)]
pub struct Vector3 {
    pub x:f32,
    pub y:f32,
    pub z:f32
}

impl Vector3 {
    /**
    Length squared. "f means fast"
    **/
    pub fn length_f(&self) -> f32 {
        self.x*self.x+self.y*self.y+self.z*self.z
    }

    pub fn length(&self) -> f32 {
        self.length_f().sqrt()
    }


}

impl ops::Add<Vector3> for Vector3 {
    type Output = Vector3;

    fn add(self, rhs: Vector3) -> Vector3 {
        Vector3{x:self.x+rhs.x, y:self.y+rhs.y, z:self.z+rhs.z}
    }
}

impl ops::Add<f32> for Vector3 {
    type Output = Vector3;

    fn add(self, rhs: f32) -> Vector3 {
        Vector3{x:self.x+rhs, y:self.y+rhs, z:self.z+rhs}
    }
}

impl ops::Sub<Vector3> for Vector3 {
    type Output = Vector3;

    fn sub(self, rhs: Self) -> Self::Output {
        Vector3{x:self.x-rhs.x, y:self.y-rhs.y, z:self.z-rhs.z}
    }
}

impl ops::Mul<f32> for Vector3 {
    type Output = Vector3;

    fn mul(self, rhs: f32) -> Self::Output {
        Vector3{x:self.x*rhs, y:self.y*rhs, z:self.z*rhs}
    }
}


pub struct Point {
    pub pos:Vector3,
    pub prev_pos:Vector3,
    pub locked:bool,
}

impl Point {
    pub fn new(x:f32, y:f32, z:f32, locked:bool) -> Point {
        Point {
            pos:Vector3{x,y,z},prev_pos:Vector3{x,y,z},locked
        }
    }
}


pub struct AABB {
    pub pos:Vector3,
    pub size:Vector3
}

impl AABB {
    pub fn in_box(&self, point: &Point) -> bool {
        point.pos.x >= self.pos.x && point.pos.x <= self.pos.x+self.size.x
            && point.pos.y >= self.pos.y && point.pos.y <= self.pos.x+self.size.y
            && point.pos.z >= self.pos.z && point.pos.z <= self.pos.z+self.size.z
    }
}




/**
This function also applies velocity
**/
pub fn simple_force_to_points(points:&mut Vec<Point>, force:Vector3, delta:f32) {
    for i in 0..points.len(){
        let point = &mut points[i];
        if point.locked{continue}

        let prev_pos: Vector3 = point.pos.clone();
        point.pos = point.pos + ((point.pos - point.prev_pos) + force * delta * delta);
        point.prev_pos = prev_pos;
    }
}

// changes stop herre
pub fn simple_sim_step(lines:&mut Vec<Line>,points:&mut Vec<Point>){
    for i in 0..lines.len() {
        let line:&mut Line = &mut lines[i];
        let (a,b) = line.get_both_points(points);
        let center:[f32;2] = [(a.x+b.x)/2.0,(a.y+b.y)/2.0];
        let mut dir:[f32;2] = [a.x-b.x,a.y-b.y];
        // normalize dir
        let dir_len:f32 = (dir[0]*dir[0]+dir[1]*dir[1]).sqrt(); dir[0] = dir[0]/dir_len; dir[1] = dir[1]/dir_len;

        if !a.locked{
            a.x = center[0] + (dir[0] * line.length) / 2.0;
            a.y = center[1] + (dir[1] * line.length) / 2.0;
        }
        if !b.locked{
            b.x = center[0] - (dir[0] * line.length) / 2.0;
            b.y = center[1] - (dir[1] * line.length) / 2.0;
        }
    }
}


pub fn aabb_collision(aabbs:&Vec<AABB>,points:&mut Vec<Point>){
    for aabb in aabbs{
        for point in &mut *points{
            if aabb.in_box(&point){
                let top_dist = (point.y-aabb.y).abs();
                let left_dist = (point.x-aabb.x).abs();
                let bottom_dist = (aabb.y+aabb.height-point.y).abs();
                let right_dist = (aabb.x+aabb.width-point.x).abs();

                if top_dist<left_dist && top_dist<bottom_dist && top_dist<right_dist {
                    point.y = aabb.y
                } else if left_dist<bottom_dist && left_dist<right_dist {
                    point.x = aabb.x
                } else if bottom_dist<right_dist {
                    point.y = aabb.y+aabb.height
                } else {
                    point.x = aabb.x+aabb.width
                }
            }
        }
    }
}


pub fn create_rope(start:[f32;2], length:f32, lines:usize, pin_first:bool) -> (Vec<Point>, Vec<Line>) {
    let mut next_pos:[f32;2] = [start[0]+length,start[1]];
    let mut points:Vec<Point> = Vec::with_capacity(lines + 1);
    points.push(Point::new(start[0], start[1], pin_first));
    for _ in 0..lines {
        points.push(Point::new(next_pos[0], next_pos[1], false));
        next_pos[0] += length;
    }

    let mut rope:Vec<Line> = Vec::with_capacity(lines);
    for i in 0..lines {
        rope.push(Line{a: i, b: i + 1, length });
    }

    (points, rope)
}

pub fn offset_line_points(lines: &mut Vec<Line>, offset:usize){
    for line in lines.iter_mut() {
        line.a = line.a + offset;
        line.b = line.b + offset;
    }
}