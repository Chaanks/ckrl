use std::io::Write;


pub fn start_logger() {
    env_logger::Builder::new()
        .format(|buf, record| {
            writeln!(buf,
                "{} [{}] - {}",
                chrono::Local::now().format("%Y-%m-%dT%H:%M:%S"),
                record.level(),
                record.args()
            )
        })
        .filter(None, log::LevelFilter::Debug)
        .init();
}


#[cfg(test)]
mod tests { 

    use super::start_logger;
    use log::{debug, info, warn, error};

    #[test]
    fn make_logger() {
        start_logger();

        warn!("warn");
        info!("info");
        debug!("debug");
    
        info!("such information");
        info!("such information");
        warn!("o_O");
        warn!("o_O");
        error!("boom");
        error!("boom");
        debug!("deboogging");
        debug!("deboogging");
    }

}
