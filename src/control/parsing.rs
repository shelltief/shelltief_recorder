//! # Parsing
//!
//! Crate to control the parsing of options in `shellrecord` cli
use crate::ffmpeg::{get_devices, Stream};
use std::convert::TryFrom;

/// enum to register an option
/// Can be either an option without a provided argument
/// or an option with an array of arguments
pub(crate) enum CliOption {
    Noarg(String),
    Argument{opt: String, args: Vec<String>},
}

pub(crate) struct Command {
    opts: Option<Vec<CliOption>>,
    args: Vec<String>,
}

fn get_option(args: &Vec<String>, i: &mut usize) -> Result<CliOption, String> {
    let opt = args[*i].clone();
    if opt.starts_with("---") {
        return Err(format!("Option: '{opt}' starts with 3 dashes -- illegal"));
    } if ! opt.starts_with("--") && opt.len() != 2 {
        return Err(format!("Option: '{opt}' is not a short option with one letter -- illegal"));
    }
    let opt = opt.trim_start_matches("-").to_owned();
    *i += 1;
    if *i == args.len() {
        return Ok(CliOption::Noarg(opt));
    }
    let mut optargs: Vec<String> = Vec::new();
    while *i < args.len() && ! args[*i].starts_with("-") {
        optargs.push(args[*i].clone());
        *i += 1;
    }
    Ok(CliOption::Argument{opt, args: optargs})
}

pub(crate) fn parse_command_line(args: Vec<String>) -> Result<Option<Command>, String>
{
    let mut i: usize = 0;
    if args.len() == 0 {
        return Ok(None);
    }
    let mut parsed_opts: Vec<CliOption> = Vec::new();
    let mut parsed_args: Vec<String> = Vec::new();
    let mut arg: String;
    while i < args.len() && args[i] != "--" && args[i].starts_with("-") {
        arg = args[i].clone();
        parsed_opts.push(get_option(&args, &mut i)?);
    }
    if i < args.len() && args[i] == "--" {
        i += 1;
    }
    while i < args.len() {
        parsed_args.push(args[i].clone());
        i += 1;
    }
    if parsed_opts.len() == 0 {
        return Ok(Some(Command{
            opts: None,
            args: parsed_args,
        }));
    }
    Ok(Some(Command{
        opts: Some(parsed_opts),
        args: parsed_args,
    }))
}

impl TryFrom<CliOption> for Stream {
    type Error = String;

    fn try_from(value: CliOption) -> Result<Self, Self::Error> {
        match value {
            CliOption::Noarg(arg) => Err(String::from("No argument provided for stream")),
            CliOption::Argument{opt, args} => {
                if opt != "stream" && opt != "s" {
                    return Err(format!("Option '{opt}' is not 'stream' or 's' "));
                }
                match args.len() {
                    2 => Ok(Stream::new(args[0].clone(), None, args[1].clone(), None)),
                    3 => Ok(Stream::new(args[0].clone(), Some(args[1].clone()),
                    args[2].clone(), None)),
                    _ => Err(format!("Illegal number of arguments for stream:\
'{}' args received", args.len())),
                }
            },
        }
    }
}

fn help() {
    let help_str = "
shellrecord [--stream|-s video [audio] output] [--path|-p project_dir_path] project_name

--stream|-s video [audio] output
    specifies video stream (with optional audio) to capture

--path|-p project_dir_path
    specifies path in which to look for project name directory
";
    eprintln!("{help_str}");
}

pub(super) fn parse_options(command: Command) -> Result<(Vec<Stream>, String, Option<String>), String> {
    let Command{opts, args} = command;
    if args.len() != 1 {
        return Err(String::from("project_name should be the only argument"));
    }
    let project_name = args[0].clone();
    if opts.is_none() {
        return Err(String::from("at least one 'stream' argument should be present"));
    }
    let opts: Vec<CliOption> = opts.unwrap();
    let mut streams: Vec<Stream> = Vec::new();
    let mut project_path: Option<String> = None;
    for option in opts.into_iter() {
        match option {
            CliOption::Noarg(opt) if opt == "h" || opt == "help" => {
                help();
                return Err(String::new());
            },
            CliOption::Noarg(opt) if opt == "l" || opt == "list" => {
                eprintln!("{:#?}", get_devices());
                return Err(String::new());
            }
            CliOption::Noarg(opt) => return Err(String::from("Illegal option '{opt}'")),
            CliOption::Argument{ref opt, ref args} if opt == "stream" || opt == "s" => {
                streams.push(Stream::try_from(option)?);
            },
            CliOption::Argument{opt, args} if opt == "path" || opt == "p" => {
                if args.len() != 1 {
                    return Err(format!("project path should be the only argument to option: '{opt}'"));
                }
                if let Some(path) = project_path {
                    return Err(String::from("Multiple project paths provided"));
                }
                project_path = Some(args[0].clone());
            },
            CliOption::Argument{opt,args: _} => return Err(format!("Illegal option: '{opt}' detected")),
        }
    }
    Ok((streams, project_name, project_path))
}
