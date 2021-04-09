use std::process;

use track::Res;
use track::app;
use track::manager;
use track::table;
use track::time;
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
    let today_group = time::today_date();

    // Match the input
    let matches = app::app().get_matches();

    // NEW
    if let Some(sub) = matches.subcommand_matches(app::New::name()) {
        // Can use unwrap because it is required
        let task_name = sub.value_of(app::NewValue::name()).unwrap();

        // Add the task to todays group
        manager.add_task(today_group, task_name.to_owned());
    }

    // REMOVE
    else if let Some(sub) = matches.subcommand_matches(app::Remove::name()) {
        let id = sub.value_of(app::RemoveValue::name())
            .unwrap()
            .parse::<usize>()?;

        manager.remove_task(id);
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
        let id = sub.value_of(app::StartValue::name())
            .unwrap()
            .parse::<usize>()?;

        let started_id = manager.start_task(id)?;

        // If the task was successfully created, display it
        if let Some(task) = manager.task(started_id) {
            println!("Starting:");
            table::display(task);
        }
    }

    // STOP
    else if let Some(_) = matches.subcommand_matches(app::Stop::name()) {
        let stopped_id = manager.stop_current()?;

        if let Some(task) = manager.task(stopped_id) {
            println!("Stopping:");
            table::display(task);
        }
    }

    file_access.write(&manager)?;
    Ok(())
}
