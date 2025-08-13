use std::any::Any;
use macroquad::prelude::*;
//put "cargo add macroquad" in terminal to install lib
#[macroquad::main("VerletRopes")]
async fn main() {
    let mut rope_data = create_rope([100.0, 100.0],30.0,17,true); //negative length makes it all clump up
    let (points, lines) = (&mut rope_data.0, &mut rope_data.1);
    let mut sim_paused:bool = true;
    let mut tool:ToolTypes = ToolTypes::Select;
    let mut selected:usize = 0;
    let mut steps = 15;

    loop {
        // update
        if is_key_pressed(KeyCode::A) {sim_paused = !sim_paused;}
        if is_key_down(KeyCode::Q) || !sim_paused {
            simple_force_to_points(points, [0.0, 1.0], 1.0);
            for _ in 0..steps {
                simple_sim_step(lines, points)
            }
        };
        if is_mouse_button_pressed(MouseButton::Left) {
            match tool {
                ToolTypes::Select => {
                    let (mx,my) = mouse_position();

                    for i in 0..points.len() {
                        let point = &mut points[i];

                        let dist_x = point.x - mx;
                        let dist_y = point.y - my;

                        if (dist_x*dist_x + dist_y*dist_y) < 25.0 {
                            selected = i;
                            break;
                        }
                    }
                },
                ToolTypes::MovePoint => {
                    let point = &mut points[selected];
                    (point.x, point.y) = mouse_position();
                    (point.px, point.py) = mouse_position();
                },
                ToolTypes::Lock => {
                    let point = &mut points[selected];
                    point.locked = !point.locked;
                },
                ToolTypes::Point => {
                    if is_key_down(KeyCode::Tab) {
                        let (mut new_points, mut new_lines) = create_rope(mouse_position().into(),35.0,15,true);
                        offset_line_points(&mut new_lines, points.len());
                        points.append(&mut new_points);
                        lines.append(&mut new_lines);
                    } else {
                        points.push(
                            Point::new(mouse_position().0, mouse_position().1, true)
                        );
                    }
                }
                ToolTypes::RemovePoint => {
                    //swap_remove is O(1) but randoms order
                    points.remove(selected);
                    lines.retain_mut(|line| {
                        if line.a == selected || line.b == selected {
                            return false;
                        }
                        if line.a > selected {
                            line.a -= 1;
                        }
                        if line.b > selected {
                            line.b -= 1;
                        }
                        true
                    });
                }
                ToolTypes::Line => {
                    // it was supposed to be that Line selects the first point and LineOtherPoint selects the second
                    // but Select already has one point selected, so only one line tool is needed
                    // edit: actually, im still going to do that (so that I don't have to switch tools)


                    // if i ever update this, remember to just ctrl-c+v all of the code from Select
                    let (mx,my) = mouse_position();
                    for i in 0..points.len() {
                        let point = &mut points[i];

                        let dist_x = point.x - mx;
                        let dist_y = point.y - my;

                        if (dist_x*dist_x + dist_y*dist_y) < 25.0 {
                            selected = i;
                            break;
                        }
                    }

                    // the only unique thing that makes this different from Select
                    tool = ToolTypes::LineOtherPoint;
                }
                ToolTypes::LineOtherPoint => {
                    let mut other_point = selected;

                    // if i ever update this, remember to just ctrl-c+v all of the code from Select
                    // and change "selected" to other_point
                    let (mx,my) = mouse_position();
                    for i in 0..points.len() {
                        let point = &mut points[i];

                        let dist_x = point.x - mx;
                        let dist_y = point.y - my;

                        if (dist_x*dist_x + dist_y*dist_y) < 25.0 {
                            other_point = i;
                            break;
                        }
                    }

                    // the real code
                    if other_point != selected {
                        lines.push(
                            Line {
                                a: other_point,
                                b: selected,
                                length: 40.0,
                            }
                        );
                        tool = ToolTypes::Line;
                    }
                }
            }
        }
        if is_key_down(KeyCode::Key1) {
            tool = ToolTypes::Select;
        } else if is_key_down(KeyCode::Key2) {
            tool = ToolTypes::MovePoint;
        } else if is_key_down(KeyCode::Key3) {
            tool = ToolTypes::Point;
        } else if is_key_down(KeyCode::Key4) {
            tool = ToolTypes::Lock;
        } else if is_key_down(KeyCode::Key5) {
            tool = ToolTypes::RemovePoint;
        } else if is_key_down(KeyCode::Key6) {
            tool = ToolTypes::Line;
        }

        if is_key_pressed(KeyCode::W) || is_key_down(KeyCode::E) {
            steps += 1;
        } else if is_key_pressed(KeyCode::S) || is_key_down(KeyCode::D) {
            steps -= 1;
        }

        // draw
        clear_background(BLACK);

        for line in lines.iter() {
            let (a,b) = line.get_both_points(points);
            draw_line(a.x, a.y, b.x, b.y, 2.0, WHITE);
        }

        for point in points.iter() {
            draw_circle(point.x, point.y, 5.0, if point.locked { GOLD } else { RED });
        }

        let selected_point = &points[selected];
        draw_circle(selected_point.x, selected_point.y, 4.0, BLUE);

        if tool.to_string() == ToolTypes::LineOtherPoint.to_string() {
            let point = &points[selected];
            draw_line(point.x, point.y, mouse_position().0, mouse_position().1, 3.0, GREEN);
        }

        draw_text(&steps.to_string(),301.0,401.0,20.0,GREEN);
        draw_text(&tool.to_string(),301.0,431.0,20.0,GREEN);
        draw_text(&steps.to_string(),300.0,400.0,20.0,WHITE);
        draw_text(&tool.to_string(),300.0,430.0,20.0,WHITE);

        next_frame().await
    }
}

enum ToolTypes {
    Point,
    Line,
    LineOtherPoint,
    RemovePoint,
    MovePoint,
    Lock,
    Select
}

impl ToolTypes {
    fn to_string(&self) -> &'static str {
        match self {
            ToolTypes::Select => {"Select"},
            ToolTypes::MovePoint => {"Move Point"},
            ToolTypes::Lock => {"Throw away the key"},
            ToolTypes::Point => {"Add point"},
            ToolTypes::RemovePoint => {"Murder the point and hide the evidence"},
            ToolTypes::Line => {"Start the creation of entire universes"},
            ToolTypes::LineOtherPoint => {"You are now using a different tool!?!? (line ender)"}
        }
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


fn create_rope(start:[f32;2], length:f32, lines:usize, pin_first:bool) -> (Vec<Point>, Vec<Line>) {
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

fn offset_line_points(lines: &mut Vec<Line>, offset:usize){
    for line in lines.iter_mut() {
        line.a = line.a + offset;
        line.b = line.b + offset;
    }
}