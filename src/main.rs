use macroquad::prelude::*;
//put "cargo add macroquad" in termanal to install lib
#[macroquad::main("VerletRopes")]
async fn main() {
    let mut rope_data = create_rope([50.0, 50.0],-10.0,10);
    let (points, lines) = (&mut rope_data.0, &mut rope_data.1);

    loop {
        // update
        if is_key_down(KeyCode::Q) {
            simple_force_to_points(points, [0.0, 1.0], 1.0);
            for _ in 0..1 {
                simple_sim_step(lines, points)
            }
        };
        // draw
        clear_background(BLACK);

        for point in points.iter() {
            draw_circle(point.x, point.y, 5.0, RED);
        }

        for line in lines.iter() {
            let (a,b) = line.get_both_points(points);
            draw_line(a.x, a.y, b.x, b.y, 3.0, WHITE);
        }

        next_frame().await
    }
}

struct Point {
    pub x:f32,
    pub y:f32,
    pub px:f32,
    pub py:f32,
    pub locked:bool
}

impl Point {
    fn new(x:f32,y:f32,locked:bool) -> Point {
        Point {
            x,y,px:x,py:y,locked
        }
    }
}

struct Line {
    pub a: usize,
    pub b: usize,
    pub length:f32
}

impl Line {
    fn get_both_points<'a>(&self, points: &'a mut [Point]) -> (&'a mut Point, &'a mut Point) {
        if self.a < self.b {
            let (first, second) = points.split_at_mut(self.b);
            (&mut first[self.a], &mut second[0])
        } else {
            let (first, second) = points.split_at_mut(self.a);
            (&mut second[0], &mut first[self.b])
        }
    }
}

fn simple_forces_to_points(points:&mut Vec<Point>, forces:Vec<[f32;2]>, delta:f32){
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

fn simple_force_to_points(points:&mut Vec<Point>, force:[f32;2], delta:f32) {
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


fn simple_sim_step(lines:&mut Vec<Line>,points:&mut Vec<Point>){
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


fn create_rope(start:[f32;2],length:f32,lines:usize)-> (Vec<Point>, Vec<Line>) {
    let mut next_pos:[f32;2] = [start[0]+length,start[1]];
    let mut points:Vec<Point> = Vec::with_capacity(lines + 1);
    points.push(Point::new(start[0],start[1],true));
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