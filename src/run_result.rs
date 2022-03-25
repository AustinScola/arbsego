pub struct RunResult {
    pub errors: u32,
}

impl RunResult {
    pub fn new(errors: u32) -> Self {
        Self { errors }
    }
}
