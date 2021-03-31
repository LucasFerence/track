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

fn main() {
    if let Err(err) = try_main() {
        eprintln!("{}", err);
        process::exit(2);
    }
}

fn try_main() -> Res<()> {
    let matches = app::app().get_matches();

    if let Some(sub) = matches.subcommand_matches(app::New::name()) {

        // Can use unwrap because it is required
        let value = sub.value_of(app::NewValue::name()).unwrap();
        println!("{:?}", value);
    }

    Ok(())
}
