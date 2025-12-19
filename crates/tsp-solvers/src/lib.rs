pub mod held_karp;
pub(crate) mod utils;

pub(crate) type CustomBitVec = bitvec::vec::BitVec<usize, bitvec::order::Lsb0>;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
