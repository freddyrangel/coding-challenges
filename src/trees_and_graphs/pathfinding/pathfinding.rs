/*
 * PATHFINDING
 * https://btholt.github.io/complete-intro-to-computer-science/pathfinding
 *
 * write in a function thats a X by X array of arrays of numbers
 * as well two x/y combinations and have it return the shortest
 * length (you don't need to track the actual path) from point A
 * to point B.
 *
 * the numbers in the maze array represent as follows:
 * 0 – open space
 * 1 - closed space, cannot pass through. a wall
 * 2 - one of the two origination points
 *
 * you will almost certainly need to transform the maze into your own
 * data structure to keep track of all the meta data
 */

#[cfg(test)]
mod tests {
    use super::*;

    //#[test]
    //fn should_solve_a_4x4_() {
    //let mut maze = Maze::new(vec![
    //vec![2, 0, 0, 0],
    //vec![0, 0, 0, 0],
    //vec![0, 0, 0, 0],
    //vec![0, 0, 0, 2],
    //]);

    //assert_eq!(maze.find_shortest_path_length((0, 0), (3, 3)), 6);
    //}

    //#[test]
    //fn should_solve_a_6x6_maze() {
    //let maze = Maze::new(vec![
    //vec![0, 0, 0, 0, 0, 0],
    //vec![0, 2, 0, 0, 0, 0],
    //vec![0, 0, 0, 0, 0, 0],
    //vec![0, 1, 1, 1, 1, 1],
    //vec![0, 0, 0, 0, 0, 0],
    //vec![0, 0, 2, 0, 0, 0],
    //]);

    //assert_eq!(maze.find_shortest_path_length((1, 1), (2, 5)), 7);
    //}

    //#[test]
    //fn should_solve_a_8x8_maze() {
    //let maze = Maze::new(vec![
    //vec![0, 0, 1, 0, 0, 0, 0, 0],
    //vec![0, 0, 0, 0, 0, 0, 0, 0],
    //vec![0, 0, 1, 0, 0, 0, 0, 1],
    //vec![0, 0, 0, 0, 0, 1, 0, 0],
    //vec![0, 0, 0, 1, 0, 1, 1, 0],
    //vec![0, 0, 0, 0, 0, 0, 1, 0],
    //vec![0, 2, 0, 0, 0, 0, 1, 0],
    //vec![0, 0, 0, 0, 0, 0, 1, 2],
    //]);

    //assert_eq!(maze.find_shortest_path_length((1, 7), (7, 7)), 16);
    //}

    //#[test]
    //fn should_solve_a_15x15_maze() {
    //let maze = Maze::new(vec![
    //vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    //vec![0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    //vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    //vec![0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0],
    //vec![0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0],
    //vec![0, 0, 1, 1, 0, 0, 0, 1, 1, 1, 0, 0, 1, 1, 0],
    //vec![0, 0, 1, 0, 0, 1, 1, 0, 0, 0, 1, 0, 0, 1, 0],
    //vec![0, 0, 1, 0, 1, 0, 0, 0, 1, 0, 0, 1, 0, 1, 0],
    //vec![0, 0, 1, 0, 1, 0, 1, 1, 2, 1, 0, 1, 0, 1, 0],
    //vec![0, 0, 1, 0, 1, 0, 0, 1, 0, 1, 0, 1, 0, 1, 0],
    //vec![0, 0, 1, 0, 0, 1, 0, 0, 0, 1, 0, 1, 0, 1, 0],
    //vec![0, 0, 0, 1, 0, 0, 1, 1, 1, 1, 0, 1, 0, 1, 0],
    //vec![0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0],
    //vec![0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 0, 0, 1, 0],
    //vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    //]);

    //assert_eq!(maze.find_shortest_path_length((1, 1), (8, 8)), 78);
    //}
}
