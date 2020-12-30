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
use crate::Person::{First, Second, Third};
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

/// Часть речи
#[derive(Debug, PartialEq)]
pub enum PartOfSpeech {
    /// Прилагательное
    Adjective,
    /// Наречие
    Adverb,
    /// Местоименное наречие
    AdverbPronominal,
    /// Числительное-прилагательное
    AdjectiveNumeral,
    /// Местоимение-прилагательное
    AdjectivePronoun,
    /// Часть композита - сложного слова
    Composite,
    /// Союз
    Conjunction,
    /// Междометие
    Interjection,
    /// Числительное
    Numeral,
    /// Частица
    Particle,
    /// Предлог
    Preposition,
    /// Существительное
    Noun,
    /// Местоимение-существительное
    AdjectiveNoun,
    /// Глагол
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
    /// Падеж
    Case(Case),
    /// Время глаголов
    Tense(Tense),
    /// Число
    Plurality(Plurality),
    /// Наклонение глагола
    Mood(Mood),
    /// Форма прилагательного
    Adjective(Adjective),
    /// Степень сравнения
    ComparativeDegree(ComparativeDegree),
    /// Лицо глагола
    Person(Person),
    /// Род
    Gender(Gender),
    /// Вид глагола
    PerfectiveAspect(PerfectiveAspect),
    /// Залог
    Voice(Voice),
    /// Одушевленность
    Animacy(Animacy),
    /// Переходность
    Transitivity(Transitivity),
    /// Прочие обозначения
    Other(Other),
}
impl fmt::Display for Fact {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

/// Падеж
#[derive(Debug, PartialEq)]
pub enum Case {
    /// Именительный
    Nominative,
    /// Родительный
    Genitive,
    /// Дательный
    Dative,
    /// Винительный
    Accusative,
    /// Творительный
    Instrumental,
    /// Предложный
    Prepositional,
    /// Партитив (второй родительный)
    Partitive,
    /// Местный (второй предложный)
    Locative,
    /// Звательный
    Vocative,
}

/// Время глагола
#[derive(Debug, PartialEq)]
pub enum Tense {
    /// Настоящее время
    Present,
    /// Непрошедшее время
    Inpresent,
    /// Прошедшее время
    Past,
}

/// Число
#[derive(Debug, PartialEq)]
pub enum Plurality {
    /// Множественное
    Plural,
    /// Единственное
    Singular,
}

/// Наклонение глагола
#[derive(Debug, PartialEq)]
pub enum Mood {
    /// Деепричастие
    Gerunds,
    /// Инфинитив
    Infinitive,
    /// Причастие
    Participle,
    /// Изьявительное наклонение
    Indicative,
    /// Повелительное наклонение
    Imperative,
}

/// Форма прилагательного
#[derive(Debug, PartialEq)]
pub enum Adjective {
    /// Краткое
    Short,
    /// Полное
    Long,
    /// Притяжательное
    Possessive,
}

/// Степень сравнения
#[derive(Debug, PartialEq)]
pub enum ComparativeDegree {
    /// Превосходная
    Superlative,
    /// Сравнительная
    Comparative,
}

/// Лицо глагола
#[derive(Debug, PartialEq)]
pub enum Person {
    /// 1-е лицо
    First,
    /// 2-е лицо
    Second,
    /// 3-е лицо
    Third,
}

/// Род
#[derive(Debug, PartialEq)]
pub enum Gender {
    /// Мужской
    Masculine,
    /// Женский
    Feminine,
    /// Средний
    Neuter,
}

/// Вид глагола
#[derive(Debug, PartialEq)]
pub enum PerfectiveAspect {
    /// Совершенный
    Perfective,
    /// Несовершенный
    Imperfective,
}

/// Залог
#[derive(Debug, PartialEq)]
pub enum Voice {
    /// Страдательный залог
    Passive,
    /// Действительный залог
    Active,
}

/// Одушевленность
#[derive(Debug, PartialEq)]
pub enum Animacy {
    /// Одушевленное
    Animate,
    /// Неодушевленное
    Inanimate,
}

/// Переходность глагола
#[derive(Debug, PartialEq)]
pub enum Transitivity {
    /// Переходный глагол
    Transitive,
    /// Непереходный глагол
    Intransitive,
}

/// Прочие обозначения
#[derive(Debug, PartialEq)]
pub enum Other {
    /// Вводное слово
    Parenthesis,
    /// Географическое название
    Geo,
    /// Образование формы затруднено
    Awkward,
    /// Имя собственное
    ProperNoun,
    /// Искаженная форма
    Distorted,
    /// Общая форма мужского и женского рода
    CommonForm,
    /// Обсценная лексика
    Obscene,
    /// Отчество
    Patronymic,
    /// Предикатив
    Predicative,
    /// Разговорная форма
    Informal,
    /// Редко встречающееся слово
    Rare,
    /// Сокращение
    Abbreviation,
    /// Устаревшая форма
    Obsolete,
    /// Фамилия
    FamilyName,
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
