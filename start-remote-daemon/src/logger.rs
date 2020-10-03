use log::LevelFilter;
use log4rs::append::file::FileAppender;
use log4rs::config::{Appender, Config, Logger, Root};
use log4rs::encode::pattern::PatternEncoder;
use log4rs::Handle;
use std::env;
use std::path::Path;

pub fn init_logger() -> Handle {
    let home = env::var("HOME").unwrap();
    let logfile = Path::new(&home).join("remote-demo.log");
    let file = FileAppender::builder()
        .encoder(Box::new(PatternEncoder::new("{d} - {m}{n}")))
        .build(logfile)
        .unwrap();

    let config = Config::builder()
        .appender(Appender::builder().build("file", Box::new(file)))
        .logger(
            Logger::builder()
                .appender("file")
                .additive(false)
                .build("remote-demo", LevelFilter::Debug),
        )
        .build(Root::builder().appender("file").build(LevelFilter::Debug))
        .unwrap();

    log4rs::init_config(config).unwrap()
}
