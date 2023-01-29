# Simple CLI ToDo list/journal using Rust

## Usage examples

**Add** (adds new task to specific or default journal file. If there is none it creates the default one named `.rusty-journal.json`)
- `<program_name> -j .custom-journal.json add "Take the trash out before your wife comes home"`
- `<program_name> add "Close the flowers and water the windows"`

**List** (lists current tasks from specific or default journal file if there are any)
- `<program_name> -j .custom-journal.json list`
- `<program_name> list`

**Done** (removes task from the list by position which you can see with `list` command)
- `<program_name> -j .custom-journal.json done 2`
- `<program_name> done 2`

