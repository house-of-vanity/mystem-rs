extern crate mystem;

use mystem::Other::Obscene;

#[allow(unused_must_use)]
fn main() -> Result<(), mystem::AppError> {
    let mut instance = mystem::MyStem::new()?;
    for stem in instance.stemming("Связался с лучшим - подохни как все, Говноед.".into())?
    {
        println!(
            "'{}' most likely is a '{}' and lexeme is '{}'.{}{}",
            stem.text,
            stem.lex[0].grammem.part_of_speech,
            stem.lex[0].lex,
            {
                match stem.lex[0]
                    .grammem
                    .facts
                    .contains(&mystem::Fact::Other(Obscene))
                {
                    true => " Obscene lexis.",
                    false => "",
                }
            },
            {
                match stem.lex.len() {
                    0 | 1 => "".to_string(),
                    x if x > 1 => format!(" Also has {} found lexems.", x),
                    _ => unreachable!(),
                }
            }
        )
    }
    instance.terminate();
    Ok(())
}
