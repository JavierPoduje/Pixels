enum Mode {
    Normal,
    StartBlock,
    InBlock,
}

pub struct UndoQueue<T: Clone> {
    queue: Vec<T>,
    index: usize,
    mode: Mode,
}

impl<T: Clone> UndoQueue<T> {
    pub fn new(entry: T) -> UndoQueue<T> {
        UndoQueue {
            queue: vec![entry],
            index: 0,
            mode: Mode::Normal,
        }
    }

    pub fn current(&self) -> T {
        self.queue[self.index].clone()
    }

    pub fn start_undo_block(&mut self) {
        self.mode = Mode::StartBlock;
    }

    pub fn close_undo_block(&mut self) {
        self.mode = Mode::Normal;
    }

    pub fn push(&mut self, entry: T) {
        match self.mode {
            Mode::Normal => {
                self.queue.truncate(self.index + 1);
                self.queue.push(entry);
                self.index += 1;
            }
            Mode::StartBlock => {
                self.queue.truncate(self.index + 1);
                self.queue.push(entry);
                self.index += 1;
                self.mode = Mode::InBlock;
            }
            Mode::InBlock => {
                self.queue[self.index] = entry;
            }
        }
    }

    pub fn undo(&mut self) {
        if self.index >= 1 {
            self.index -= 1;
        }
    }

    pub fn redo(&mut self) {
        if self.index < self.queue.len() - 1 {
            self.index += 1;
        }
    }
}
