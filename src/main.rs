use lexer::Lexer;
use morpher::{
    word::{Word, WordGender},
    word_stem::{NounDeclination, WordLemma, WordLemmaAttributes},
    Morpher,
};
use parser::Parser;

pub mod lexer;
pub mod morpher;
pub mod parser;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let word_lemmas = [
        WordLemma::new(
            "mens",
            WordLemmaAttributes::Noun {
                gender: WordGender::Feminine,
                declination: NounDeclination::A,
            },
        ),
        WordLemma::new("ama", WordLemmaAttributes::Verb),
    ];

    let all_words: Vec<Word> = word_lemmas
        .iter()
        .flat_map(|lemma| lemma.get_all_words())
        .collect();

    let test = "mensam amo.";

    let mut lexer = Lexer::new(test);
    let tokens = lexer.tokenize()?;
    println!("{:?}", tokens);

    let morpher = Morpher::new(&tokens, &all_words);
    let sentences = morpher.morph()?;
    for sentence in &sentences {
        println!("{}", sentence);
    }

    for sentence in &sentences {
        let mut parser = Parser::new(&sentence);
        parser.parse()?;
    }

    Ok(())
}
