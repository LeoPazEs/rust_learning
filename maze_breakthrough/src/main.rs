// Why do we need loops?
fn print_maze(maze: &[[bool; 5]; 5]) {
    for i in 0..5 {
        for j in 0..5 {
            print!("{}x{}: {}|", i, j, if maze[i][j] { 1 } else { 0 });
        }
        println!();
    }
}
fn solve_maze(maze: &[[bool; 5]; 5], starting: [i8; 2], before: [i8; 2]) -> bool {
    if starting[0] + 1 < 5 && [starting[0] + 1, starting[1]] != before {
        if !maze[(starting[0] + 1) as usize][starting[1] as usize] {
            if !solve_maze(maze, [starting[0] + 1, starting[1]], starting) {
                print!(" {}x{} ", starting[0], starting[1]);
                return false;
            }
        }
    }
    if starting[0] - 1 >= 0 && [starting[0] - 1, starting[1]] != before {
        if !maze[(starting[0] - 1) as usize][starting[1] as usize] {
            if !solve_maze(maze, [starting[0] - 1, starting[1]], starting) {
                print!(" {}x{} ", starting[0], starting[1]);
                return false;
            }
        }
    }
    if starting[1] + 1 < 5 && [starting[0], starting[1] + 1] != before {
        if !maze[starting[0] as usize][(starting[1] + 1) as usize] {
            if !solve_maze(maze, [starting[0], starting[1] + 1], starting) {
                print!(" {}x{} ", starting[0], starting[1]);
                return false;
            }
        }
    }
    if starting[1] - 1 >= 0 && [starting[0], starting[1] - 1] != before {
        if !maze[starting[0] as usize][(starting[1] - 1) as usize] {
            if !solve_maze(maze, [starting[0], starting[1] - 1], starting) {
                print!(" {}x{} ", starting[0], starting[1]);
                return false;
            }
        }
    }
    if starting[0] == 4 && starting[1] == 4 {
        print!("{}x{} ", starting[0], starting[1]);
        return false;
    }
    return true;
}
fn main() {
    let mazes = [
        [
            [false, true, false, false, false],
            [false, true, false, true, false],
            [false, false, false, true, false],
            [false, true, false, true, false],
            [false, true, false, true, false],
        ],
        [
            [false, false, false, false, false],
            [true, true, true, true, false],
            [false, false, false, false, false],
            [false, true, true, true, true],
            [false, false, false, false, false],
        ],
        [
            [false, true, false, true, false],
            [false, true, false, true, false],
            [false, true, false, true, false],
            [false, true, false, true, false],
            [false, false, false, false, false],
        ],
        [
            [false, false, false, false, false],
            [false, true, true, true, false],
            [false, true, false, true, false],
            [true, true, true, true, false],
            [false, false, false, false, false],
        ],
        [
            [false, true, false, false, true],
            [false, true, false, true, false],
            [false, false, false, false, false],
            [true, true, true, true, true],
            [false, false, false, false, false],
        ],
    ];

    // Solve each maze
    for (i, maze) in mazes.iter().enumerate() {
        println!("Solving Maze {}:", i + 1);
        print_maze(maze);
        let starting: [i8; 2] = [0, 0];
        solve_maze(maze, starting, starting);
        println!(); // Add a newline for better separation
    }
}
