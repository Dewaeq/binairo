pub const OPEN: u8 = 0;
pub const DIM: usize = 12;
pub const SIZE: usize = DIM * DIM;

pub type Field = [u8; SIZE];

pub struct Board {
    /// Square 0 is the left upper square, `squares[0]`
    pub squares: Field,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Square;
impl Board {
    pub fn new(squares: Field) -> Option<Board> {
        if !(Board { squares }).valid_board() {
            return None;
        }
        Some(Board { squares })
    }

    pub fn from_input(input: &str) -> Option<Board> {
        let list: Vec<&str> = input.split("").collect();
        assert!(list.len() == DIM, "Please provide a full length string");

        let mut data: Field = [OPEN; SIZE];
        for i in 0..SIZE {
            let value = list.get(i).unwrap();

            data[i] = value
                .parse::<u8>()
                .expect(&format!("Failed to parse {} to u8", value));
        }
        Board::new(data)
    }

    fn valid_board(&mut self) -> bool {
        let mut succes = true;
        for i in 0..SIZE {
            let value = self.get_square_value(i);
            if value == OPEN {
                continue;
            }
            if !self.valid_square_value(i, value) {
                let (x, y) = Square::get_coord(i);
                println!("Illegal value {} at x:{} y:{}", value, x + 1, y + 1);
                succes = false;
            }
        }
        succes
    }

    pub fn set_square_value(&mut self, index: usize, value: u8) {
        *self.squares.get_mut(index).unwrap() = value;
    }

    pub fn get_row(&self, y: usize) -> &[u8] {
        let start = y as usize * DIM;
        let stop = start + DIM;
        &self.squares[start..stop]
    }

    fn get_column(&self, x: usize) -> [u8; DIM] {
        let mut output: [u8; DIM] = [0; DIM];
        for i in 0..DIM {
            output[i] = self.squares[x + i * DIM]
        }
        output
    }

    pub fn get_square_value(&self, square: usize) -> u8 {
        *self.squares.get(square).unwrap()
    }

    pub fn valid_square_value(&mut self, index: usize, value: u8) -> bool {
        let old_value = self.get_square_value(index);
        self.set_square_value(index, value);

        let (x, y) = Square::get_coord(index);
        let fits_row = self.square_fits_in_row(x, y, value);
        let fits_column = self.value_fits_in_column(x, y, value);

        let row = self.get_row(y);
        let col = self.get_column(x);
        let fits_count = self.fits_count(&row, value) && self.fits_count(&col, value);

        self.set_square_value(index, old_value);

        fits_row && fits_column && fits_count
    }

    fn square_fits_in_row(&self, x: usize, y: usize, value: u8) -> bool {
        let mut count = 0;
        let row = self.get_row(y);

        let mut i = x + 1;
        while i < row.len() {
            if row[i] != value {
                break;
            }
            count += (row[i] == value) as usize;
            i += 1;
        }

        if x > 0 {
            i = x - 1;
            loop {
                if row[i] != value {
                    break;
                }
                count += (row[i] == value) as usize;
                if i == 0 {
                    break;
                }
                i -= 1;
            }
        }

        count < 2
    }

    fn value_fits_in_column(&self, x: usize, y: usize, value: u8) -> bool {
        let mut count = 0;
        let col = self.get_column(x);

        let mut i = y + 1;
        while i < col.len() {
            if col[i] != value {
                break;
            }
            count += (col[i] == value) as usize;
            i += 1;
        }

        if y > 0 {
            i = y - 1;
            loop {
                if col[i] != value {
                    break;
                }
                count += (col[i] == value) as usize;
                if i == 0 {
                    break;
                }
                i -= 1;
            }
        }

        count < 2
    }

    pub fn fits_count(&self, line: &[u8], value: u8) -> bool {
        value == OPEN || line.iter().filter(|&x| *x == value).count() <= 4
    }

    pub fn print_board(&self) {
        for i in 0..SIZE {
            let x = Square::get_coord(i).0;
            let value = self.get_square_value(i);
            let s = match value {
                1 => "O",
                2 => "I",
                3 => "X",
                _ => ".",
            };

            if x == DIM - 1 {
                println!(" {} ", s);
            } else {
                print!(" {} ", s);
            }
        }
    }
}

impl Square {
    fn get_coord(square: usize) -> (usize, usize) {
        let y = square / DIM;
        let x = square % DIM;
        (x, y)
    }
}
