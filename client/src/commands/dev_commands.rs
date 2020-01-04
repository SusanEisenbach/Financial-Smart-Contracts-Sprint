use super::Command;
use chrono::{DateTime, Utc};
use client::client_proxy::ClientProxy;
use libra_types::waypoint::Waypoint;
use std::collections::HashMap;
use std::env;
use std::path::Path;
use std::time::{Duration, UNIX_EPOCH};

/// Major command for account related operations.
pub struct DevCommand {}

impl Command for DevCommand {
    fn get_aliases(&self) -> Vec<&'static str> {
        vec!["dev"]
    }
    fn get_description(&self) -> &'static str {
        "Local move development"
    }
    fn execute(&self, client: &mut ClientProxy, params: &[&str]) {
        let commands: Vec<Box<dyn Command>> = vec![
            Box::new(DevCommandCompile {}),
            Box::new(DevCommandPublish {}),
            Box::new(DevCommandExecute {}),
            Box::new(DevCommandAddValidator {}),
            Box::new(DevCommandRemoveValidator {}),
            Box::new(DevCommandGenWaypoint {}),
        ];
        subcommand_execute(&params[0], commands, client, &params[1..]);
    }
}

/// Sub command to compile move program
pub struct DevCommandCompile {}

impl Command for DevCommandCompile {
    fn get_aliases(&self) -> Vec<&'static str> {
        vec!["compile", "c"]
    }
    fn get_params_help(&self) -> &'static str {
        "<sender_account_address>|<sender_account_ref_id> <file_path> <module|script> [output_file_path (compile into tmp file by default)]"
    }
    fn get_description(&self) -> &'static str {
        "Compile move program"
    }
    fn execute(&self, client: &mut ClientProxy, params: &[&str]) {
        if params.len() < 4 || params.len() > 5 {
            println!("Invalid number of arguments for compilation");
            return;
        }
        println!(">> Compiling program");

        // Update working directory to where libra repository is found
        // TODO: Remove once libra doesn't rely on compiler cargo memeber
        let current_working_directory = env::current_dir().unwrap();
        // TODO: Unhard code this
        let libra_directory = Path::new("../libra");
        assert!(env::set_current_dir(&libra_directory).is_ok());

        match client.compile_program(params) {
            Ok(path) => println!("Successfully compiled a program at {}", path),
            Err(e) => println!("{}", e),
        }

        // Change working directory back to original working directory.
        // TODO: Remove once libra doesn't rely on compiler cargo memeber
        assert!(env::set_current_dir(&current_working_directory).is_ok());
    }
}

/// Sub command to publish move resource
pub struct DevCommandPublish {}

impl Command for DevCommandPublish {
    fn get_aliases(&self) -> Vec<&'static str> {
        vec!["publish", "p"]
    }

    fn get_params_help(&self) -> &'static str {
        "<sender_account_address>|<sender_account_ref_id> <compiled_module_path>"
    }

    fn get_description(&self) -> &'static str {
        "Publish move module on-chain"
    }

    fn execute(&self, client: &mut ClientProxy, params: &[&str]) {
        if params.len() != 3 {
            println!("Invalid number of arguments to publish module");
            return;
        }
        match client.publish_module(params) {
            Ok(_) => println!("Successfully published module"),
            Err(e) => println!("{}", e),
        }
    }
}

/// Sub command to execute custom move script
pub struct DevCommandExecute {}

impl Command for DevCommandExecute {
    fn get_aliases(&self) -> Vec<&'static str> {
        vec!["execute", "e"]
    }

    fn get_params_help(&self) -> &'static str {
        "<sender_account_address>|<sender_account_ref_id> <compiled_module_path> [parameters]"
    }

    fn get_description(&self) -> &'static str {
        "Execute custom move script"
    }

    fn execute(&self, client: &mut ClientProxy, params: &[&str]) {
        if params.len() < 3 {
            println!("Invalid number of arguments to execute script");
            return;
        }
        match client.execute_script(params) {
            Ok(_) => println!("Successfully finished execution"),
            Err(e) => println!("{}", e),
        }
    }
}

