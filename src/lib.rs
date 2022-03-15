use std::time::{Duration, Instant};

pub struct Chronometer {
    chrono: Option<Instant>,
    old: Option<Duration>,
    
    /// Vector of all the laps since last reset
    pub laps: Vec<Duration>,

    /// Has the chronometer started yet
    pub started: bool,

    /// Is the chronometer in pause mode
    pub paused: bool,
}

impl Chronometer {
    /// Create a new Chronometer
    /// 
    /// # Examples
    /// 
    /// Basic usage:
    /// 
    /// ```
    /// use chronometer::Chronometer;
    /// 
    /// let mut chrono = Chronometer::new();
    /// ```
    pub fn new() -> Chronometer {
        Chronometer {
            chrono: None,
            old: None,
            started: false,
            paused: false,
            laps: vec![],
        }
    }

    /// Start and "unpause" the chronometer
    /// 
    /// # Examples
    /// 
    /// Basic usage:
    /// 
    /// ```
    /// chrono.start();
    /// ```
    pub fn start(&mut self) {
        self.chrono = Some(Instant::now());
        self.started = true;
        self.paused = false;
    }

    /// Pause the chronometer
    /// 
    /// # Examples
    /// 
    /// Basic usage:
    /// 
    /// ```
    /// chrono.start();
    /// 
    /// let time = time::Duration::from_millis(100);
    /// 
    /// thread::sleep(time);
    /// chrono.pause(); // chrono.duration == Duration<0.1s>
    /// thread::sleep(time);
    /// 
    /// // chrono.duration == Duration<0.1s>
    /// 
    /// chrono.start();
    /// thread::sleep(time);
    /// // chrono.duration == Duration<0.2s>
    /// ```
    pub fn pause(&mut self) {
        self.old = match self.chrono {
            Some(chrono) => match self.old {
                Some(old) => Some(chrono.elapsed() + old),
                None => Some(chrono.elapsed()),
            },
            None => None,
        };
        self.chrono = None;
        self.paused = true;
    }

    /// Add a lap to the chronometer
    /// 
    /// # Examples
    /// 
    /// Basic usage:
    /// 
    /// ```
    /// chrono.start();
    /// 
    /// let time = time::Duration::from_millis(100);
    /// 
    /// thread::sleep(time);
    /// 
    /// chrono.lap(); // chono.laps == [Duration<0.1s>, Duration<0.2s>]
    /// 
    /// thread::sleep(time);
    /// 
    /// chrono.lap(); // chono.laps == [Duration<0.1s>, Duration<0.2s>]
    /// ```
    pub fn lap(&mut self) {
        if let Some(chrono) = self.chrono {
            self.laps.push(chrono.elapsed());
        }
    }

    /// Reset the chronometer, useful to restart everything and all the mesurements in your program
    /// 
    /// # Examples
    /// 
    /// Basic usage:
    /// 
    /// ```
    /// chrono.start();
    /// 
    /// let time = time::Duration::from_millis(100);
    /// 
    /// thread::sleep(time);
    /// 
    /// chrono.lap(); // chono.laps == [Duration<0.1s>, Duration<0.2s>]
    /// chrono.reset();
    /// // chono == {
    /// //  started: false,
    /// //  paused: false,
    /// //  laps: vec![],
    /// // }
    /// ```
    pub fn reset(&mut self) {
        self.chrono = None;
        self.paused = false;
        self.started = false;
        self.old = None;
        self.laps = vec![]
    }

    /// Gets you the elapsed time on your chronometer as a std::time::Duration
    /// 
    /// # Examples
    /// 
    /// Basic usage:
    /// 
    /// ```
    /// chrono.start();
    /// 
    /// let time = time::Duration::from_millis(100);
    /// 
    /// thread::sleep(time);
    /// 
    /// println!("elapsed: {}", chrono.duration().as_millis()); // elapsed: 100
    /// ```
    pub fn duration(&self) -> Option<Duration> {
        if self.started {
            if self.paused {
                self.old
            } else {
                match self.chrono {
                    Some(chrono) => match self.old {
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

/// Displays the current duration as milliseconds
impl std::fmt::Display for Chronometer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self.duration() {
            Some(duration) => write!(f, "{}", duration.as_millis()),
            None => write!(f, "<not started>"),
        }
    }
}

/// Displays all the public chronometer informations
impl std::fmt::Debug for Chronometer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "<Chronometer {{ started: {}, paused: {}, laps: {}, current value: {:?} }}>", self.started, self.paused, self.laps.len(), match self.duration() {
            Some(duration) => duration,
            None => std::time::Duration::new(0, 0),
        })
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

        assert_eq!(chrono.chrono, None);
        assert_eq!(chrono.laps, vec![]);

        assert_eq!(chrono.duration(), None);
    }

    #[test]
    fn start() {
        let mut chrono = Chronometer::new();

        chrono.start();

        assert_eq!(chrono.started, true);
        assert_eq!(chrono.paused, false);

        assert_ne!(chrono.chrono, None);

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