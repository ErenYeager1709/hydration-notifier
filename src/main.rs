use std::{thread, time::Duration};

use clap::{Parser, arg, command};
use notify_rust::Notification;
use rand::Rng;

const MESSAGES: [&str; 20] = [
    "Don't forget to hydrate!",
    "Time for a sip!",
    "Water break!",
    "Stay hydrated, legend.",
    "Your body called — it wants water.",
    "Hydration checkpoint!",
    "Drink up, stay sharp.",
    "Refill your tank!",
    "H₂Oh yeah, drink some water.",
    "Take a gulp, not a guess.",
    "Fuel = water. Refill now!",
    "Keep the plants alive? Start with you.",
    "Level up your hydration stat.",
    "Your future self says thanks for the water.",
    "Don’t dry out now!",
    "Stay moist (in a healthy way).",
    "Water: the original energy drink.",
    "Be like a cactus… wait, no. Drink!",
    "Hydration: it’s what champions do.",
    "Even your code runs smoother with water.",
];

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value_t = 60 * 60)]
    interval_seconds: u64,

    #[arg(short, long, default_value_t = 10)]
    duration_seconds: u64,
}

fn main() {
    let args = Args::parse();
    let mut rng = rand::rng();

    loop {
        if let Err(e) = Notification::new()
            .appname("Hydration Notifier")
            .summary("Drink Water")
            .timeout(Duration::from_secs(args.duration_seconds))
            .body(MESSAGES[rng.random_range(0..MESSAGES.len())])
            .show()
        {
            eprintln!("Failed to show notification: {}", e);
        }

        thread::sleep(Duration::from_secs(args.interval_seconds));
    }
}
