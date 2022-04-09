use std::marker::PhantomData;

#[derive(Debug)]
pub struct PinValues<'a> {
    /// https://stackoverflow.com/questions/40484154/parameter-a-is-never-used-error-when-a-is-used-in-type-parameter-bound
    // Causes the type to function *as though* it has a `&'a ()` field,
    // despite not *actually* having one.
    _marker: PhantomData<&'a ()>,
}

impl PinValues<'static> {
    pub const SHUTDOWN_BTN: u8 = 21;
    pub const RADIO_RESTART_BTN: u8 = 20;
    pub const PROGRAM_POWER_LED: u8 = 26;
}
