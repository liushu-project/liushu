let Formula = ./Formula/package.dhall
let Config
    : Type
    = { formulas : List Formula.Type }

in  { Config, Formula }
