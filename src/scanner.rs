use crate::{Error, Token};

pub trait TokenScanner<'a, I>
where
    I: Iterator<Item = char> + Clone,
{
    fn tokens(&self, error: &'a mut Error) -> Scanner<'a, I>;
}

impl<'a, I> TokenScanner<'a, I> for I
where
    I: Iterator<Item = char> + Clone,
{
    fn tokens(&self, error: &'a mut Error) -> Scanner<'a, I> {
        Scanner::new(self.clone(), error)
    }
}

pub struct Scanner<'a, I>
where
    I: Iterator<Item = char> + Clone,
{
    source: I,
    error: &'a mut Error,
}

impl<I> Iterator for Scanner<'_, I>
where
    I: Iterator<Item = char> + Clone,
{
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        None
    }
}

impl<'a, I> Scanner<'a, I>
where
    I: Iterator<Item = char> + Clone,
{
    pub fn new(source: I, error: &'a mut Error) -> Self {
        Self { source, error }
    }
}
