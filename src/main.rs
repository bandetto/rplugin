#![allow(unused)]

use std::env::{args, Args};
use std::fs::{create_dir_all, remove_dir_all, File};
use std::io::{Read, Write};
use std::path::PathBuf;
use std::process::ExitCode;
use std::thread::sleep;
use std::time::Duration;
use std::time::SystemTime;

type Fail = Result<(), String>;
type FailOr<T> = Result<T, String>;

const PLUGIN_DIR: &str = "/tmp/rplugin";

fn split_daydir_dir_file(p: &str) -> FailOr<(String, String, String)>
{
  let path = PathBuf::from(p);
  let filename = path.file_name()
                     .map(|e| e.to_str().expect("file is valid unicode"))
                     .ok_or("invalid local path for filename".to_string())?;
  let file_dir = path.parent()
                     .and_then(|d| d.file_name())
                     .ok_or("invalid local path for dirname".to_string())?;
  let dirname = file_dir.to_str().expect("dirname is valid unicode");
  let daydirname = if dirname.len() >= 8 {
    &dirname[..8]
  } else {
    return Err("invalid local path for daydirname".to_string());
  };
  log(format!("filename split: {daydirname}, {dirname}, {filename}"));
  Ok((daydirname.into(), dirname.into(), filename.into()))
}

fn split_daydir_dir(p: &str) -> FailOr<(String, String)>
{
  let path = PathBuf::from(p);
  let dir_path =
    path.file_name().ok_or("invalid local path for dirname".to_string())?;
  let dirname = dir_path.to_str().expect("dirname is valid unicode");
  let day_dir_path = path.parent()
                         .and_then(|p| p.file_name())
                         .ok_or("invalid local path for dirname".to_string())?;
  let daydirname = day_dir_path.to_str().expect("daydirname is valid unicode");
  log(format!("dir split: {daydirname}, {dirname}"));
  Ok((daydirname.into(), dirname.into()))
}

fn rand() -> bool
{
  let e = SystemTime::now().duration_since(std::time::UNIX_EPOCH)
                           .expect("time went backwards")
                           .as_nanos();
  (e & 31) == 31
}

fn randomly_fail() -> Fail
{
  if cfg!(feature = "fail") && rand() {
    return Err("fail triggered".to_string());
  }
  Ok(())
}

fn log<T: AsRef<str>>(s: T)
{
  #[cfg(feature = "logs")]
  eprintln!("rplugin: {}", s.as_ref())
}

fn setup_plugin_for_backup(mut args: Args) -> Fail
{
  let local_path = args.nth(1);
  if let Some(local_path) = local_path {
    let (daydirname, dirname) = split_daydir_dir(&local_path)?;
    create_dir_all(format!("{PLUGIN_DIR}/{daydirname}/{dirname}"))
      .map_err(|err| format!("could not create backup directory: {err}"))?;
    Ok(())
  } else {
    Err("no local path is provided".to_string())
  }
}

fn setup_plugin_for_restore(mut args: Args) -> Fail
{
  Ok(())
}

fn cleanup_plugin_for_backup(mut args: Args) -> Fail
{
  Ok(())
}

fn cleanup_plugin_for_restore(mut args: Args) -> Fail
{
  Ok(())
}

fn backup_file(mut args: Args) -> Fail
{
  #[cfg(feature = "delay")]
  sleep(Duration::from_secs(1));
  randomly_fail()?;
  let local_path = args.nth(1);
  if let Some(local_path) = local_path {
    let (daydirname, dirname, filename) = split_daydir_dir_file(&local_path)?;
    let mut backup_file =
      File::create(format!("{PLUGIN_DIR}/{daydirname}/{dirname}/{filename}"))
        .map_err(|err| format!("could not create backup file: {err}"))?;
    let mut local_file = File::open(local_path).map_err(|err| {
                           format!("could not open local file: {err}")
                         })?;
    let mut local_data = vec![];
    local_file.read_to_end(&mut local_data)
              .map_err(|err| format!("could not read local file: {err}"))?;
    backup_file.write_all(&local_data)
               .map_err(|err| format!("could not write to a backup_file: {err}"))?;
    backup_file.flush().map_err(|err| {
                          format!("could not flush a backup_file: {err}")
                        })?;
    Ok(())
  } else {
    Err("no local path is provided".to_string())
  }
}

