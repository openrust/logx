#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

#[doc(hidden)]
pub enum Level {
    None = 0,
    Error = 1,
    Warn = 2,
    Info = 3,
    Debug = 4,
    Trace = 5,
}

#[doc(hidden)]
pub fn level() -> Level {
    match () {
        #[cfg(debug_assertions)]
        () => {
            #[cfg(feature = "level-none")]
                return Level::None;

            #[cfg(feature = "level-error")]
                return Level::Error;

            #[cfg(feature = "level-warn")]
                return Level::Warn;

            #[cfg(feature = "level-info")]
                return Level::Info;

            #[cfg(feature = "level-debug")]
                return Level::Debug;

            #[cfg(feature = "level-trace")]
                return Level::Trace;

            Level::Debug
        }
        #[cfg(not(debug_assertions))]
        () => {
            #[cfg(feature = "release-level-none")]
                return Level::None;

            #[cfg(feature = "release-level-error")]
                return Level::Error;

            #[cfg(feature = "release-level-warn")]
                return Level::Warn;

            #[cfg(feature = "release-level-info")]
                return Level::Info;

            #[cfg(feature = "release-level-debug")]
                return Level::Debug;

            #[cfg(feature = "release-level-trace")]
                return Level::Trace;

            Level::Info
        }
    }
}