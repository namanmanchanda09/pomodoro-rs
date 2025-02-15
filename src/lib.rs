use rodio::source::{SineWave, Source};
use rodio::{OutputStream, Sink};
use std::time::Duration;
use std::{
    io::{self, Write},
    thread::sleep,
    time::SystemTime,
};

pub fn start_timer(countdown: u64) {
    let start = SystemTime::now();
    let countdown = Duration::from_secs(countdown);

    let mut count = countdown.as_secs();

    loop {
        let count_to_str = seconds_to_time(count);
        print!("\r{} ", count_to_str);

        io::stdout().flush().unwrap();
        sleep(Duration::new(1, 0));
        if start.elapsed().unwrap() > countdown {
            break;
        }
        count -= 1;
    }

    play_audio();
}

pub fn seconds_to_time(total_seconds: u64) -> String {
    let minutes = (total_seconds % 3600) / 60;
    let seconds = total_seconds % 60;

    format!("{:02}:{:02}", minutes, seconds)
}

fn play_audio() {
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let sink = Sink::try_new(&stream_handle).unwrap();

    let source = SineWave::new(440.0)
        .take_duration(Duration::from_secs_f32(3.0))
        .amplify(1.0);
    sink.append(source);

    sink.sleep_until_end();
}
