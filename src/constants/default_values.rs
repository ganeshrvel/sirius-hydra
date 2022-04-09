use crate::constants::app_env::AppEnv;
use std::marker::PhantomData;

#[derive(Debug)]
pub struct DefaultValues<'a> {
    /// https://stackoverflow.com/questions/40484154/parameter-a-is-never-used-error-when-a-is-used-in-type-parameter-bound
    // Causes the type to function *as though* it has a `&'a ()` field,
    // despite not *actually* having one.
    _marker: PhantomData<&'a ()>,
}

impl DefaultValues<'static> {
    pub const RESTART_BTN_PRESS_TIME_MS: i64 = 4000_i64;
    pub const FILE_LOGGING_LEVEL: log::LevelFilter = if AppEnv::IS_RELEASE {
        log::LevelFilter::Error
    } else {
        log::LevelFilter::Debug
    };
    pub const STDOUT_LOGGING_LEVEL: log::LevelFilter = if AppEnv::IS_RELEASE {
        log::LevelFilter::Debug
    } else {
        log::LevelFilter::Debug
    };
}
