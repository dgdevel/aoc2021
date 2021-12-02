
use super::aocutil;

struct Instruction {
    verb : String,
    value : u32
}


fn parse_instruction(line: &String) -> Instruction {
    let parts = line.split(" ").collect::<Vec<&str>>();
    Instruction { verb:parts[0].to_string(), value:parts[1].parse::<u32>().unwrap() }
}

struct Position {
    x_horizontal : u32,
    y_depth : u32,
    z_aim : u32
}

pub fn p2_1() -> String {
    let lines = aocutil::read_file_to_string_list("p2_1".to_string());
    let instructions = lines.iter().map(parse_instruction).collect::<Vec<Instruction>>();
    let final_state = instructions.iter().fold(Position {x_horizontal:0, y_depth:0, z_aim:0}, |state, instruction| {
        if instruction.verb.eq("forward") {
            Position { x_horizontal:state.x_horizontal + instruction.value, y_depth:state.y_depth, z_aim:0 }
        } else if instruction.verb.eq("down") {
            Position { x_horizontal:state.x_horizontal, y_depth:state.y_depth + instruction.value, z_aim:0 }
        } else if instruction.verb.eq("up") {
            Position { x_horizontal:state.x_horizontal, y_depth:state.y_depth - instruction.value, z_aim:0 }
        } else {
            state
        }
    });
    let result = final_state.x_horizontal * final_state.y_depth;
    format!("{:?}", result).to_string()
}

pub fn p2_2() -> String {
    let lines = aocutil::read_file_to_string_list("p2_1".to_string());
    let instructions = lines.iter().map(parse_instruction).collect::<Vec<Instruction>>();
    let final_state = instructions.iter().fold(Position {x_horizontal:0, y_depth:0, z_aim:0}, |state, instruction| {
        if instruction.verb.eq("forward") {
            Position {
                x_horizontal:state.x_horizontal + instruction.value,
                y_depth:state.y_depth + instruction.value * state.z_aim, 
                z_aim:state.z_aim
            }
        } else if instruction.verb.eq("down") {
            Position {
                x_horizontal:state.x_horizontal,
                y_depth:state.y_depth,
                z_aim:state.z_aim + instruction.value
            }
        } else if instruction.verb.eq("up") {
            Position {
                x_horizontal:state.x_horizontal,
                y_depth:state.y_depth,
                z_aim:state.z_aim - instruction.value
            }
        } else {
            state
        }
    });
    let result = final_state.x_horizontal * final_state.y_depth;
    format!("{:?}", result).to_string()
}

