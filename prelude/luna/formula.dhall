let Prelude = ../package.dhall

let ice
    : Prelude.Formula.Type
    = { id = "luna"
      , name = Some "朙月拼音"
      , use_hmm = True
      , dictionaries = [] : List Text
      }

in  ice
