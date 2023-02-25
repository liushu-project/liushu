let Prelude = ../package.dhall

let sunman
    : Prelude.Formula
    = { id = "sunman"
      , name = Some "山人全息"
      , dictionaries = [ "words.dict.csv", "phrase.brief.dict.csv", "phrases.core.dict.csv" ]
      }

in  sunman
