# Chronometer

Stopwatch lib for rust. Start, pause, reset and lap like any stopwatch. Nothing special I'm just learning rust.

## Getting Started

Add this line to your `Cargo.toml`
```
chronometer = "0.1.2"
```

> I'm not that well organized to update this readme evrytime I change version, so just to be sure, check the latest release [here](https://crates.io/crates/chronometer)  

## Docs

Full docs [here](https://docs.rs/chronometer/0.1.1/chronometer/)

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

## License
[MIT](LICENSE)
___  
Love and potatoes, nowlow 🙌
