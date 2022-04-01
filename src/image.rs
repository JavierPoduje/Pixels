use im::Vector;
use std::iter::FromIterator;
use wasm_bindgen::prelude::*;

#[derive(Clone, Copy, PartialEq)]
struct Rgb {
    r: u8,
    g: u8,
    b: u8,
}

#[derive(Clone)]
#[wasm_bindgen]
pub struct Image {
    width: usize,
    height: usize,
    cells: Vector<Rgb>,
}

#[wasm_bindgen]
impl Image {
    #[wasm_bindgen(constructor)]
    pub fn new(width: usize, height: usize) -> Image {
        let cells = Vector::from_iter((0..width * height).map(|_| Rgb {
            r: 200,
            g: 200,
            b: 255,
        }));

        Image {
            width,
            height,
            cells,
        }
    }

    pub fn cells(&self) -> Vec<u8> {
        self.cells
            .iter()
            .map(|rgb| vec![rgb.r, rgb.g, rgb.b])
            .collect::<Vec<Vec<u8>>>()
            .concat()
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn brush(&self, x: usize, y: usize, color: Vec<u8>) -> Option<Image> {
        let index = (y * self.width) + x;
        let color = Rgb {
            r: color[0],
            g: color[1],
            b: color[2],
        };

        if self.cells[index] == color {
            None
        } else {
            let new_cells = self.cells.update(index, color);

            Some(Image {
                width: self.width,
                height: self.height,
                cells: new_cells,
            })
        }
    }
}
