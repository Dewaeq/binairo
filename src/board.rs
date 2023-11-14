pub const OPEN: u8 = 255;

#[derive(Debug)]
pub struct Board {
    /// Square 0 is the left upper square, `squares[0]`
    pub squares: Vec<u8>,
    pub dim: usize,
    pub size: usize,
    pub options: Vec<char>,
    max_items: u8,
}

impl Board {
    pub fn new(squares: &str) -> Option<Board> {
        let list: Vec<char> = squares.replace("-", "").replace("\n", "").chars().collect();
        let size = list.len();
        let dim = f32::sqrt(size as f32).round() as usize;
        let options = extract_options(&list);
        let num_options = options.len();

        let mut squares = vec![OPEN; size];

        for i in 0..size {
            let value = list.get(i).unwrap();
            let key = if char::is_whitespace(*value) || *value == '.' {
                OPEN
            } else {
                options.iter().position(|x| x == value).unwrap() as u8
            };

            squares[i] = key;
        }

        let mut board = Board {
            squares,
            dim,
            size,
            options,
            max_items: (dim / num_options) as u8,
        };

        if !board.valid_board() {
            return None;
        }

        Some(board)
    }

    fn valid_board(&mut self) -> bool {
        let mut succes = true;

        if self.size != self.dim * self.dim {
            println!("Please submit a board with a valid size");
            succes = false;
        }

        for i in 0..self.size {
            let value = self.get_square_value(i);
            if value == OPEN {
                continue;
            }
            if !self.valid_square_value(i, value) {
                let (x, y) = get_coord(i, self.dim);
                println!(
                    "Illegal value {} at x:{} y:{}",
                    self.options[value as usize],
                    x + 1,
                    y + 1
                );
                succes = false;
            }
        }
        succes
    }

    pub fn set_square_value(&mut self, index: usize, value: u8) {
        *self.squares.get_mut(index).unwrap() = value;
    }

    pub fn get_row(&self, y: usize) -> &[u8] {
        let start = y as usize * self.dim;
        let stop = start + self.dim;

        &self.squares[start..stop]
    }

    fn get_column<'a>(&self, x: usize) -> Vec<u8> {
        let mut output = vec![OPEN; self.dim];
        for i in 0..self.dim {
            output[i] = self.squares[x + i * self.dim]
        }

        output
    }

    pub fn get_square_value(&self, square: usize) -> u8 {
        *self.squares.get(square).unwrap()
    }

    pub fn valid_square_value(&mut self, index: usize, value: u8) -> bool {
        let old_value = self.get_square_value(index);
        self.set_square_value(index, value);

        let (x, y) = get_coord(index, self.dim);
        let row = self.get_row(y);
        let col = self.get_column(x);
        let fits_count = self.fits_count(&row, value) && self.fits_count(&col, value);
        let fits_row = self.fits_line(&row, x, value);
        let fits_column = self.fits_line(&col, y, value);

        self.set_square_value(index, old_value);

        fits_row && fits_column && fits_count
    }

    fn fits_line(&self, line: &[u8], pos: usize, value: u8) -> bool {
        let mut count = 0;

        let mut i = pos + 1;
        while i < line.len() {
            if line[i] != value {
                break;
            }
            count += (line[i] == value) as usize;
            i += 1;
        }

        if pos > 0 {
            i = pos - 1;
            loop {
                if line[i] != value {
                    break;
                }
                count += (line[i] == value) as usize;
                if i == 0 {
                    break;
                }
                i -= 1;
            }
        }

        count < 2
    }

    fn fits_count(&self, line: &[u8], value: u8) -> bool {
        value == OPEN || line.iter().filter(|&x| *x == value).count() <= self.max_items as usize
    }

    pub fn print_board(&self) {
        for i in 0..self.size {
            let x = get_coord(i, self.dim).0;
            let value = self.get_square_value(i);
            let s = if value == OPEN {
                ".".to_owned()
            } else {
                self.options[value as usize].to_string()
            };

            if x == self.dim - 1 {
                println!(" {} ", s);
            } else {
                print!(" {} ", s);
            }
        }
    }
}

fn get_coord(square: usize, dim: usize) -> (usize, usize) {
    let y = square / dim;
    let x = square % dim;
    (x, y)
}

fn extract_options(squares: &Vec<char>) -> Vec<char> {
    let mut options = vec![];

    for &c in squares {
        if !char::is_whitespace(c) && c != '.' && !options.contains(&c) {
            options.push(c);
        }
    }

    options
}
