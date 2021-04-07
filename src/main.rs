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

use track::Res;
use track::app;
use track::data;
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
    data::ensure_file()?;

    let matches = app::app().get_matches();

    // NEW
    if let Some(sub) = matches.subcommand_matches(app::New::name()) {

        // Can use unwrap because it is required
        let value = sub.value_of(app::NewValue::name()).unwrap();

        // Get file access and root
        let file_access = FileAccess::new();
        let mut root: data::Root = file_access.read()?;

        // Create entry from input value
        let entry = root.create_entry(value.to_owned());

        // Either add task to current day, or create a new day
        match root.today() {
            Some(today) => {
                today.add_entry(entry);
            },
            _ => {
                let mut new_day = data::Day::new();
                new_day.add_entry(entry);
                root.add_day(new_day);
            }
        }

        file_access.write(&root)?;
    }

    // REPORT
    if let Some(_) = matches.subcommand_matches(app::Report::name()) {
        // Show the current day

        // Get file access and root
        let file_access = FileAccess::new();
        let mut root: data::Root = file_access.read()?;

        if let Some(today) = root.today() {
            table::display(today);
        }
    }

    // START
    if let Some(sub) = matches.subcommand_matches(app::Start::name()) {

        // Can use unwrap because it is required
        let id = sub.value_of(app::StartValue::name()).unwrap();

        // Get file access and root
        let file_access = FileAccess::new();
        let mut root: data::Root = file_access.read()?;

        // Find the entry in the current day with the ID
        let found_entry = root.today()
            .map(|today| today.find_by_id(id))
            .unwrap_or_default();

        if let Some(fe) = found_entry {
            fe.start();

            println!("Started:");
            table::display(fe);

            file_access.write(&root)?;
        }
    }

    Ok(())
}
