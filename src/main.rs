use rppal::gpio::{Gpio, Trigger};
use std::error::Error;
// use std::sync::atomic::Ordering;
// use std::sync::atomic::{AtomicBool, Ordering};
// use std::sync::Arc;
// use std::thread;
use std::time::{Duration, Instant};

use std::fs::File;
use std::io::{BufWriter, Write};

const SENSOR_PIN: u8 = 23;

#[derive(Debug)]
enum SignalFormat {
    NEC,
    AEHA,
    SONY,
}

fn analyze(data: &Vec<u32>) -> Vec<u32> {
    let fmt = if data[0] < 2600 {
        SignalFormat::SONY
    } else if data[0] < 6000 {
        SignalFormat::AEHA
    } else {
        SignalFormat::NEC
    };

    let leader_len = match fmt {
        SignalFormat::NEC => 16.0,
        SignalFormat::AEHA => 8.0,
        SignalFormat::SONY => 4.0,
    };

    let tick = data[0] as f64 / leader_len;
    let x = data[data.len() - 1];

    let data: Vec<_> = data
        .iter()
        .map(|&x| (x as f64 / tick).round() as u32)
        .collect();

    // for (&high, &low) in data.iter().step_by(2).zip(data.iter().skip(1).step_by(2)) {
    //     if high > 0 {
    //         // other
    //     } else {
    //         // data
    //     }
    // }

    println!("Format: {:?}", fmt);
    println!("Tick: {} (us)", tick);

    data
}

fn main() -> Result<(), Box<dyn Error>> {
    // init peripheral
    // let mut led = Gpio::new()?.get(LED_PIN)?.into_output();
    let mut sensor = Gpio::new()?.get(SENSOR_PIN)?.into_input();

    // let dur = Duration::from_millis(50);

    sensor.set_interrupt(Trigger::FallingEdge)?;
    let mut raw = Vec::with_capacity(1000);
    let mut prev = sensor.poll_interrupt(false, None)?.unwrap();
    let mut tick = Instant::now();
    println!("# start to read.");

    loop {
        let val = sensor.read();
        if val != prev {
            let time = tick.elapsed();
            raw.push(time.subsec_micros());
            tick = Instant::now();
            prev = val;
        } else if tick.elapsed() > Duration::from_secs(1) {
            let time = tick.elapsed();
            let s = (time.as_secs() * 1_000_000) as u32 + time.subsec_micros();
            raw.push(s);
            break;
        }
    }

    let data = analyze(&raw);

    let mut file = BufWriter::new(File::create("raw.txt")?);
    for r in raw {
        // writer.write_all(&r.to_string());
        writeln!(file, "{}", r)?;
    }
    file.flush()?;

    let mut file = BufWriter::new(File::create("tick.txt")?);
    for x in data {
        // writer.write_all(&r.to_string());
        writeln!(file, "{}", x)?;
    }
    file.flush()?;

    Ok(())
}
