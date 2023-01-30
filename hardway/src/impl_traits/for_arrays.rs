trait Size{
    fn size(&self) -> usize;
}

impl<const N: usize> Size for [i32; N] {
    fn size(&self) -> usize {
        todo!()
    }
}