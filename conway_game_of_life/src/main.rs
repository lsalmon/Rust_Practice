use raylib::prelude::*;
use raylib::ffi::{EndDrawing, GetFrameTime, GetCharPressed, GetMousePosition, GetTime, IsKeyDown, IsKeyPressed, IsMouseButtonDown, IsMouseButtonPressed, IsMouseButtonReleased, WaitTime};
use raylib::ffi::MouseButton;
use raylib::ffi::KeyboardKey;

fn main() {
    let mut grid: [[bool; 200]; 200] = [[false; 200]; 200];
    let width: i32 = 4;
    let height: i32 = 3;
    let screen_w: i32 = 800;
    let screen_h: i32 = 600;

    const NEIGHBOUR_CELLS: [(i32, i32); 8] = 
    [
        (-1, -1), (-1, 0), (-1, 1),
        (0, -1), /* no (0, 0) */ (0, 1),
        (1, -1), (1, 0), (1, 1)
    ];

    let (mut rl, thread) = raylib::init()
        .size(screen_w, screen_h)
        .title("Conway")
        .build();

    let mut timer: f64;
    unsafe {
        timer = GetTime();
    }
    let display_frame_delay: f64 = 1.0;
    let mut display_frame: bool = true;

    while !rl.window_should_close() {
        let mut x: i32;
        let mut y: i32;

        unsafe {
            if display_frame {
                display_frame = false;
                timer = GetTime();
            }
            else if (GetTime() - timer) > display_frame_delay {
                display_frame = true;
                timer = GetTime();
            }

            if IsMouseButtonDown(MouseButton::MOUSE_BUTTON_LEFT as i32) {
                let mut pos = GetMousePosition();
                pos.x /= width as f32;
                pos.y /= height as f32;
                let user_x = pos.x as i32;
                let user_y = pos.y as i32;
                if user_x >= 0 && user_x < 200
                    && user_y >= 0 && user_y < 200 {
                    grid[user_y as usize][user_x as usize] = true;
                }
            }
        }

        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::WHITE);

        let run_automaton = d.is_key_down(KeyboardKey::KEY_SPACE);

        // Temp grid to borrow in the loop
        let mut next_grid = grid;

        for (i, row) in next_grid.iter_mut().enumerate() {
            for (j, col) in row.iter_mut().enumerate() {
                x = (j as i32)*width;
                y = (i as i32)*height;

                // Check neighbouring cells
                let mut live_neigh_cell_cnt: i32 = 0;
                for (shift_r, shift_c) in NEIGHBOUR_CELLS {
                    let neigh_cell: (i32, i32) = (x+shift_r, y+shift_c);
                    if neigh_cell.0 >= 0 && neigh_cell.0 < grid.len() as i32
                        && neigh_cell.1 >= 0 && neigh_cell.1 < grid.len() as i32
                        && grid[neigh_cell.1 as usize][neigh_cell.0 as usize] {
                            live_neigh_cell_cnt += 1;
                        }
                }

                if run_automaton && display_frame {
                    if !*col && live_neigh_cell_cnt == 3 {
                        *col = true;
                    }
                    else if *col {
                        if live_neigh_cell_cnt < 2 {
                            *col = false;
                        }
                        else if live_neigh_cell_cnt == 2 || live_neigh_cell_cnt == 3 {
                            // Technically do nothing
                            *col = true;
                        }
                        else if live_neigh_cell_cnt > 3 {
                            *col = false;
                        }
                    }
                }

                let c: Color = if *col { Color::BLACK } else { Color::WHITE };

                d.draw_rectangle(x, y, width, height, c);
            }
        }

        grid = next_grid;
    }
}
