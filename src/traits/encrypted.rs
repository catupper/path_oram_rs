pub trait Encrypted: PartialEq + Eq + Copy {
    fn dummy() -> Self;
}
