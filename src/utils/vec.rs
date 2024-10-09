pub fn is_in<T: Eq + PartialOrd>(v: &[T], i: &T) -> bool {
    v.contains(i)
}
