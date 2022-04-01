use wasm_bindgen::prelude::*;

mod image;
mod uq;

// When the `wee_alloc` feature is enabled, this uses `wee_alloc` as the global
// allocator.
//
// If you don't want to use `wee_alloc`, you can safely delete this.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub struct InternalState {
    undo_queue: uq::UndoQueue<image::Image>,
}

#[wasm_bindgen]
impl InternalState {
    #[wasm_bindgen(constructor)]
    pub fn new(width: usize, height: usize) -> InternalState {
        InternalState {
            undo_queue: uq::UndoQueue::new(image::Image::new(width, height)),
        }
    }

    pub fn image(&self) -> image::Image {
        self.undo_queue.current()
    }

    pub fn undo(&mut self) {
        self.undo_queue.undo();
    }

    pub fn redo(&mut self) {
        self.undo_queue.redo();
    }

    pub fn start_undo_block(&mut self) {
        self.undo_queue.start_undo_block();
    }

    pub fn close_undo_block(&mut self) {
        self.undo_queue.close_undo_block();
    }

    pub fn brush(&mut self, x: usize, y: usize, color: Vec<u8>) {
        let image = self.undo_queue.current();
        let new_image = image.brush(x, y, color);
        match new_image {
            None => (),
            Some(new_image) => self.undo_queue.push(new_image),
        }
    }
}
