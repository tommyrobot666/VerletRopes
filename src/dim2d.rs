use crate::verletcore::{Line};

pub struct Point {
    pub x:f32,
    pub y:f32,
    pub px:f32,
    pub py:f32,
    pub locked:bool,
}

impl Point {
    pub fn new(x:f32, y:f32, locked:bool) -> Point {
        Point {
            x,y,px:x,py:y,locked
        }
    }
}


pub struct AABB {
    pub x:f32,
    pub y:f32,
    pub width:f32,
    pub height:f32
}

impl AABB {
    pub fn in_box(&self, point: &Point) -> bool {
        point.x >= self.x && point.x <= self.x+self.width && point.y >= self.y && point.y <= self.y+self.height
    }
}

impl Line {
    pub fn get_both_points<'a>(&self, points: &'a mut [Point]) -> (&'a mut Point, &'a mut Point) {
        if self.a < self.b {
            let (first, second) = points.split_at_mut(self.b);
            (&mut first[self.a], &mut second[0])
        } else {
            let (first, second) = points.split_at_mut(self.a);
            (&mut second[0], &mut first[self.b])
        }
    }
}




#[allow(dead_code)]
pub fn simple_forces_to_points(points:&mut Vec<Point>, forces:Vec<[f32;2]>, delta:f32){
    for i in 0..points.len(){
        let point = &mut points[i];
        if point.locked{continue}

        let force = forces[i];
        let prev_pos:[f32;2] = [point.x,point.y];
        point.x += (point.x - point.px) + force[0] * delta * delta;
        point.y += (point.y - point.py) + force[1] * delta * delta;
        point.px = prev_pos[0];
        point.py = prev_pos[1];
    }
}

pub fn simple_force_to_points(points:&mut Vec<Point>, force:[f32;2], delta:f32) {
    for i in 0..points.len(){
        let point = &mut points[i];
        if point.locked{continue}

        let prev_pos:[f32;2] = [point.x,point.y];
        point.x += (point.x - point.px) + force[0] * delta * delta;
        point.y += (point.y - point.py) + force[1] * delta * delta;
        point.px = prev_pos[0];
        point.py = prev_pos[1];
    }
}


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