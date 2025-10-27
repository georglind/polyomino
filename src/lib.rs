pub mod parser;
pub mod polyomino;

use wasm_bindgen::prelude::*;

use crate::polyomino::Game;

#[wasm_bindgen]
pub struct JsGame {
    game: Game,

    #[wasm_bindgen(skip)]
    solutions: Vec<Vec<usize>>,
}

#[wasm_bindgen]
impl JsGame {
    #[wasm_bindgen]
    pub fn fromYaml(yaml: &str) -> Self {
        // populate from yaml
        JsGame {
            game: Game::from_yaml(yaml),
            solutions: Vec::new(),
        }
    }

    #[wasm_bindgen]
    pub fn solve(&mut self) -> usize {
        // generates the solutions.
        let solution = self.game.solve();

        if let Some(solution) = solution {
            self.solutions.push(solution);
        }

        self.solutions.len()
    }

    #[wasm_bindgen]
    pub fn solveAll(&mut self) -> usize {
        // solve at least once.
        self.solve();

        let mut previous_count: usize = 0;
        loop {
            let count = self.solve();
            if count == previous_count {
                break;
            }
            previous_count = count;
        }
        self.solutions.len()
    }

    #[wasm_bindgen]
    pub fn solution(&self, index: usize) -> Vec<usize> {
        // represent solution as string?
        let mut board: Vec<usize> = vec![0; self.game.len()];

        for r in &self.solutions[index] {
            let indices = self.game.row(*r);
            let tile_idx = indices.last().unwrap() - 1 - self.game.len();

            for i in 0..(indices.len() - 1) {
                board[indices[i] - 1] = tile_idx;
            }
        }
        board
    }

    // #[wasm_bindgen]
    // pub fn hint(&self) -> Vec<usize> {
    //     // find the tile placement with the most solutions
    // }
}

#[cfg(test)]
mod test {
    use super::JsGame;

    fn setup() -> JsGame {
        let yaml = "
---
Board: |
    xxxxxx
     xxxxx
    xxxxxxx
    xxxxxxx
    xxxxxxx
    xxxxx x
    xxx
0: |
    xxx
      xx
1: |
    xxxx
    x
2: |
    xxx
    x x
3: |
    xxx
    xx
4: |
    xxx
    xxx
5: |
    xxxx
     x
6: |
    x
    xxx
      x
7: |
      x
      x
    xxx
";
        JsGame::fromYaml(yaml)
    }

    #[test]
    fn basics() {
        let mut game = setup();

        for i in 1..68 {
            assert_eq!(game.solve(), i);
            assert_eq!(
                game.solution(0),
                [
                    0, 0, 1, 1, 1, 1, 0, 0, 0, 6, 1, 3, 3, 3, 5, 6, 6, 6, 3, 3, 5, 5, 5, 5, 6, 7,
                    4, 4, 4, 2, 2, 2, 7, 4, 4, 4, 2, 2, 7, 7, 7
                ]
            );
        }
        assert_eq!(game.solve(), 68);
        assert_eq!(game.solve(), 68);

        assert_eq!(game.solveAll(), 68)
    }
}
