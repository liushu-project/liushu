let Formula
    : Type
    = { id : Text, name : Optional Text, dictionaries : List Text }

let Config
    : Type
    = { formulas : List Formula }

in  { Formula, Config }
