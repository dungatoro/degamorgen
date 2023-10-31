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
