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
                break
            }
        }
    }
}

pub trait TruthTable {
    fn as_bit(&self) -> char;
}

impl TruthTable for bool {
    fn as_bit(&self) -> char {
        if *self { '1' }
        else     { '0' }
    }

}
