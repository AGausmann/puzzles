use glam::{ivec2, IVec2, IVec3};
use std::collections::HashMap;
use std::io::{stdin, Read};

fn main() -> anyhow::Result<()> {
    let mut input = String::new();
    stdin().read_to_string(&mut input)?;

    let (map, instrs) = input.split_once("\n\n").unwrap();
    let map: Vec<Vec<char>> = map.lines().map(|line| line.chars().collect()).collect();
    let start = ivec2(map[0].iter().take_while(|&&c| c != '.').count() as i32, 0);

    // Part 1
    {
        let mut position = start;
        let mut direction = ivec2(1, 0);
        let mut facing = 0;

        for instr in instrs.trim().split_inclusive(['L', 'R']) {
            let run: i32 = instr
                .strip_suffix(['L', 'R'])
                .unwrap_or(instr)
                .parse()
                .unwrap();

            for _ in 0..run {
                let next = position + direction;
                match map
                    .get(next.y as usize)
                    .and_then(|row| row.get(next.x as usize))
                {
                    Some('.') => {
                        position = next;
                    }
                    Some('#') => {
                        break;
                    }
                    None | Some(' ') => {
                        let opposite_edge = (0i32..)
                            .map(|i| position - direction * i)
                            .find(|coord| {
                                match map
                                    .get(coord.y as usize)
                                    .and_then(|row| row.get(coord.x as usize))
                                {
                                    None | Some(' ') => true,
                                    _ => false,
                                }
                            })
                            .unwrap();

                        let next = opposite_edge + direction;
                        if map[next.y as usize][next.x as usize] == '#' {
                            break;
                        } else {
                            position = next;
                        }
                    }
                    _ => panic!(),
                }
            }

            if instr.ends_with('L') {
                direction = ivec2(direction.y, -direction.x);
                facing = (facing + 3) % 4;
            } else if instr.ends_with('R') {
                direction = ivec2(-direction.y, direction.x);
                facing = (facing + 1) % 4;
            }
        }

        let ans = 1000 * (position.y + 1) + 4 * (position.x + 1) + facing;
        println!("{}", ans);
    }

    // Part 2
    {
        // Solve the relative orientations of each face.
        let face_size = if map.len() >= 50 { 50 } else { 4 };
        let start_face = start / face_size;

        #[derive(Debug, Clone, Copy)]
        struct FaceInfo {
            normal: IVec3,
            map_x_axis: IVec3,
            map_y_axis: IVec3,
        }
        impl FaceInfo {
            fn direction_2to3(&self, direction: IVec2) -> IVec3 {
                self.map_x_axis * direction.x + self.map_y_axis * direction.y
            }

            fn direction_3to2(&self, direction: IVec3) -> IVec2 {
                IVec2::new(
                    self.map_x_axis.dot(direction),
                    self.map_y_axis.dot(direction),
                )
            }
        }

        let mut faces = HashMap::new();
        let mut face_normals = HashMap::new();
        face_normals.insert(-IVec3::Z, start_face);
        faces.insert(
            start_face,
            FaceInfo {
                normal: -IVec3::Z,
                map_x_axis: IVec3::X,
                map_y_axis: IVec3::Y,
            },
        );
        let mut queue = vec![start_face];
        let neighbor_offsets = [IVec2::X, IVec2::Y, -IVec2::X, -IVec2::Y];
        while let Some(current_face_location) = queue.pop() {
            let current_face = faces[&current_face_location];
            for n in neighbor_offsets {
                let neighbor_location = current_face_location + n;
                if faces.contains_key(&neighbor_location) {
                    continue;
                }

                let neighbor_cell = neighbor_location * face_size;
                match map
                    .get(neighbor_cell.y as usize)
                    .and_then(|row| row.get(neighbor_cell.x as usize))
                {
                    Some(' ') | None => {
                        continue;
                    }
                    _ => {}
                }

                let neighbor_normal = current_face.direction_2to3(n);
                let neighbor_info = FaceInfo {
                    normal: neighbor_normal,
                    map_x_axis: if n.abs() == IVec2::X {
                        -current_face.normal * n.x
                    } else {
                        current_face.map_x_axis
                    },
                    map_y_axis: if n.abs() == IVec2::Y {
                        -current_face.normal * n.y
                    } else {
                        current_face.map_y_axis
                    },
                };
                face_normals.insert(neighbor_normal, neighbor_location);
                faces.insert(neighbor_location, neighbor_info);
                queue.push(neighbor_location);
            }
        }

        let mut position = start;
        let mut direction = ivec2(1, 0);

        for instr in instrs.trim().split_inclusive(['L', 'R']) {
            let run: i32 = instr
                .strip_suffix(['L', 'R'])
                .unwrap_or(instr)
                .parse()
                .unwrap();

            for _ in 0..run {
                let next = position + direction;
                match map
                    .get(next.y as usize)
                    .and_then(|row| row.get(next.x as usize))
                {
                    Some('.') => {
                        position = next;
                    }
                    Some('#') => {
                        break;
                    }
                    None | Some(' ') => {
                        let from_face_location = position / face_size;
                        let from_face = &faces[&from_face_location];
                        let from_direction_3d = from_face.direction_2to3(direction);

                        let to_face_location = face_normals[&from_direction_3d];
                        let to_face = &faces[&to_face_location];
                        let to_direction_3d = -from_face.normal;
                        let to_direction = to_face.direction_3to2(to_direction_3d);

                        let from_edge = IVec2::new(-direction.y, direction.x);
                        let edge_offset = from_edge.dot(position % face_size);

                        let to_edge = IVec2::new(-to_direction.y, to_direction.x);
                        let mut to_position_on_edge = to_edge * edge_offset;
                        if to_position_on_edge.x < 0 {
                            to_position_on_edge.x = face_size - 1 + to_position_on_edge.x;
                        }
                        if to_position_on_edge.y < 0 {
                            to_position_on_edge.y = face_size - 1 + to_position_on_edge.y;
                        }

                        let to_position = to_face_location * face_size
                            + to_position_on_edge
                            + (-to_direction).max(IVec2::ZERO) * (face_size - 1);

                        if map[to_position.y as usize][to_position.x as usize] == '#' {
                            break;
                        } else {
                            position = to_position;
                            direction = to_direction;
                        }
                    }
                    _ => panic!(),
                }
            }

            if instr.ends_with('L') {
                direction = ivec2(direction.y, -direction.x);
            } else if instr.ends_with('R') {
                direction = ivec2(-direction.y, direction.x);
            }
        }

        let facing = match <[i32; 2]>::from(direction) {
            [1, 0] => 0,
            [0, 1] => 1,
            [-1, 0] => 2,
            [0, -1] => 3,
            _ => panic!(),
        };

        let ans = 1000 * (position.y + 1) + 4 * (position.x + 1) + facing;
        println!("{}", ans);
    }

    Ok(())
}
