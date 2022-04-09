use std::marker::PhantomData;
use crate::constants::app_env::AppEnv;

#[non_exhaustive]
#[derive(Debug)]
pub struct FilePaths<'a> {
    /// https://stackoverflow.com/questions/40484154/parameter-a-is-never-used-error-when-a-is-used-in-type-parameter-bound
    // Causes the type to function *as though* it has a `&'a ()` field,
    // despite not *actually* having one.
    _marker: PhantomData<&'a ()>,
}

impl FilePaths<'static> {
    pub const CONFIG: &'static str = "./config.yaml";
    pub const LOG: &'static str = if AppEnv::IS_RELEASE {
        "./sirius-hydra-logs/logging-release.log"
    } else {
        "./sirius-hydra-logs/logging-debug.log"
    };
}
