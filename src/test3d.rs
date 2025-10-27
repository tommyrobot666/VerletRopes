use crate::{
    verletcore::{*},
    dim3d::{*}
};

use macroquad::prelude::*;

pub async fn main() {
    let mut rope_data = create_rope(Vector3{x:0.1,y:0.1,z:100.0},0.3,17,true);
    let (points, lines) = (&mut rope_data.0, &mut rope_data.1);
    let aabbs = &mut Vec::with_capacity(8);
    let mut sim_paused:bool = true;
    let mut tool:ToolTypes = ToolTypes::Select;
    let mut selected:usize = 0;
    let mut steps = 15;
    let mut box_corner: (f32,f32) = (0.0,0.0); // type of [f32;2] caused no error, but looked wierd

    loop {
        // update
        if is_key_pressed(KeyCode::A) {sim_paused = !sim_paused;}
        if is_key_down(KeyCode::Q) || !sim_paused {
            simple_velocity_step(points, Vector3{x:0.0,y:1.0,z:0.0}, 1.0);
            for _ in 0..steps {
                simple_resolve_step(lines, points)
            }
            aabb_dot_collision(aabbs, points);
        };
        if tool != ToolTypes::Line {
            let point_near_mouse = get_point_near_mouse(points);
            if !point_near_mouse.is_none() {
                selected = point_near_mouse.unwrap();
            }
        }
        if is_mouse_button_pressed(MouseButton::Left) {
            match tool {
                ToolTypes::Select => {
                    draw_text("This test uses auto select",50.0,50.0,20.0,WHITE);
                },
                ToolTypes::MovePoint => {
                    let point = &mut points[selected];
                    (point.pos.x, point.pos.y) = mouse_position();
                    (point.prev_pos.x, point.prev_pos.y) = mouse_position();
                },
                ToolTypes::Lock => {
                    let point = &mut points[selected];
                    point.locked = !point.locked;
                },
                ToolTypes::Point => {
                    let (mx,my) = mouse_position();
                    if is_key_down(KeyCode::Tab) {
                        combine_simulations((points,lines),create_rope(Vector3{x:mx,y:my,z:100.0},35.0,15,true));
                    } else {
                        points.push(
                            Point::new(mx, my, 100.0,true)
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
                    let point_near_mouse = get_point_near_mouse(points);

                    if !point_near_mouse.is_none() {
                        let point = point_near_mouse.unwrap();
                        if point != selected {
                            lines.push(
                                Line {
                                    a: point,
                                    b: selected,
                                    length: 40.0,
                                }
                            );
                        }
                    }
                }
                ToolTypes::AABB => {
                    box_corner = mouse_position().into();
                    tool = ToolTypes::AABBOtherPoint;
                }
                ToolTypes::AABBOtherPoint => {
                    let other_box_corner: (f32,f32) = mouse_position().into();
                    aabbs.push(
                        AABB {
                            pos: Vector3{x:box_corner.0,y:box_corner.1,z:50.0},
                            size: Vector3{x:other_box_corner.0*100.0,y:other_box_corner.1*100.0,z:100.0}
                        }
                    );
                    tool = ToolTypes::AABB;
                }
                _ => {
                    draw_text("This tool doesn't exist",50.0,50.0,20.0,WHITE);
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
        } else if is_key_down(KeyCode::Key7) {
            tool = ToolTypes::AABB;
        }

        if is_key_pressed(KeyCode::W) || is_key_down(KeyCode::E) {
            steps += 1;
        } else if is_key_pressed(KeyCode::S) || is_key_down(KeyCode::D) {
            steps -= 1;
        }

        // draw
        clear_background(BLACK);
        let screen_size = (screen_width(),screen_height());

        for aabb in aabbs.iter() {
            let pos1 = aabb.pos.project_vertex_screen(screen_size, 1.0);
            let pos2 = (aabb.pos + aabb.size).project_vertex_screen(screen_size, 1.0);
            draw_rectangle(pos1.0, pos1.1, pos2.0, pos2.1, MAGENTA);
        }

        for line in lines.iter() {
            let (a,b) = line.get_points(points);
            let pos1 = a.pos.project_vertex_screen(screen_size, 1.0);
            let pos2 = b.pos.project_vertex_screen(screen_size, 1.0);
            draw_line(pos1.0, pos1.1, pos2.0, pos2.1, 2.0, WHITE);
        }

        for point in points.iter() {
            if point.pos.on_screen(screen_size, 1.0) {
                let pos1 = point.pos.project_vertex_screen(screen_size, 1.0);
                println!("{},{}", pos1.0, pos1.1);
                draw_circle(pos1.0, pos1.1, 5.0, if point.locked { GOLD } else { RED });
            }
        }

        {
            let selected_point = &points[selected];
            let pos1 = selected_point.pos.project_vertex_screen(screen_size, 1.0);
            draw_circle(pos1.0, pos1.1, 4.0, BLUE);
        }

        if tool.to_string() == ToolTypes::LineOtherPoint.to_string() {
            let point = &points[selected];
            let pos1 = point.pos.project_vertex_screen(screen_size, 1.0);
            draw_line(pos1.0, pos1.1, mouse_position().0, mouse_position().1, 3.0, GREEN);
        }

        if tool.to_string() == ToolTypes::AABBOtherPoint.to_string() {
            draw_rectangle(box_corner.0, box_corner.1, mouse_position().0-box_corner.0, mouse_position().1-box_corner.1, PINK);
        }

        draw_text(&steps.to_string(),301.0,401.0,20.0,RED);
        draw_text(&tool.to_string(),301.0,431.0,20.0,RED);
        draw_text(&steps.to_string(),300.0,400.0,20.0,WHITE);
        draw_text(&tool.to_string(),300.0,430.0,20.0,WHITE);

        // return to test select
        if is_key_down(KeyCode::Escape) {
            break;
        }
        next_frame().await
    }
}

fn get_point_near_mouse(points:&mut Vec<Point>) -> Option<usize> {
    let (mx,my) = mouse_position();

    for i in 0..points.len() {
        let point = &mut points[i];

        let dist_x = (point.pos.x * screen_width() * 1.0) - mx;
        let dist_y = (point.pos.y * screen_height() * 1.0) - my;

        if (dist_x*dist_x + dist_y*dist_y) < 100.0 {
            return Some(i);
        }
    }
    None
}
