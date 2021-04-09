use clap::{App, Arg, crate_name, crate_authors, crate_version};

pub fn app() -> App<'static, 'static> {
    App::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!())
        .subcommand(New::create())
        .subcommand(Remove::create())
        .subcommand(Report::create())
        .subcommand(Start::create())
        .subcommand(Stop::create())
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

// --- REMOVE COMMAND ---

pub struct Remove;
impl Remove {
    fn create() -> App<'static, 'static> {
        App::new(Self::name())
            .arg(RemoveValue::create())
    }

    pub fn name() -> &'static str {
        "remove"
    }
}

pub struct RemoveValue;
impl RemoveValue {
    fn create() -> Arg<'static, 'static> {
        Arg::with_name(Self::name())
            .required(true)
            .index(1)
    }

    pub fn name() -> &'static str {
        "remove-value"
    }
}

// --- REPORT SUBCOMMAND ---

pub struct Report;
impl Report {
    fn create() -> App<'static, 'static> {
        App::new(Self::name())
    }

    pub fn name() -> &'static str {
        "report"
    }
}

// --- START SUBCOMMAND ---

pub struct Start;
impl Start {
    fn create() -> App<'static, 'static> {
        App::new(Self::name())
            .arg(StartValue::create())
    }

    pub fn name() -> &'static str {
        "start"
    }
}

pub struct StartValue;
impl StartValue {
    fn create() -> Arg<'static, 'static> {
        Arg::with_name(Self::name())
            .required(true)
            .index(1)
    }

    pub fn name() -> &'static str {
        "start-value"
    }
}

// --- STOP SUBCOMMAND ---

pub struct Stop;
impl Stop {
    fn create() -> App<'static, 'static> {
        App::new(Self::name())
    }

    pub fn name() -> &'static str {
        "stop"
    }
}
