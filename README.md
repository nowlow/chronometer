# Chronometer

Stopwatch lib for rust. Start, pause, reset and lap like any stopwatch. Nothing special I'm just learning rust.

## Getting Started

This crate hasn't been uploaded on cargo yet, so clone this repo and make it work, you're probably better at rust than me. Like any cargo project, it has the default architecture.

## Features

### Methods

#### `::new`
Create a new chronometer.  

#### `.start()`
Starts the chonometer.

#### `.pause()`
Pause the chronometer.

#### `.reset()`
Reset everything about the chronometer.

#### `.lap()`
Add a new lap to the `laps`.

#### `.duration() -> std::time::Duration`
Gets you the time that the chronometer has computed.  
Might find a better name for that one later.  

### Fields

#### `started: bool`
Has the chronometer started yet?

#### `paused: bool`
Is the chronometer currently in pause state?

#### `laps: Vec<Duration>`
All the durations in which the chronometer has marked a lap.

## Example

```rs
use device_query::{DeviceQuery, DeviceState, Keycode};
use chronometer::Chronometer;

fn main() {
    let device_state = DeviceState::new();
    let mut key_repeat = false;
    let mut space_count = 0;
    let mut chrono: Chronometer = Chronometer::new();

    loop {
        let keys: Vec<Keycode> = device_state.get_keys();
        
        println!("Chrono: {}", chrono);
        if keys.len() == 0 {
            key_repeat = false;
        }
        for key in keys.iter() {
            if *key == Keycode::Space && !key_repeat {
                space_count += 1;
                match space_count % 2 {
                    1 => chrono.start(),
                    _ => chrono.pause(),
                };
            }

            if *key == Keycode::R {
                chrono.reset()
            }

            if *key == Keycode::R {
                chrono.reset()
            }

            match *key {
                Keycode::Space => key_repeat = true,
                _ => key_repeat = false,
            }
        }
    }
}
```

## Getting help
I don't think anyone would ever want to use this crate but if you do and you find bug, before internally insulting me, please fill an issue or send me a message on [twitter](https://twitter.com/nowlow_).

___  
Love and potatoes, nowlow 🙌