#[derive(Debug)]
pub struct Position(pub u8, pub u8);

impl Position {
    pub fn get_file_letter(&self) -> char {
        let Position(x, _) = self;
        match x {
            0 => 'a',
            1 => 'b',
            2 => 'c',
            3 => 'd',
            4 => 'e',
            5 => 'f',
            6 => 'g',
            7 => 'h',
            _ => panic!("invalid position"),
        }
    }

    pub fn get_rank_letter(&self) -> char {
        // TODO can do better than that
        let Position(_, y) = self;
        match y {
            0 => '1',
            1 => '2',
            2 => '3',
            3 => '4',
            4 => '5',
            5 => '6',
            6 => '7',
            7 => '8',
            _ => panic!("invalid position"),
        }
    }

    pub fn get_file_rank(&self) -> String {
        let mut res = String::new();
        res.push(self.get_file_letter());
        res.push(self.get_rank_letter());
        return res;
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_get_file_rank() {
        let pos = Position(1, 2);
        let file_rank = pos.get_file_rank();
        assert_eq!(file_rank, "b3");
    }
}
