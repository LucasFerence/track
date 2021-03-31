use clap::{App, Arg, crate_name, crate_authors, crate_version};

pub fn app() -> App<'static, 'static> {
    App::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!())
        .subcommand(New::create())
}

// --- NEW SUBCOMMAND ---

pub struct New;
impl New {
    fn create() -> App<'static, 'static> {
        App::new(Self::name())
            .arg(NewValue::create())
    }

    pub fn name() -> &'static str {
        "new"
    }
}

pub struct NewValue;
impl NewValue {
    fn create() -> Arg<'static, 'static> {
        Arg::with_name(Self::name())
            .required(true)
            .index(1)
    }

    pub fn name() -> &'static str {
        "new-value"
    }
}
