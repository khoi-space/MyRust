// Day 8: Logging

use log::{error, warn, info, debug, trace};

fn main() {
    env_logger::init();

    error!("Database crashed!");
    warn!("Alert: RAM overhead 80%");
    info!("User 'admin' has been logged in");
    debug!("x=5");
    trace!("Receive 12 data bytes from IP...");
}