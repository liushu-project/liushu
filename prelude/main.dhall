let Prelude = ./package.dhall

let sunman = ./sunman/formula.dhall

let ice = ./ice/formula.dhall

let luna = ./luna/formula.dhall

let config
    : Prelude.Config
    = { formulas = [ sunman, ice, luna ] }

in  config
