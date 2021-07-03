use clap::{App, Arg};
use otdrs::parser::parse_file;
use std::fs::File;
use std::io::prelude::*;


fn main() -> std::io::Result<()>  {
    let matches = App::new("otdr-fixer")
        .version("0.1")
        .author("James Harrison <james@gigaclear.com>")
        .about("Applies simple modifications to OTDR files")
        .subcommand(App::new("set_identifiers")
            .about("Allows the identifiers within a specific file to be modified and written to a new file")
            .arg(
                Arg::new("in_path")
                    .required(true)
                    .short('i')
                    .long("in_path")
                    .takes_value(true),
            )
            .arg(
                Arg::new("out_path")
                    .required(true)
                    .short('o')
                    .long("out_path")
                    .takes_value(true),
            )
            .arg(
                Arg::new("fibre_id").required(false).short('f').long("fibre_id").takes_value(true)
                .about("Set the fibre identifier")
            )
            .arg(
                Arg::new("cable_id").required(false).short('c').long("cable_id").takes_value(true)
                .about("Set the cable identifier")
            )
            .arg(
                Arg::new("originating_location").required(false).short('a').long("originating_location").takes_value(true)
                .about("Set the originating location of the trace, aka Location A")
            )
            .arg(
                Arg::new("terminating_location").required(false).short('b').long("terminating_location").takes_value(true)
                .about("Set the terminating location of the trace, aka Location B")
            )
            .arg(
                Arg::new("operator").required(false).long("operator").takes_value(true)
                .about("Set the operator name for the trace")
            )
            .arg(
                Arg::new("comment").required(false).long("comment").takes_value(true)
                .about("Set the free text comment")
            )
        )
        .get_matches();
    

    if let Some(matches) = matches.subcommand_matches("set_identifiers") {
        
        let in_path = matches.value_of("in_path").unwrap();
        let mut in_file = File::open(in_path)?;
        let mut in_data = Vec::new();
        in_file.read_to_end(&mut in_data)?;
        let mut sor = parse_file(&in_data).unwrap().1;
        match sor.general_parameters.as_mut() {
            Some(mut gp) => {
                if matches.is_present("fibre_id") {
                    gp.fiber_id = matches.value_of("fibre_id").unwrap();
                }
                if matches.is_present("cable_id") {
                    gp.cable_id = matches.value_of("cable_id").unwrap();
                }
                if matches.is_present("originating_location") {
                    gp.originating_location = matches.value_of("originating_location").unwrap();
                }
                if matches.is_present("terminating_location") {
                    gp.terminating_location = matches.value_of("terminating_location").unwrap();
                }
                if matches.is_present("operator") {
                    gp.operator = matches.value_of("operator").unwrap();
                }
                if matches.is_present("comment") {
                    gp.comment = matches.value_of("comment").unwrap();
                }
            }
            None => {}
        }
        let mut out_file = File::create(matches.value_of("out_path").unwrap()).unwrap();
        let bytes_to_write = sor.to_bytes().unwrap();
        out_file.write_all(&bytes_to_write)?;

    }


    return Ok(());
}
