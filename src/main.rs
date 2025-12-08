use notify_rust::Notification;

fn main() {
    Notification::new()
        .appname("Hydration Notifier")
        .summary("Drink Water")
        .body("Don't forget to hydrate!")
        .show()
        .unwrap();
}
