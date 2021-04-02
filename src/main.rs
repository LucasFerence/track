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

    if let Some(sub) = matches.subcommand_matches(app::New::name()) {

        // Can use unwrap because it is required
        let value = sub.value_of(app::NewValue::name()).unwrap();

        // Get file access and root
        let file_access = FileAccess::new();
        let mut root: data::Root = file_access.read()?;

        // Create entry from input value
        let entry = root.create_entry(value.to_owned());

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

    Ok(())
}
