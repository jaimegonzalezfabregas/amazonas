#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn is_valid_test_3() {
        let mut test_board = Board::parse("ATA|TT |A  ");
        assert_eq!(test_board.is_valid(), true);
    }
    #[test]
    fn is_valid_test_4() {
        let mut test_board = Board::parse("ATA | T  |    | ATA");
        assert_eq!(test_board.is_valid(), true);
    }
}
