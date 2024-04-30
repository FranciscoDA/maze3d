#[derive(PartialEq, PartialOrd)]
pub enum GameEventType {
    GameStart { start_time: f64, duration: f64 },
    GameEnd { start_time: f64, duration: f64 },
}

impl GameEventType {
    pub fn start_time(&self) -> f64 {
        match self {
            Self::GameStart { start_time, .. } | Self::GameEnd { start_time, .. } => *start_time,
        }
    }

    pub fn end_time(&self) -> f64 {
        match self {
            Self::GameStart {
                start_time, duration, ..
            }
            | Self::GameEnd {
                start_time, duration, ..
            } => start_time + duration,
        }
    }

    pub fn duration(&self) -> f64 {
        self.end_time() - self.start_time()
    }
    pub fn elapsed_seconds(&self, clock: f64) -> f64 {
        self.duration().min((clock - self.start_time()).max(0.0))
    }
    pub fn elapsed_normalized(&self, clock: f64) -> f64 {
        self.elapsed_seconds(clock) / self.duration()
    }
    pub fn is_completed(&self, clock: f64) -> bool {
        self.end_time() <= clock
    }
}
