use std::marker::PhantomData;

#[non_exhaustive]
#[derive(Debug)]
pub struct Strings<'a> {
    /// https://stackoverflow.com/questions/40484154/parameter-a-is-never-used-error-when-a-is-used-in-type-parameter-bound
    // Causes the type to function *as though* it has a `&'a ()` field,
    // despite not *actually* having one.
    _marker: PhantomData<&'a ()>,
}

impl Strings<'static> {
    pub const APP_NAME: &'static str = "Sirius Hydra";
    pub const FFMPEG_PROCESS_SHELL_COMMAND: &'static str = "sh -c ffmpeg";
    pub const FFMPEG_PROCESS_NAME: &'static str = "ffmpeg";
    pub const FFMPEG_EXECUTABLE: &'static str = "ffmpeg";
    pub const FFPLAY_PROCESS_NAME: &'static str = "ffplay";
    pub const FFPLAY_EXECUTABLE: &'static str = "ffplay";
}
