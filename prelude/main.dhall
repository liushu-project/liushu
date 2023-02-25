let Prelude = ./package.dhall

let sunman = ./sunman/formula.dhall

let config
    : Prelude.Config
    = { formulas = [ sunman ] }

in  config
