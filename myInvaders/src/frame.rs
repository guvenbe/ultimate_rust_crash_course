use crate::{MUM_COLS, MUM_ROWS};

pub type Frame = Vec<Vec<&'static str>>;

pub fn new_frame() -> Frame {
    let mut cols = Vec::with_capacity(MUM_COLS);
    for _ in 0..MUM_COLS{
        let mut col = Vec::with_capacity(MUM_ROWS);
        for _ in 0..MUM_ROWS{
            col.push(" ");
        }
        cols.push(col);
    }
    cols
}

pub trait Drawable {
    fn draw(&self, frame: &mut Frame);
}