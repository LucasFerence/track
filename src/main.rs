/// Use:
/// 
/// To add new task -> return ID
/// `track new "This is a custom daily task"`
/// 
/// To remove task
/// `track remove {id}`
/// 
/// Show existing tasks with IDs, and time tracked for each
/// `track report`
/// 
/// Begin tracking a specific task
/// `track start {id}`
/// 
/// End tracking a specific task -> return time active
/// `track end {id}`

use std::process;
use chrono::Local;

use track::Res;
use track::app;
use track::manager;
use track::table;
use track::file::FileAccess;

fn main() {
    if let Err(err) = try_main() {
        eprintln!("{}", err);
        process::exit(2);
    }
}

fn try_main() -> Res<()> {
    // Ensure the file is correct before we do anything
    manager::ensure_file()?;

    // Get the file access and manager
    let file_access = FileAccess::new();
    let mut manager: manager::Manager = file_access.read()?;

    // Get the gorup for today
    let today_group = today();

    // Match the input
    let matches = app::app().get_matches();

    // NEW
    if let Some(sub) = matches.subcommand_matches(app::New::name()) {
        // Can use unwrap because it is required
        let task_name = sub.value_of(app::NewValue::name()).unwrap();

        // Add the task to todays group
        manager.add_task(today_group, task_name.to_owned());
    }

    // REPORT
    else if let Some(_) = matches.subcommand_matches(app::Report::name()) {
        if let Some(today) = manager.group(&today_group) {
            table::display(today);
        }
    }

    // START
    else if let Some(sub) = matches.subcommand_matches(app::Start::name()) {
        // Can use unwrap because it is required
        let id = sub.value_of(app::StartValue::name()).unwrap();

        if let Some(parsed_id) = id.parse::<usize>().ok() {
            manager.start_task(parsed_id);
        }
    }

    // STOP
    else if let Some(_) = matches.subcommand_matches(app::Stop::name()) {
        manager.stop_current();
    }

    file_access.write(&manager)?;
    Ok(())
}

fn today() -> String {
    Local::now().date().to_string()
}
