pub mod parser;
pub mod polyomino;

use wasm_bindgen::prelude::*;

use crate::polyomino::Game;
use algox::algox::Matrix;

#[wasm_bindgen]
pub struct JsGame {
    game: Game,

    #[wasm_bindgen(skip)]
    solutions: Vec<Vec<usize>>,

    #[wasm_bindgen(skip)]
    matrix: Matrix,
}

#[wasm_bindgen]
impl JsGame {
    #[wasm_bindgen]
    pub fn fromYaml(yaml: &str) -> Self {
        // populate from yaml
        JsGame {
            game: Game::from_yaml(yaml),
            solutions: Vec::new(),
            matrix: Matrix::new(0),
        }
    }

    #[wasm_bindgen]
    pub fn solve(&mut self) -> usize {
        // generates the solutions.
        self.matrix = self.game.build_matrix();
        self.solutions = self.matrix.solve();

        self.solutions.len()
    }

    #[wasm_bindgen]
    pub fn solution(&self, index: usize) -> Vec<usize> {
        // represent solution as string?
        let mut board: Vec<usize> = vec![0; self.game.len()];

        for r in &self.solutions[index] {
            let indices = self.matrix.row(*r);
            let tile_idx = indices.last().unwrap() - 1 - self.game.len();

            for i in 0..(indices.len() - 1) {
                board[indices[i] - 1] = tile_idx;
            }
        }
        board
    }

    #[wasm_bindgen]
    pub fn hint(&self) -> Vec<usize> {
        // find the tile placement with the most solutions
        
    }
}

#[cfg(test)]
mod test {
    use super::JsGame;

    #[test]
    fn basics() {
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
        let mut game = JsGame::fromYaml(yaml);

        let size = game.solve();
        assert_eq!(size, 68);

        let solution = game.solution(1);
    }
}