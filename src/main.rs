use macroquad::prelude::*;
//put "cargo add macroquad" in termanal to install lib
#[macroquad::main("VerletRopes")]
async fn main() {
    // let &mut (mut points, mut lines) = &mut create_rope([0.0,0.0],-10.0,10);
    // let points:&mut Vec<&mut Point> = &mut Line::extract_points(lines.to_vec());
    let mut rope_data = create_rope([0.0, 0.0],-10.0,10);
    let (points, lines) = (&mut rope_data.0, &mut rope_data.1);

    loop {
        // update
        simple_force_to_points(points, [0.0,1.0], 1.0);
        simple_sim_step(lines);
        // draw
        clear_background(BLACK);

        for point in points.iter() {
            draw_circle(point.x, point.y, 5.0, RED);
        }

        for line in lines.iter() {
            draw_line(line.a.x, line.a.y, line.b.x, line.b.y, 3.0, WHITE);
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

struct PointAndForce<'a> {
    pub point:&'a mut Point,
    pub force:[f32;2]
}

struct Line<'a> {
    pub a: &'a mut Point,
    pub b: &'a mut Point,
    pub length:f32
}

impl<'a> Line<'a> {
    fn extract_points(lines:Vec<Line<'a>>) -> Vec<&'a mut Point> {
        let mut out:Vec<&'a mut Point> = Vec::with_capacity(lines.len());
        for line in lines {
            if !out.iter().any(|i| (*i as *const Point) == (line.a as *const Point)) {
                out.push(line.a);
            }
            if !out.iter().any(|i| (*i as *const Point) == (line.b as *const Point)) {
                out.push(line.b);
            }
        }
        out
    }
}

fn simple_forces_to_points(mut points:&mut Vec<PointAndForce>, delta:f32){
    for i in 0..points.len(){
        let point = &mut points[i].point;
        if point.locked{continue}

        let force = &mut points[i].force;
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


fn simple_sim_step(lines:&mut Vec<Line>){
    for i in 0..lines.len() {
        let line:&mut Line = &mut lines[i];
        let center:[f32;2] = [(line.a.x+line.b.x)/2.0,(line.a.y+line.b.y)/2.0];
        let mut dir:[f32;2] = [line.a.x-line.b.x,line.a.y-line.b.y];
        // normalize dir
        let dir_len:f32 = (dir[0]*dir[0]+dir[1]*dir[1]).sqrt(); dir[0] = dir[0]/dir_len; dir[1] = dir[1]/dir_len;

        if !line.a.locked{
            line.a.x = center[0] + (dir[0] * line.length) / 2.0;
            line.a.y = center[1] + (dir[1] * line.length) / 2.0;
        }
        if !line.b.locked{
            line.b.x = center[0] - (dir[0] * line.length) / 2.0;
            line.b.y = center[1] - (dir[1] * line.length) / 2.0;
        }
    }
}


fn create_rope<'a>(start:[f32;2],length:f32,lines:i8)-> (Vec<Point>, Vec<Line<'a>>) {
    let mut next_pos:[f32;2] = [start[0]+length,start[1]];
    let mut rope:Vec<Line<'a>> = Vec::with_capacity(lines as usize);
    let mut points:Vec<Point> = Vec::with_capacity(lines as usize + 1);
    points.push(Point::new(start[0],start[1],true));
    let mut last_point:&Point = points.get(0).unwrap();


    for _ in 0..lines {
        points.push(Point::new(next_pos[0], next_pos[1], false));
        let mut current_point = points.get(points.len()-1).unwrap();
        next_pos[0] += length;
        rope.push(Line{a: &mut current_point, b: &mut last_point, length });

        last_point = current_point;
    }

    (points, rope)
}