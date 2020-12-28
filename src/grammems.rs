use crate::error::*;
use crate::Adjective::{Long, Possessive, Short};
use crate::Animacy::{Animate, Inanimate};
use crate::Case::*;
use crate::ComparativeDegree::{Comparative, Superlative};
use crate::Gender::{Feminine, Masculine, Neuter};
use crate::Mood::{Gerunds, Imperative, Indicative, Infinitive, Participle};
use crate::Other::{
    Abbreviation, Awkward, CommonForm, Distorted, FamilyName, Geo, Informal, Obscene, Obsolete,
    Parenthesis, Patronymic, Predicative, ProperNoun, Rare,
};
use crate::PerfectiveAspect::{Imperfective, Perfective};
use crate::Plurality::{Plural, Singular};
use crate::Tense::{Inpresent, Past, Present};
use crate::Transitivity::{Intransitive, Transitive};
use crate::VerbPerson::{First, Second, Third};
use crate::Voice::{Active, Passive};
use std::fmt;
use std::str::FromStr;

#[derive(Debug)]
/// Represent grammems for [`Stemming`](./struct.Stemming.html)
pub struct Grammem {
    /// Part of speech of [`Stemming`](./struct.Stemming.html)
    pub part_of_speech: PartOfSpeech,
    /// Parsed `Vec` of [`Facts`](./enum.Fact.html)
    pub facts: Vec<Fact>,
    /// Non-parsed list of grammems from mystem
    pub facts_raw: Vec<String>,
}

#[derive(Debug, PartialEq)]
pub enum PartOfSpeech {
    /// прилагательное
    Adjective,
    /// наречие
    Adverb,
    /// местоименное наречие
    AdverbPronominal,
    /// числительное-прилагательное
    AdjectiveNumeral,
    /// местоимение-прилагательное
    AdjectivePronoun,
    /// часть композита - сложного слова
    Composite,
    /// союз
    Conjunction,
    /// междометие
    Interjection,
    /// числительное
    Numeral,
    /// частица
    Particle,
    /// предлог
    Preposition,
    /// существительное
    Noun,
    /// местоимение-существительное
    AdjectiveNoun,
    /// глагол
    Verb,
}
impl FromStr for PartOfSpeech {
    type Err = crate::AppError;
    fn from_str(input: &str) -> Result<PartOfSpeech, Self::Err> {
        match input {
            "A" => Ok(PartOfSpeech::Adjective),
            "ADV" => Ok(PartOfSpeech::Adverb),
            "ADVPRO" => Ok(PartOfSpeech::AdverbPronominal),
            "ANUM" => Ok(PartOfSpeech::AdjectiveNumeral),
            "APRO" => Ok(PartOfSpeech::AdjectivePronoun),
            "COM" => Ok(PartOfSpeech::Composite),
            "CONJ" => Ok(PartOfSpeech::Conjunction),
            "INTJ" => Ok(PartOfSpeech::Interjection),
            "NUM" => Ok(PartOfSpeech::Numeral),
            "PART" => Ok(PartOfSpeech::Particle),
            "PR" => Ok(PartOfSpeech::Preposition),
            "S" => Ok(PartOfSpeech::Noun),
            "SPRO" => Ok(PartOfSpeech::AdjectiveNoun),
            "V" => Ok(PartOfSpeech::Verb),
            _ => Err(AppError::PartOfSpeechError("Failed to get Part of Speech.")),
        }
    }
}
impl fmt::Display for PartOfSpeech {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, PartialEq)]
pub enum Fact {
    Case(Case),
    Tense(Tense),
    Plurality(Plurality),
    Mood(Mood),
    Adjective(Adjective),
    ComparativeDegree(ComparativeDegree),
    Person(VerbPerson),
    Gender(Gender),
    PerfectiveAspect(PerfectiveAspect),
    Voice(Voice),
    Animacy(Animacy),
    Transitivity(Transitivity),
    Other(Other),
}
impl fmt::Display for Fact {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, PartialEq)]
pub enum Case {
    Nominative,    //именительный
    Genitive,      //родительный
    Dative,        //дательный
    Accusative,    //винительный
    Instrumental,  //творительный
    Prepositional, //предложный
    Partitive,     //партитив (второй родительный)
    Locative,      //местный (второй предложный)
    Vocative,      //звательный
}

#[derive(Debug, PartialEq)]
pub enum Tense {
    Present,   //настоящее
    Inpresent, //непрошедшее
    Past,      //прошедшее
}

#[derive(Debug, PartialEq)]
pub enum Plurality {
    Plural,   //настоящее
    Singular, //непрошедшее
}

#[derive(Debug, PartialEq)]
pub enum Mood {
    Gerunds,    //деепричастие
    Infinitive, //инфинитив
    Participle, //причастие
    Indicative, //изьявительное наклонение
    Imperative, //повелительное наклонение
}

#[derive(Debug, PartialEq)]
pub enum Adjective {
    Short,      //Краткое
    Long,       //Полное
    Possessive, //притяжательное
}

#[derive(Debug, PartialEq)]
pub enum ComparativeDegree {
    Superlative, //превосходная
    Comparative, //сравнительная
}

