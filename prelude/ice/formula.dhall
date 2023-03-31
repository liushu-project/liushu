let Prelude = ../package.dhall

let ice =
      Prelude.Formula::{
      , id = "ice"
      , name = Some "雾凇拼音"
      , dictionaries = [ "8105.dict.tsv" ]
      }

in  ice
