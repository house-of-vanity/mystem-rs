extern crate mystem;

fn main() -> Result<(), mystem::AppError> {
    let mut instance = mystem::MyStem::new()?;
    for stem in instance.stemming("Связался с лучшим - подохни как все.".into())?
    {
        println!("{} is a lexeme of {}", stem.lex, stem.text)
    }

    #[allow(unused_must_use)]
    instance.terminate();
    Ok(())
}
