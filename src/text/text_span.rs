#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct TextSpan(usize, usize);

impl TextSpan {
    pub fn new(start: usize, end: usize) -> TextSpan { TextSpan(start, end) }
    pub fn start(&self) -> usize { self.0 }
    pub fn end(&self) -> usize { self.1 }
}