pub struct DevCommandAddValidator {}

impl Command for DevCommandAddValidator {
    fn get_aliases(&self) -> Vec<&'static str> {
        vec!["add_validator"]
    }

    fn get_params_help(&self) -> &'static str {
        "<validator_account_address>"
    }

    fn get_description(&self) -> &'static str {
        "Add an account address to the validator set"
    }

    fn execute(&self, client: &mut ClientProxy, params: &[&str]) {
        if params.len() != 2 {
            println!("Invalid number of arguments to add validator");
            return;
        }
        match client.add_validator(params, true) {
            Ok(_) => println!("Successfully finished execution"),
            Err(e) => println!("{}", e),
        }
    }
}

pub struct DevCommandRemoveValidator {}

impl Command for DevCommandRemoveValidator {
    fn get_aliases(&self) -> Vec<&'static str> {
        vec!["remove_validator"]
    }

    fn get_params_help(&self) -> &'static str {
        "<validator_account_address>"
    }

    fn get_description(&self) -> &'static str {
        "Remove an existing account address from the validator set"
    }

    fn execute(&self, client: &mut ClientProxy, params: &[&str]) {
        if params.len() != 2 {
            println!("Invalid number of arguments to remove validator");
            return;
        }
        match client.remove_validator(params, true) {
            Ok(_) => println!("Successfully finished execution"),
            Err(e) => println!("{}", e),
        }
    }
}

pub struct DevCommandGenWaypoint {}

impl Command for DevCommandGenWaypoint {
    fn get_aliases(&self) -> Vec<&'static str> {
        vec!["gen_waypoint"]
    }

    fn get_params_help(&self) -> &'static str {
        ""
    }

    fn get_description(&self) -> &'static str {
        "Generate a waypoint for the latest epoch change LedgerInfo"
    }

    fn execute(&self, client: &mut ClientProxy, params: &[&str]) {
        if params.len() != 1 {
            println!("No parameters required for waypoint generation");
            return;
        }
        println!("Retrieving the uptodate ledger info...");
        if let Err(e) = client.test_validator_connection() {
            println!("Failed to get uptodate ledger info connection: {}", e);
            return;
        }

        let latest_epoch_change_li = match client.latest_epoch_change_li() {
            Some(li) => li,
            None => {
                println!("No epoch change LedgerInfo found");
                return;
            }
        };
        let li_time_str = DateTime::<Utc>::from(
            UNIX_EPOCH
                + Duration::from_micros(latest_epoch_change_li.ledger_info().timestamp_usecs()),
        );
        match Waypoint::new(latest_epoch_change_li.ledger_info()) {
            Err(e) => println!("Failed to generate a waypoint: {}", e),
            Ok(waypoint) => println!(
                "Waypoint (end of epoch {}, time {}): {}",
                latest_epoch_change_li.ledger_info().epoch(),
                li_time_str,
                waypoint
            ),
        }
    }
}

/// Execute sub command.
pub fn subcommand_execute(
    parent_command_name: &str,
    commands: Vec<Box<dyn Command>>,
    client: &mut ClientProxy,
    params: &[&str],
) {
    let mut commands_map = HashMap::new();
    for (i, cmd) in commands.iter().enumerate() {
        for alias in cmd.get_aliases() {
            if commands_map.insert(alias, i) != None {
                panic!("Duplicate alias {}", alias);
            }
        }
    }

    if params.is_empty() {
        print_subcommand_help(parent_command_name, &commands);
        return;
    }

    match commands_map.get(&params[0]) {
        Some(&idx) => commands[idx].execute(client, &params),
        _ => print_subcommand_help(parent_command_name, &commands),
    }
}

/// Print the help message for all sub commands.
pub fn print_subcommand_help(parent_command: &str, commands: &[Box<dyn Command>]) {
    println!(
        "usage: {} <arg>\n\nUse the following args for this command:\n",
        parent_command
    );
    for cmd in commands {
        println!(
            "{} {}\n\t{}",
            cmd.get_aliases().join(" | "),
            cmd.get_params_help(),
            cmd.get_description()
        );
    }
    println!("\n");
}
