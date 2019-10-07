
pub fn err_log(err_info:&str){
//    fern::Dispatch::new()
//        // Perform allocation-free log formatting
//        .format(|out, message, record| {
//            out.finish(format_args!(
//                "{}[{}][{}] {}",
//                chrono::Local::now().format("[%Y-%m-%d][%H:%M:%S]"),
//                record.target(),
//                record.level(),
//                message
//            ))
//        })
//        // Add blanket level filter -
//        .level(log::LevelFilter::Debug)
//        // - and per-module overrides
//        .level_for("hyper", log::LevelFilter::Info)
//        // Output to stdout, files, and other Dispatch configurations
//        .chain(std::io::stdout())
//        .chain(fern::log_file("error.log").unwrap())
//        // Apply globally
//        .apply().unwrap();
//
//// and log using log crate macros!
////    info!("helllo, world!");
////    debug!("debug test");
//    error!("{}",err_info);



    match fern::log_file("error.log") {
        Ok(log_file) => {
            match fern::Dispatch::new()
                // Perform allocation-free log formatting
                .format(|out, message, record| {
                    out.finish(format_args!(
                        "{}[{}][{}] {}",
                        chrono::Local::now().format("[%Y-%m-%d][%H:%M:%S]"),
                        record.target(),
                        record.level(),
                        message
                    ))
                })
                // Add blanket level filter -
                .level(log::LevelFilter::Debug)
                // - and per-module overrides
                .level_for("hyper", log::LevelFilter::Info)
                // Output to stdout, files, and other Dispatch configurations
                .chain(std::io::stdout())
                .chain(log_file)
                // Apply globally
                .apply() {
                Ok(f) => {
                    error!("{}",err_info);
                }
                Err(e) => {
                    println!("{}:{:?}","写错误日志文件内容失败",e);
                }
            }
        }
        Err(e) => {
            println!("{}:{:?}","加载错误日志文件失败",e);
        }
    }



}