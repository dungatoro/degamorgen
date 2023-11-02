pub trait Inputs {
    fn next_input(&mut self);
}

impl Inputs for Vec<bool> {
    fn next_input(&mut self) {
        for bit in self.iter_mut().rev() {
            if *bit == true {
                *bit = false;
            } else {
                *bit = true;
                return
            }
        }
        *self = Vec::new(); // empties if it overflows
    }
}

pub fn to_block_end(text: &Vec<char>, idx: &mut usize) {
    let mut num_open = 1;
    let mut num_closed = 0;
    while num_open != num_closed {
        *idx += 1;
        match text[*idx] {
            '(' => num_open += 1,
            ')' => num_closed += 1,
            _ => ()
        }
    }
}
