# MyStem Rust Wrapper

Rust wrapper for the Yandex MyStem 3.1 morpholocial analyzer of the Russian language.

#### System Requrements
The wrapper was tested on Ubuntu Linux 18.04+, Windows 10. 
Mystem binary should be accessible via PATH so manual installation is required. [MyStem Web Site](https://yandex.ru/dev/mystem/)

### A Quick Example

```rust
let mut instance = mystem::MyStem::new()?;
for stem in instance.stemming("Связался с лучшим - подохни как все.".into())? {
    println!(
        "'{}' most likely is a '{}' and lexeme is '{}'.",
        stem.text,
        stem.lex[0].grammem.part_of_speech,
        stem.lex[0].lex
    )
}

//'Связался' most likely is a 'Verb' and lexeme is 'связываться'.
//'с' most likely is a 'Preposition' and lexeme is 'с'.
//'лучшим' most likely is a 'Adjective' and lexeme is 'хороший'.
//'подохни' most likely is a 'Verb' and lexeme is 'подыхать'.
//'как' most likely is a 'Conjunction' and lexeme is 'как'.
//'все' most likely is a 'AdjectivePronoun' and lexeme is 'весь'.

```