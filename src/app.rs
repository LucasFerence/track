use clap::{App, Arg, crate_name, crate_authors, crate_version};

pub fn app() -> App<'static, 'static> {
    App::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!())
        .subcommand(New::create())
        .subcommand(Remove::create())
        .subcommand(Tasks::create())
        .subcommand(Groups::create())
        .subcommand(Use::create())
        .subcommand(Start::create())
        .subcommand(Stop::create())
        .subcommand(Tomorrow::create())
        .subcommand(Complete::create())
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

// --- TASKS SUBCOMMAND ---

pub struct Tasks;
impl Tasks {
    fn create() -> App<'static, 'static> {
        App::new(Self::name())
    }

    pub fn name() -> &'static str {
        "tasks"
    }
}

// --- GROUPS SUBCOMMAND ---

pub struct Groups;
impl Groups {
    fn create() -> App<'static, 'static> {
        App::new(Self::name())
    }

    pub fn name() -> &'static str {
        "groups"
    }
}

// --- USE SUBCOMMAND ---

pub struct Use;
impl Use {
    fn create() -> App<'static, 'static> {
        App::new(Self::name())
            .arg(UseValue::create())
            .arg(UseReset::create())
    }

    pub fn name() -> &'static str {
        "use"
    }
}

pub struct UseValue;
impl UseValue {
    fn create() -> Arg<'static, 'static> {
        Arg::with_name(Self::name())
            .index(1)
    }

    pub fn name() -> &'static str {
        "use-value"
    }
}

pub struct UseReset;
impl UseReset {
    fn create() -> Arg<'static, 'static> {
        Arg::with_name(Self::name())
            .short("r")
    }

    pub fn name() -> &'static str {
        "use-reset"
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

// --- TOMORROW SUBCOMMAND ---

pub struct Tomorrow;
impl Tomorrow {
    fn create() -> App<'static, 'static> {
        App::new(Self::name())
    }

    pub fn name() -> &'static str {
        "tmrw"
    }
}

// --- COMPLETE SUBCOMMAND ---

pub struct Complete;
impl Complete {
    fn create() -> App<'static, 'static> {
        App::new(Self::name())
            .arg(CompleteValue::create())
            .arg(CompleteCurrent::create())
    }

    pub fn name() -> &'static str {
        "complete"
    }
}

pub struct CompleteValue;
impl CompleteValue {
    fn create() -> Arg<'static, 'static> {
        Arg::with_name(Self::name())
            .index(1)
    }

    pub fn name() -> &'static str {
        "complete-value"
    }
}

pub struct CompleteCurrent;
impl CompleteCurrent {
    fn create() -> Arg<'static, 'static> {
        Arg::with_name(Self::name())
            .short("c")
    }

    pub fn name() -> &'static str {
        "complete-curr"
    }
}
