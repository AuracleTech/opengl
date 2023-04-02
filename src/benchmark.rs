use std::time::Instant;
use terminal_color_builder::OutputFormatter as tcb;

/*
let mut benchmark = Benchmark::start(&["EXAMPLE_ONE", "EXAMPLE_TWO"]);
benchmark.stop();
*/
pub struct Benchmark {
    pub text: String,
    pub start: Instant,
}

impl Benchmark {
    pub fn start(args: &[&str]) -> Self {
        let text = args.join(" ");
        Self {
            text,
            start: Instant::now(),
        }
    }

    pub fn stop(&mut self) {
        let duration = Instant::now().duration_since(self.start);
        let text = format!("Benchmark for {}", self.text);
        let prefix = tcb::new().fg().hex("#A351F9").text_str(&text).print();
        let duration_formatted = match duration.as_nanos() {
            0..=9999 => format!("{} ns", duration.as_nanos()),
            10_000..=9_999_999 => format!("{} µs", duration.as_micros()),
            10_000_000..=9_999_999_999 => format!("{} ms", duration.as_millis()),
            _ => format!("{} s", duration.as_secs()),
        };
        println!("{} {}", prefix, duration_formatted);
    }
}
