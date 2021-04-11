///
/// TODO:
/// 1. Allow archiving current file or opening an archive file
/// 2. Add complete/uncomplete functionality

use std::process;

use track::{Res, ResErr};
use track::app;
use track::manager;
use track::table;
use track::time;

fn main() {
    if let Err(err) = try_main() {
        eprintln!("{}", err);
        process::exit(2);
    }
}

fn try_main() -> Res<()> {
    let mut manager = manager::Manager::init()?;

    // Match the input
    let matches = app::app().get_matches();

    // NEW
    if let Some(sub) = matches.subcommand_matches(app::New::name()) {
        // Can use unwrap because it is required
        let task_name = sub.value_of(app::NewValue::name()).unwrap();

        // Add the task to todays group
        let new_task = manager.add_task(task_name.to_owned())?;

        // Display
        println!("Added:");
        table::display(&new_task);
    }

    // REMOVE
    else if let Some(sub) = matches.subcommand_matches(app::Remove::name()) {
        let id = sub.value_of(app::RemoveValue::name())
            .unwrap()
            .parse::<usize>()?;

        let removed_task = manager.remove_task(id)?;

        // Display
        println!("Removed:");
        table::display(&removed_task);
    }

    // TASKS
    else if let Some(_) = matches.subcommand_matches(app::Tasks::name()) {
        let group = manager.group()?;
        println!("{}:", group.name());
        table::display(group);
    }

    // GROUPS
    else if let Some(_) = matches.subcommand_matches(app::Groups::name()) {
        table::display(&manager);
    }

    // USE
    else if let Some(sub) = matches.subcommand_matches(app::Use::name()) {
        
        if sub.occurrences_of(app::UseReset::name()) > 0 {
            // If we want to reset the used group, it will make it whatever today is
            manager.reset_group();
            println!("Resetting group...")
        } else {
            let id = sub.value_of(app::UseValue::name())
                .ok_or(ResErr::from("Invalid command"))?
                .parse::<usize>()?;

            manager.use_group(id)?;
        }

        let group = manager.group()?;
        println!("Using group: {}", group.name());
    }

    // START
    else if let Some(sub) = matches.subcommand_matches(app::Start::name()) {
        // Can use unwrap because it is required
        let id = sub.value_of(app::StartValue::name())
            .unwrap()
            .parse::<usize>()?;

        let started_task = manager.start_task(id)?;

        println!("Starting:");
        table::display(&started_task);
    }

    // STOP
    else if let Some(_) = matches.subcommand_matches(app::Stop::name()) {
        let stopped_task = manager.stop_current()?;

        println!("Stopping:");
        table::display(&stopped_task);
    }

    // TOMORROW
    else if let Some(_) = matches.subcommand_matches(app::Tomorrow::name()) {
        let tomorrow = time::tomorrow_local()
            .format(manager::DATE_FORMAT)
            .to_string();

        let group = manager.add_group(tomorrow)?;
        let group_name = group.name();

        println!("Added group: {}", group_name);
        
        manager.use_group(group.id())?;
        println!("Using group: {}", group_name);
    }

    manager.commit()
}
