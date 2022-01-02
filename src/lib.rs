use std::collections::VecDeque;

#[derive(Debug, PartialEq)]
pub struct Solution {}

/* key takeaways
   - start with each building and use BFS to calculate
     the distance from the building to a every reachable
     empty land
     - a data structure to sum up the total distance from
       all buildings per empty land
   - find the min total distance among empty lands. To qualify
     - the empty land should have been visited by all buildings
*/

impl Solution {
  pub fn shortest_distance(grid: Vec<Vec<usize>>) -> isize {
    let rows = grid.len();
    if rows == 0 {
      return -1;
    };
    let cols = grid[0].len();
    if cols == 0 {
      return -1;
    }
    /*
      - the empty land of choice must be reachable by
        every single building to qualify
      - so we need to record how many buildings we
        have
    */
    let mut num_buildings = 0;

    /*
      - sum up the total distance from all buildings
        to a given empty land
      - the distance from each building is the level
        at which the empty land is reached
        by a building using bfs
    */
    let mut total_distance = vec![vec![0; cols]; rows];
    /*
      - how many buildings have reached a given empty
        land using bfs
    */
    let mut num_buildings_reached = vec![vec![0; cols]; rows];
    println!("dist: {:?}", total_distance);

    for row in 0..rows {
      for col in 0..cols {
        /* if it is a building
          - count it
          - and do a bfs from it
        */
        if grid[row][col] == 1 {
          num_buildings = num_buildings + 1;
          Self::bfs(
            &grid,
            &mut total_distance,
            &mut num_buildings_reached,
            row,
            col,
          );
        }
      }
    }

    /*
      - we have to make sure everyone is smaller
        than this initially
    */
    let mut min_total_distance = usize::MAX;

    for row in 0..rows {
      for col in 0..cols {
        /*
          - must be an empty land
          - must have been visited by all buildings
        */
        if grid[row][col] == 0
          && total_distance[row][col] > 0
          && num_buildings_reached[row][col] == num_buildings
        {
          if total_distance[row][col] < min_total_distance {
            min_total_distance = total_distance[row][col];
          }
        }
      }
    }

    if min_total_distance != usize::MAX {
      return min_total_distance as isize;
    }

    -1
  }

  fn bfs(
    grid: &Vec<Vec<usize>>,
    total_distance: &mut Vec<Vec<usize>>,
    num_buildings_reached: &mut Vec<Vec<usize>>,
    x: usize,
    y: usize,
  ) {
    let rows = grid.len();
    let cols = grid[0].len();

    let mut visited = vec![vec![false; cols]; rows];
    /*
      - right, left, down, up
    */
    let neighbors = vec![(1, 0), (-1, 0), (0, 1), (0, -1)];
    let mut dist: usize = 0; // level in bfs
    let mut queue = VecDeque::from([(x, y)]);

    while queue.len() > 0 {
      /*
        - your neighbors are a step further
          away from you
      */
      dist = dist + 1;
      /*
        - use size to make sure we have
          visited all nodes at one level
          before moving on to the next
      */
      let size = queue.len();

      for _ in 0..size {
        if let Some((node_x, node_y)) = queue.pop_front() {
          for neighbor in &neighbors {
            let (x_move, y_move) = neighbor;
            let next_x_check = node_x as isize + x_move;
            let next_y_check = node_y as isize + y_move;

            /* make sure the next move is within bounds */
            if next_x_check >= 0
              && next_x_check < rows as isize
              && next_y_check >= 0
              && next_y_check < cols as isize
            {
              let next_x = next_x_check as usize;
              let next_y = next_y_check as usize;

              if grid[next_x][next_y] == 0 && !visited[next_x][next_y] {
                /* add the distance, which is the level in bfs */
                visited[next_x][next_y] = true;
                total_distance[next_x][next_y] += dist;
                /* record how many buildings have reached me so far */
                num_buildings_reached[next_x][next_y] += 1;
                queue.push_back((next_x, next_y));
              }
            }
          }
        }
      }
    }
  }

  pub fn test_fixture_1() -> Vec<Vec<usize>> {
    vec![
      vec![1, 0, 2, 0, 1],
      vec![0, 0, 0, 0, 0],
      vec![0, 0, 1, 0, 0],
    ]
  }

  pub fn test_fixture_2() -> Vec<Vec<usize>> {
    vec![vec![1, 0], vec![0, 0]]
  }
}
#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn sample_1() {
    let result = Solution::shortest_distance(Solution::test_fixture_1());
    assert_eq!(result, 7);
  }

  #[test]
  fn sample_2() {
    let result = Solution::shortest_distance(Solution::test_fixture_2());
    assert_eq!(result, 1);
  }
}