fn restore_file(mut args: Args) -> Fail
{
  #[cfg(feature = "delay")]
  sleep(Duration::from_secs(1));
  randomly_fail()?;
  let local_path = args.nth(1);
  if let Some(local_path) = local_path {
    let (daydirname, dirname, filename) = split_daydir_dir_file(&local_path)?;
    let mut restore_file =
      File::open(format!("{PLUGIN_DIR}/{daydirname}/{dirname}/{filename}"))
        .map_err(|err| format!("could not open backup file: {err}"))?;
    let mut local_file = File::create(local_path).map_err(|err| {
                           format!("could not create local file: {err}")
                         })?;
    let mut restore_data = vec![];
    restore_file.read_to_end(&mut restore_data)
                .map_err(|err| format!("could not read restore file: {err}"))?;
    local_file.write_all(&restore_data)
              .map_err(|err| format!("could not write to local file: {err}"))?;
    local_file.flush()
              .map_err(|err| format!("could not flush a local file: {err}"))?;
    Ok(())
  } else {
    Err("no local path is provided".to_string())
  }
}

fn backup_data(mut args: Args) -> Fail
{
  #[cfg(feature = "delay")]
  sleep(Duration::from_secs(1));
  randomly_fail()?;
  let local_path = args.nth(1);
  if let Some(local_path) = local_path {
    let (daydirname, dirname, filename) = split_daydir_dir_file(&local_path)?;
    let mut stdin = std::io::stdin();
    let mut backup_file =
      File::create(format!("{PLUGIN_DIR}/{daydirname}/{dirname}/{filename}"))
        .map_err(|_| "could not create backup file")?;
    loop {
      let mut b = [0; 1024];
      let count =
        stdin.read(&mut b)
             .map_err(|err| format!("could not read from stdin: {err}"))?;
      if count == 0 {
        break;
      }
      backup_file.write_all(&b[..count])
                 .map_err(|err| format!("could not write data to backup file: {err}"))?;
    }
    backup_file.flush().map_err(|err| {
                          format!("could not flush a backup file: {err}")
                        })?;
    Ok(())
  } else {
    Err("no local path is provided".to_string())
  }
}

fn restore_data(mut args: Args) -> Fail
{
  #[cfg(feature = "delay")]
  sleep(Duration::from_secs(1));
  randomly_fail()?;
  let local_path = args.nth(1);
  if let Some(local_path) = local_path {
    let (daydirname, dirname, filename) = split_daydir_dir_file(&local_path)?;
    let mut stdout = std::io::stdout();
    let mut restore_file =
      File::open(format!("{PLUGIN_DIR}/{daydirname}/{dirname}/{filename}"))
        .map_err(|err| format!("could not create restore file: {err}"))?;
    loop {
      let mut b = [0; 1024];
      let count = restore_file.read(&mut b)
        .map_err(|err| format!("could not read from restore file: {err}"))?;
      if count == 0 {
        break;
      }
      stdout.write_all(&b[..count])
            .map_err(|err| format!("could not write data to stdout: {err}"))?;
    }
    stdout.flush().map_err(|err| format!("could not flush stdout: {err}"))?;
    Ok(())
  } else {
    Err("no local path is provided".to_string())
  }
}

fn plugin_api_version(mut args: Args) -> Fail
{
  println!("0.5.0");
  Ok(())
}

fn delete_backup(mut args: Args) -> Fail
{
  let dirname = args.nth(1);
  if let Some(dirname) = dirname {
    if dirname.len() < 8 {
      return Err("invalid dirname".to_string());
    }
    let daydirname = &dirname[..8];
    remove_dir_all(format!("{PLUGIN_DIR}/{daydirname}/{dirname}"))
      .map_err(|err| format!("could not delete backup: {err}"))?;
    Ok(())
  } else {
    Err("no timestamp is provided".to_string())
  }
}

fn version(args: Args) -> Fail
{
  println!("rplugin version 0.0.1");
  Ok(())
}

#[rustfmt::skip]
fn entry(mut args: Args) -> Fail
{
  log(format!("{args:?}"));
  if let Some(subcommand) = args.next() {
    match subcommand.as_str() {
      "setup_plugin_for_backup"    => setup_plugin_for_backup(args),
      "setup_plugin_for_restore"   => setup_plugin_for_restore(args),
      "cleanup_plugin_for_backup"  => cleanup_plugin_for_backup(args),
      "cleanup_plugin_for_restore" => cleanup_plugin_for_restore(args),
      "backup_file"                => backup_file(args),
      "restore_file"               => restore_file(args),
      "backup_data"                => backup_data(args),
      "restore_data"               => restore_data(args),
      "delete_backup"              => delete_backup(args),
      "plugin_api_version"         => plugin_api_version(args),
      "--version"                  => version(args),
      _                            => Err(format!("unknown subcommand: {subcommand}")),
    }
  } else {
    Err("no subcommand was provided".to_string())
  }
}

fn main() -> ExitCode
{
  let ret;
  let mut args = args();
  let _ = args.next().expect("program name is provided");
  match entry(args) {
    Err(e) => {
      eprintln!("rplugin: error: {e}");
      ret = ExitCode::FAILURE
    }
    Ok(()) => ret = ExitCode::SUCCESS,
  }
  #[cfg(feature = "linger")]
  sleep(Duration::from_secs(2));
  ret
}
