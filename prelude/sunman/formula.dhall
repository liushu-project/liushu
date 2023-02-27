let Prelude = ../package.dhall

let sunman
    : Prelude.Formula
    = { id = "sunman"
      , name = Some "山人全息"
      , dictionaries =
        [ "words.dict.csv", "phrases.brief.dict.csv", "phrases.core.dict.csv" ]
      }

in  sunman
