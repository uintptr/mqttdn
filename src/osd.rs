use std::{
    io::Write,
    path::PathBuf,
    process::{Command, Stdio},
};

use log::error;
use which::which;

use crate::error::{Error, Result};

const OSD_PROGRAM_NAME: &str = "aosd_cat";
const OSD_TEXT_SIZE: i32 = 90;
const OSD_TEXT_COLOR: &str = "white";
const DEFAULT_WIDTH: i32 = 3840;
const DEFAULT_HEIGHT: i32 = 2160;

pub struct Osd {
    program: PathBuf,
    text_size: i32,
    text_color: String,
}

fn get_dimention() -> (i32, i32) {
    (DEFAULT_WIDTH, DEFAULT_HEIGHT)
}

impl Osd {
    pub fn new() -> Result<Self> {
        let program = match which(OSD_PROGRAM_NAME) {
            Ok(v) => v,
            Err(_) => {
                error!("{OSD_PROGRAM_NAME} was not found in PATH");
                return Err(Error::ProgramNotFound);
            }
        };

        Ok(Self {
            program,
            text_size: OSD_TEXT_SIZE,
            text_color: OSD_TEXT_COLOR.to_string(),
        })
    }

    pub fn display<S>(&self, text: S) -> Result<()>
    where
        S: AsRef<str>,
    {
        let (width, height) = get_dimention();

        let text_width = text.as_ref().len() as i32 * self.text_size;

        let y = (height / 2) - (self.text_size / 4);
        let x = (width / 2) - (text_width / 4);

        let mut child = Command::new(&self.program)
            .arg("-x")
            .arg(x.to_string())
            .arg("-y")
            .arg(format!("-{y}"))
            .arg("-n")
            .arg(self.text_size.to_string())
            .arg("-R")
            .arg(&self.text_color)
            .stdin(Stdio::piped())
            .spawn()?;

        if let Some(stdin) = child.stdin.as_mut() {
            if let Err(e) = stdin.write_all(text.as_ref().as_bytes()) {
                error!("{e}");
            }
        }

        match child.wait() {
            Ok(exit) => match exit.success() {
                true => Ok(()),
                false => Err(Error::ExecFailure),
            },
            Err(e) => Err(e.into()),
        }
    }
}
