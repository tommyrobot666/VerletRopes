pub struct Line {
    pub a: usize,
    pub b: usize,
    pub length:f32
}

pub enum ToolTypes {
    Point,
    Line,
    LineOtherPoint,
    RemovePoint,
    MovePoint,
    Lock,
    Select,
    AABB,
    AABBOtherPoint
}

impl ToolTypes {
    pub fn to_string(&self) -> &'static str {
        match self {
            ToolTypes::Select => {"Select"},
            ToolTypes::MovePoint => {"Move Point"},
            ToolTypes::Lock => {"Throw away the key"},
            ToolTypes::Point => {"Add point (hold Tab to create chain)"},
            ToolTypes::RemovePoint => {"Murder the point and hide the evidence"},
            ToolTypes::Line => {"Start the creation of entire universes"},
            ToolTypes::LineOtherPoint => {"You are now using a different tool!?!? (line ender)"}
            ToolTypes::AABB => {"Make a purple box thing (or else)"},
            ToolTypes::AABBOtherPoint => {"How big will this box be?"},
        }
    }
}


pub fn offset_line_points(lines: &mut Vec<Line>, offset:usize){
    for line in lines.iter_mut() {
        line.a = line.a + offset;
        line.b = line.b + offset;
    }
}