#[derive(Debug, PartialEq)]
pub enum VerbPerson {
    First,  //1-е лицо
    Second, //2-е лицо
    Third,  //3-е лицо
}

#[derive(Debug, PartialEq)]
pub enum Gender {
    Masculine, //мужской род
    Feminine,  //женский род
    Neuter,    //средний род
}

#[derive(Debug, PartialEq)]
pub enum PerfectiveAspect {
    Perfective,   //совершенный
    Imperfective, //несовершенный
}

#[derive(Debug, PartialEq)]
pub enum Voice {
    Passive, //страдательный залог
    Active,  //действительный залог
}

#[derive(Debug, PartialEq)]
pub enum Animacy {
    Animate,   //одушевленное
    Inanimate, //неодушевленное
}

#[derive(Debug, PartialEq)]
pub enum Transitivity {
    Transitive,   //переходный глагол
    Intransitive, //непереходный глагол
}

#[derive(Debug, PartialEq)]
pub enum Other {
    Parenthesis,  //вводное слово
    Geo,          //географическое название
    Awkward,      //образование формы затруднено
    ProperNoun,   //имя собственное
    Distorted,    //искаженная форма
    CommonForm,   //общая форма мужского и женского рода
    Obscene,      //обсценная лексика
    Patronymic,   //отчество
    Predicative,  //предикатив
    Informal,     //разговорная форма
    Rare,         //редко встречающееся слово
    Abbreviation, //сокращение
    Obsolete,     //устаревшая форма
    FamilyName,   //фамилия
}

impl FromStr for Fact {
    type Err = crate::AppError;
    fn from_str(input: &str) -> Result<Fact, Self::Err> {
        match input {
            "nom" => Ok(Fact::Case(Nominative)),
            "gen" => Ok(Fact::Case(Genitive)),
            "dat" => Ok(Fact::Case(Dative)),
            "acc" => Ok(Fact::Case(Accusative)),
            "ins" => Ok(Fact::Case(Instrumental)),
            "abl" => Ok(Fact::Case(Prepositional)),
            "part" => Ok(Fact::Case(Partitive)),
            "loc" => Ok(Fact::Case(Locative)),
            "voc" => Ok(Fact::Case(Vocative)),
            "praes" => Ok(Fact::Tense(Present)),
            "inpraes" => Ok(Fact::Tense(Inpresent)),
            "praet" => Ok(Fact::Tense(Past)),
            "sg" => Ok(Fact::Plurality(Singular)),
            "pl" => Ok(Fact::Plurality(Plural)),
            "ger" => Ok(Fact::Mood(Gerunds)),
            "inf" => Ok(Fact::Mood(Infinitive)),
            "partcp" => Ok(Fact::Mood(Participle)),
            "indic" => Ok(Fact::Mood(Indicative)),
            "imper" => Ok(Fact::Mood(Imperative)),
            "brev" => Ok(Fact::Adjective(Short)),
            "plen" => Ok(Fact::Adjective(Long)),
            "poss" => Ok(Fact::Adjective(Possessive)),
            "supr" => Ok(Fact::ComparativeDegree(Superlative)),
            "comp" => Ok(Fact::ComparativeDegree(Comparative)),
            "1p" => Ok(Fact::Person(First)),
            "2p" => Ok(Fact::Person(Second)),
            "3p" => Ok(Fact::Person(Third)),
            "m" => Ok(Fact::Gender(Masculine)),
            "f" => Ok(Fact::Gender(Feminine)),
            "n" => Ok(Fact::Gender(Neuter)),
            "pf" => Ok(Fact::PerfectiveAspect(Perfective)),
            "ipf" => Ok(Fact::PerfectiveAspect(Imperfective)),
            "act" => Ok(Fact::Voice(Active)),
            "pass" => Ok(Fact::Voice(Passive)),
            "anim" => Ok(Fact::Animacy(Animate)),
            "inan" => Ok(Fact::Animacy(Inanimate)),
            "tran" => Ok(Fact::Transitivity(Transitive)),
            "intr" => Ok(Fact::Transitivity(Intransitive)),
            "parenth" => Ok(Fact::Other(Parenthesis)),
            "geo" => Ok(Fact::Other(Geo)),
            "awkw" => Ok(Fact::Other(Awkward)),
            "persn" => Ok(Fact::Other(ProperNoun)),
            "dist" => Ok(Fact::Other(Distorted)),
            "mf" => Ok(Fact::Other(CommonForm)),
            "obsc" => Ok(Fact::Other(Obscene)),
            "patrn" => Ok(Fact::Other(Patronymic)),
            "praed" => Ok(Fact::Other(Predicative)),
            "inform" => Ok(Fact::Other(Informal)),
            "rare" => Ok(Fact::Other(Rare)),
            "abbr" => Ok(Fact::Other(Abbreviation)),
            "obsol" => Ok(Fact::Other(Obsolete)),
            "famn" => Ok(Fact::Other(FamilyName)),
            //_ => Ok(Fact::Case(Vocative)),
            _ => Err(AppError::GrammemError("Failed to get Grammem.")),
        }
    }
}
