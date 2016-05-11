
pub struct SetState {
    time: usize,
    max_time: usize,
    state: bool,
}

impl SetState {
    pub fn new(max_time: usize) -> SetState {
        SetState {
            time: 0,
            max_time: max_time,
            state: true,
        }
    }

    pub fn get_state(&self) -> bool {
        self.state
    }

    pub fn update_time(&mut self) {
        self.time += 1;
    }
    pub fn is_changing(&mut self, boolean: bool) {
        if self.state {
            if !boolean {
                if self.time >= self.max_time {
                    self.state = false;
                }
            }
        } else {
            if boolean {
                self.state = true;
                self.time = 0;
            }
        }
    }
}
