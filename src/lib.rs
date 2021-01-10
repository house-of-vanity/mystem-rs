#![crate_name = "mystem"]
mod error;
mod grammems;

use serde_json::Value;
use std::io::{prelude::*, BufReader, Error, Write};
use std::str::FromStr;
use subprocess::{Popen, PopenConfig, PopenError, Redirection};
#[macro_use]
extern crate log;

pub use error::*;
pub use grammems::*;

/// A Mystem process representation
#[derive(Debug)]
pub struct MyStem {
    pub process: Popen,
}

/// Lexeme struct
#[derive(Debug)]
pub struct Lexeme {
    /// Detected lexeme
    pub lex: String,
    /// Detected grammems
    pub grammem: Grammem,
    /// Wight of Lexeme
    pub weight: f64,
}

/// Stemmed result containing `Vec` of [`mystem::Lexeme`](./struct.Lexeme.html)
#[derive(Debug)]
pub struct Stemming {
    /// Original word
    pub text: String,
    /// `Vec` of [`mystem::Lexeme`](./struct.Lexeme.html) of `text`.
    pub lex: Vec<Lexeme>,
}

impl MyStem {
    /// Returns a MyStem instance with running process
    /// of mystem binary. It keeps mystem running all the time
    /// and reuse it.
    pub fn new() -> Result<Self, AppError> {
        let p = MyStem::open_process()?;
        debug!("Mystem started with PID {}", p.pid().unwrap());
        Ok(Self { process: p })
    }

    fn open_process() -> Result<Popen, PopenError> {
        Popen::create(
            &["mystem", "-i", "--format", "json", "--eng-gr", "--weight"],
            PopenConfig {
                stdout: Redirection::Pipe,
                stdin: Redirection::Pipe,
                ..Default::default()
            },
        )
    }

    /// Terminate mystem instance.
    #[allow(dead_code)]
    pub fn terminate(&mut self) -> Result<(), Error> {
        self.process.terminate()
    }

    fn detect_grammems(&mut self, gr: String) -> Result<Grammem, AppError> {
        let mut res: Vec<String> = gr
            .split(|s| s == '=' || s == ',')
            .map(|s| s.to_string())
            .collect();
        res.retain(|x| x != "");
        Ok(Grammem {
            part_of_speech: PartOfSpeech::from_str(res[0].as_str())?,
            facts: res
                .clone()
                .split_off(1)
                .iter_mut()
                .map(|f| Fact::from_str(f).unwrap())
                .collect(),
            facts_raw: res.split_off(1),
        })
    }

    /// Returns `Vec` with [`mystem::Stemming`](./struct.Stemming.html) for each word in `text`
    /// # Examples
    ///
    /// ```rust
    ///     let mut instance = mystem::MyStem::new()?;
    ///     for stem in instance.stemming("Связался с лучшим - подохни как все.".into())? {
    ///         println!("{} is a lexeme of {}", stem.lex, stem.text)
    ///     }
    ///     // связываться is a lexeme of Связался
    ///     // с is a lexeme of с
    ///     // хороший is a lexeme of лучшим
    ///     // подыхать is a lexeme of подохни
    ///     // как is a lexeme of как
    ///     // все is a lexeme of все
    /// ```
    #[allow(unused_must_use)]
    pub fn stemming(&mut self, text: String) -> Result<Vec<Stemming>, AppError> {
        if let Some(exit_status) = self.process.poll() {
            warn!(
                "MyStem process ({:?}) exited with: {:?}. Restarting...",
                self.process.pid().unwrap(),
                exit_status
            );
            self.process = MyStem::open_process()?;
        }
        let mut clean_text = text.trim().to_string();
        for c in clean_text.clone().chars() {
            if !char::is_alphabetic(c) && c != ' ' {
                clean_text = clean_text.replace(c, "");
            }
        }
        self.process
            .stdin
            .as_ref()
            .unwrap()
            .write(clean_text.as_bytes());
        self.process.stdin.as_ref().unwrap().write("\n".as_bytes());
        let mut contents = String::new();
        let mut buf_reader =
            BufReader::with_capacity(512 * 1024, self.process.stdout.as_ref().unwrap());
        buf_reader.read_line(&mut contents);

        let mut stemmings: Vec<Stemming> = Vec::new();
        match Some(contents) {
            Some(contents) => {
                let v: Vec<Value> = match serde_json::from_str(contents.as_str()) {
                    Ok(val) => val,
                    Err(_) => return Ok(vec![]),
                };
                for i in v {
                    stemmings.push(Stemming {
                        text: i["text"].to_string().replace("\"", ""),
                        lex: {
                            i["analysis"]
                                .as_array()
                                .unwrap()
                                .iter()
                                .map(|z| Lexeme {
                                    lex: z["lex"].to_string().replace("\"", ""),
                                    grammem: self
                                        .detect_grammems(z["gr"].to_string().replace("\"", ""))
                                        .unwrap(),
                                    weight: z["wt"].as_f64().unwrap_or(1.0),
                                })
                                .collect()
                        },
                    });
                }
                Ok(stemmings)
            }
            None => return Ok(vec![]),
        }
    }
}
