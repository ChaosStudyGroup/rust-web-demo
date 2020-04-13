use log4rs::append::console::{ConsoleAppender, Target};
use log4rs::encode::json::JsonEncoder;
use log4rs::config::{Config, Appender, Root};
use log4rs::filter::threshold::ThresholdFilter;
use log4rs::append::rolling_file::RollingFileAppender;
use log4rs::append::rolling_file::policy::compound::CompoundPolicy;
use log4rs::append::rolling_file::policy::compound::trigger::size::SizeTrigger;
use log4rs::append::rolling_file::policy::compound::roll::fixed_window::FixedWindowRoller;

pub fn init_log() {
    // 当前日志文件
    let logfile = &crate::conf::global().server.log;
    // 备份日志文件
    let bakfile = &format!("{}.{{}}", logfile);

    // 命令行输出日志
    let stdout = ConsoleAppender::builder().target(Target::Stdout).build();

    // 写入日志到文件，按尺寸分割
    let size_trigger = SizeTrigger::new(1024 * 1024 * 500);
    let roller = FixedWindowRoller::builder().build(bakfile, 3).unwrap();
    let policy = CompoundPolicy::new(Box::new(size_trigger), Box::new(roller));

    let logfile = RollingFileAppender::builder()
        .encoder(Box::new(JsonEncoder::new()))
        .build(logfile, Box::new(policy)).unwrap();

    // 记录警告级别以上的日志
    let stdout_appender = Appender::builder()
        .filter(Box::new(ThresholdFilter::new(log::LevelFilter::Warn)))
        .build("stderr", Box::new(stdout));

    let logfile_appender = Appender::builder().build("logfile", Box::new(logfile));

    let root = Root::builder().appender("logfile").appender("stdout").build(log::LevelFilter::Info);

    let config = Config::builder().appender(stdout_appender).appender(logfile_appender).build(root);

    let _handle = log4rs::init_config(config.unwrap()).unwrap();
}