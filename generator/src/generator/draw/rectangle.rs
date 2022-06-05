use crate::node::{Direction, Node};

use super::draw_path;

pub fn generate_svg(maze: Vec<Node>, size: &[u32], cell_size: u32) -> String {
    // Extract values from size array
    let mut size_iter = size.into_iter();

    let width = *size_iter.next().unwrap();
    let height = *size_iter.next().unwrap();
    let start_x = *size_iter.next().unwrap();
    let start_y = *size_iter.next().unwrap();
    let end_x = *size_iter.next().unwrap();
    let end_y = *size_iter.next().unwrap();

    let mut children = Vec::new();

    // Draw edges
    // Top
    children.push(draw_path(
        None,
        Some("#000000"),
        draw_edge(
            Side::Top,
            cell_size,
            start_x,
            end_x,
            (start_y, end_y),
            width,
            height,
        ),
    ));

    // Bottom
    children.push(draw_path(
        None,
        Some("#000000"),
        draw_edge(
            Side::Bottom,
            cell_size,
            start_x,
            end_x,
            (start_y, end_y),
            width,
            height,
        ),
    ));

    // Left
    children.push(draw_path(
        None,
        Some("#000000"),
        draw_edge(
            Side::Left,
            cell_size,
            start_y,
            end_y,
            (start_x, end_x),
            height,
            width,
        ),
    ));

    // Right
    children.push(draw_path(
        None,
        Some("#000000"),
        draw_edge(
            Side::Right,
            cell_size,
            start_y,
            end_y,
            (start_x, end_x),
            height,
            width,
        ),
    ));

    let width = width as usize;
    let height = height as usize;
    let cell_size = cell_size as usize;

    // Draw nodes
    for i in 0..maze.len() {
        let node = &maze[i];

        let x = i % width * cell_size;
        let y = i / width * cell_size;

        for connection in node.get_connections() {
            if !connection.active {
                continue;
            }

            let start_x: usize;
            let start_y: usize;
            let end_x: usize;
            let end_y: usize;

            match connection.direction {
                Direction::Down => {
                    start_x = x;
                    start_y = y + cell_size;
                    end_x = x + cell_size;
                    end_y = y + cell_size;
                }
                Direction::Right => {
                    start_x = x + cell_size;
                    start_y = y;
                    end_x = x + cell_size;
                    end_y = y + cell_size;
                }
                _ => continue,
            }

            children.push(draw_path(
                None,
                Some("#000000"),
                vec![
                    format!("M {} {}", start_x, start_y),
                    format!("L {} {}", end_x, end_y),
                    String::from("Z"),
                ],
            ));
        }
    }

    format!(
        "<svg width=\"{}\" height=\"{}\">",
        width * cell_size,
        height * cell_size
    ) + &children.join("")
        + "</svg>"
}

enum Side {
    Top,
    Bottom,
    Left,
    Right,
}

fn draw_edge(
    side: Side,
    cell_size: u32,
    start: u32,
    end: u32,
    comparison: (u32, u32),
    side_len: u32,
    perpendicular: u32,
) -> Vec<String> {
    let mut commands = Vec::new();

    let direction: &str;
    let condition: u32;
    let move_command: String;

    match side {
        Side::Top => {
            commands.push(String::from("M 0 0"));
            direction = "H";
            condition = 0;
            move_command = format!("m {} 0", cell_size);
        }
        Side::Bottom => {
            commands.push(format!("M 0 {}", perpendicular * cell_size));
            direction = "H";
            condition = side_len - 1;
            move_command = format!("m {} 0", cell_size);
        }
        Side::Left => {
            commands.push(String::from("M 0 0"));
            direction = "V";
            condition = 0;
            move_command = format!("m {} 0", cell_size);
        }
        Side::Right => {
            commands.push(format!("M {} 0", perpendicular * cell_size));
            direction = "V";
            condition = side_len - 1;
            move_command = format!("m 0 {}", cell_size);
        }
    };

    match comparison {
        (s, e) if s == condition && e == condition => {
            let min = start.min(end);
            let max = start.max(end);

            commands.extend([
                format!("{} {}", direction, min * cell_size),
                move_command.clone(),
                format!("{} {}", direction, max * cell_size),
                move_command,
                format!("{} {}", direction, (side_len) * cell_size),
            ]);
        }
        (s, _) if s == condition => {
            commands.extend([
                format!("{} {}", direction, start * cell_size),
                move_command.clone(),
                format!("{} {}", direction, (side_len) * cell_size),
            ]);
        }
        (_, e) if e == condition => {
            commands.extend([
                format!("{} {}", direction, end * cell_size),
                move_command.clone(),
                format!("{} {}", direction, (side_len) * cell_size),
            ]);
        }
        _ => {
            commands.push(format!("{} {}", direction, (side_len) * cell_size));
        }
    };
    commands.push("Z".to_string());

    commands
}