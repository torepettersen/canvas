pub trait IterExt: Iterator {
    fn skip_nth(self, skip: usize) -> SkipNth<Self>
    where
        Self: Sized,
    {
        SkipNth { iter: self, skip: Some(skip) }
    }
}

impl<I: Iterator> IterExt for I {}

pub struct SkipNth<I> {
    iter: I,
    skip: Option<usize>,
}

impl<I: Iterator> Iterator for SkipNth<I> {
    type Item = I::Item;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(skip) = self.skip {
            if skip == 0 {
                self.skip = None;
                self.iter.next()?;
            } else {
                self.skip = Some(skip - 1);
            }
        }
        self.iter.next()
    }
}

#[test]
fn skips_nth() {
    let a = [7, 8, 9];

    let mut iter = a.iter().skip_nth(1);

    assert_eq!(iter.next(), Some(&7));
    assert_eq!(iter.next(), Some(&9));
    assert_eq!(iter.next(), None);
}

