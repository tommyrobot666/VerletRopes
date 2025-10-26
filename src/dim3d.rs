use std::ops;
use crate::verletcore::{Line};

// You can do this to make a thing seperate
// But then you have to use thing.0.property instead of thing.property (very annoying)
// if you "impl Deref for" and "impl DerefMut for" self.0, it will go back to thing.property
/*pub struct Line(crate::verletcore::Line);

impl ops::Deref for Line {
    type Target = crate::verletcore::Line;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}*/
// this code was removed because only one struct named "Line" can exist per crate


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


impl Line {
    // i want to call this "get_both_points"
    pub fn get_points<'a>(&self, points: &'a mut [Point]) -> (&'a mut Point, &'a mut Point) {
        if self.a < self.b {
            let (first, second) = points.split_at_mut(self.b);
            (&mut first[self.a], &mut second[0])
        } else {
            let (first, second) = points.split_at_mut(self.a);
            (&mut second[0], &mut first[self.b])
        }
    }
}



/**
This function also applies velocity
**/
pub fn simple_velocity_step(points:&mut Vec<Point>, force:Vector3, delta:f32) {
    for i in 0..points.len(){
        let point = &mut points[i];
        if point.locked{continue}

        let prev_pos: Vector3 = point.pos.clone();
        point.pos = point.pos * 2.0 - point.prev_pos + force * delta * delta;
        point.prev_pos = prev_pos;
    }
}

pub fn simple_resolve_step(lines:&mut Vec<Line>,points:&mut Vec<Point>){
    for i in 0..lines.len() {
        let line:&mut Line = &mut lines[i];
        let (a,b) = line.get_points(points);
        let center:Vector3 = (a.pos+b.pos)*0.5;
        let mut dir:Vector3= a.pos-b.pos*(1.0/(a.pos-b.pos).length());

        if !a.locked{
            a.pos = center + dir * line.length * 0.5;
        }
        if !b.locked{
            b.pos = center - dir * line.length * 0.5;
        }
    }
}


pub fn aabb_dot_collision(aabbs:&Vec<AABB>,points:&mut Vec<Point>){
    for aabb in aabbs{
        for point in &mut *points{
            if aabb.in_box(&point){
                let top_dist = (point.pos.y-aabb.pos.y).abs();
                let left_dist = (point.pos.x-aabb.pos.x).abs();
                let front_dist = (point.pos.z-aabb.pos.z).abs();
                let bottom_dist = (aabb.pos.y+aabb.size.y-point.pos.y).abs();
                let right_dist = (aabb.pos.x+aabb.size.x-point.pos.x).abs();
                let back_dist = (aabb.pos.z+aabb.size.z-point.pos.z).abs();

                if front_dist<top_dist && front_dist<left_dist && front_dist<bottom_dist && front_dist<right_dist && front_dist<back_dist {
                    point.pos.z = aabb.pos.z;
                } else if back_dist<top_dist && back_dist<left_dist && back_dist<bottom_dist && back_dist<right_dist {
                    point.pos.z = aabb.pos.z+aabb.size.z;
                } else if top_dist<left_dist && top_dist<bottom_dist && top_dist<right_dist {
                    point.pos.y = aabb.pos.y
                } else if left_dist<bottom_dist && left_dist<right_dist {
                    point.pos.x = aabb.pos.x
                } else if bottom_dist<right_dist {
                    point.pos.y = aabb.pos.y+aabb.size.y
                } else {
                    point.pos.x = aabb.pos.x+aabb.size.x
                }
            }
        }
    }
}

pub fn aabb_line_collision(aabbs:&Vec<AABB>,points:&mut Vec<Point>){
    todo!()
}


pub fn create_rope(start:Vector3, length:f32, lines:usize, pin_first:bool) -> (Vec<Point>, Vec<Line>) {
    let mut next_pos:Vector3 = start+length;
    let mut points:Vec<Point> = Vec::with_capacity(lines + 1);
    points.push(Point::new(start.x, start.y, start.z, pin_first));
    for _ in 0..lines {
        points.push(Point::new(next_pos.x, next_pos.y, next_pos.z, false));
        next_pos = next_pos+length;
    }

    let mut rope:Vec<Line> = Vec::with_capacity(lines);
    for i in 0..lines {
        rope.push(Line{a: i, b: i + 1, length });
    }

    (points, rope)
}
/*
pub fn offset_line_points(lines: &mut Vec<Line>, offset:usize){
    for line in lines.iter_mut() {
        line.a = line.a + offset;
        line.b = line.b + offset;
    }
}*/