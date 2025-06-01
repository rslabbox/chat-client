use log4rs::{
    append::console::ConsoleAppender,
    config::{Appender, Config, Root},
    encode::pattern::PatternEncoder,
};
use log::LevelFilter;

/// 初始化 log4rs 日志系统
/// 格式: [时:分:秒 文件名:行数] 具体日志
/// 不同级别使用不同颜色
pub fn init_logging() -> Result<(), Box<dyn std::error::Error>> {
    // 创建控制台输出器，带颜色
    let console = ConsoleAppender::builder()
        .encoder(Box::new(PatternEncoder::new(
            "[{d(%H:%M:%S)} {f}:{L}] {h({l} {m}{n})} "
        )))
        .build();

    // 创建配置
    let config = Config::builder()
        .appender(Appender::builder().build("console", Box::new(console)))
        .build(Root::builder().appender("console").build(LevelFilter::Info))?;

    // 初始化日志系统
    log4rs::init_config(config)?;

    Ok(())
}
