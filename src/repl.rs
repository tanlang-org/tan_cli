use std::io::{stdout, Write};

use rustyline::{error::ReadlineError, Editor};
use tan::{eval::env::Env, expr::Expr};

use crate::util::eval_string_with_error_report;

const HISTORY_FILENAME: &str = ".tan_history.txt";

// #TODO if we use an array for %i and %o, we can apply all the Seq functions, potentially useful! but it's less ergonomic.
// #TODO don't advance the line-variable if the input is just a comment
// #TODO `---` comments are convenient as separators.

// #TODO rename to `shell` or something else?
pub fn handle_repl() -> anyhow::Result<()> {
    // #TODO support completer!
    // `()` can be used when no completer is required
    let mut rl = Editor::<()>::new()?;

    if rl.load_history(HISTORY_FILENAME).is_err() {
        println!("No previous history.");
    }

    println!("Tan, press CTRL-D to exit.");

    let mut env = Env::prelude();

    let mut index = 0;

    loop {
        // #TODO what would be a cool prompt? (Wolfram Language has an interesting prompt).
        // #TODO have prefix for output/result also.
        // #TODO try to use the legendary `READY` in some capacity.
        let readline = rl.readline(&format!("{index}> "));

        match readline {
            Ok(line) => {
                rl.add_history_entry(&line);

                // #TODO find better input variable name.
                // #TODO use input list/array, like wolfram, e.g. (*in* 1), nah too difficult to type!
                env.insert(format!("$i{index}"), Expr::String(line.clone()));

                let Some(value) = eval_string_with_error_report(&line, &mut env) else {
                    continue;
                };

                // #TODO find better output variable name.
                // #TODO use output list/array, like wolfram, e.g. (*out* 1)
                env.insert(format!("$o{index}"), value.clone());

                match value {
                    Expr::One => (),
                    _ => println!("{value}"),
                }

                let _ = stdout().flush();
            }
            Err(ReadlineError::Interrupted) => {
                println!("CTRL-C");
                break;
            }
            Err(ReadlineError::Eof) => {
                println!("CTRL-D");
                break;
            }
            Err(err) => {
                eprintln!("ERROR: {err:?}");
                break;
            }
        }

        index += 1;
    }

    // #TODO could we trap the (exit)?
    rl.save_history(HISTORY_FILENAME).unwrap();

    Ok(())
}
