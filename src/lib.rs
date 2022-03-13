use std::time::{Duration, Instant};

pub struct Chronometer {
    _chrono: Option<Instant>,
    _old: Option<Duration>,
    pub laps: Vec<Duration>,
    pub started: bool,
    pub paused: bool,
}

impl Chronometer {
    pub fn new() -> Chronometer {
        Chronometer {
            _chrono: None,
            _old: None,
            started: false,
            paused: false,
            laps: vec![],
        }
    }

    pub fn start(&mut self) {
        self._chrono = Some(Instant::now());
        self.started = true;
        self.paused = false;
    }

    pub fn pause(&mut self) {
        self._old = match self._chrono {
            Some(chrono) => match self._old {
                Some(old) => Some(chrono.elapsed() + old),
                None => Some(chrono.elapsed()),
            },
            None => None,
        };
        self._chrono = None;
        self.paused = true;
    }

    pub fn lap(&mut self) {
        if let Some(chrono) = self._chrono {
            self.laps.push(chrono.elapsed());
        }
    }

    pub fn reset(&mut self) {
        self._chrono = None;
        self.paused = false;
        self.started = false;
        self._old = None;
        self.laps = vec![]
    }

    pub fn duration(&self) -> Option<Duration> {
        if self.started {
            if self.paused {
                self._old
            } else {
                match self._chrono {
                    Some(chrono) => match self._old {
                        Some(old) => Some(chrono.elapsed() + old),
                        None => Some(chrono.elapsed()),
                    },
                    None => None,
                }
            }
        } else {
            None
        }
    }
}

impl std::fmt::Display for Chronometer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self.duration() {
            Some(duration) => write!(f, "{}", duration.as_millis()),
            None => write!(f, "<not started>"),
        }
    }
}


#[cfg(test)]

mod test {
    use super::Chronometer;

    #[test]
    fn create() {
        let chrono = Chronometer::new();

        assert_eq!(chrono.started, false);
        assert_eq!(chrono.paused, false);

        assert_eq!(chrono._chrono, None);
        assert_eq!(chrono.laps, vec![]);

        assert_eq!(chrono.duration(), None);
    }

    #[test]
    fn start() {
        let mut chrono = Chronometer::new();

        chrono.start();

        assert_eq!(chrono.started, true);
        assert_eq!(chrono.paused, false);

        assert_ne!(chrono._chrono, None);

        std::thread::sleep(std::time::Duration::from_millis(100));

        match chrono.duration() {
            Some(duration) => assert_eq!(duration.as_millis() > 0, true),
            None => assert_eq!(true, false),
        }
    }

    #[test]
    fn pause() {
        let mut chrono = Chronometer::new();

        chrono.start();

        assert_eq!(chrono.started, true);
        assert_eq!(chrono.paused, false);

        let time_before_pause = match chrono.duration() {
            Some(duration) => duration.as_millis(),
            None => 10000,
        };

        chrono.pause();

        assert_eq!(chrono.started, true);
        assert_eq!(chrono.paused, true);

        let time_after_pause = match chrono.duration() {
            Some(duration) => duration.as_millis(),
            None => 10000,
        };

        assert_eq!(time_before_pause, time_after_pause);
    }
}