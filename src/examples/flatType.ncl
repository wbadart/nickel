let alwaysTrue = fun l => fun t => let boolT = Assume(Bool, t) in 
    if boolT then boolT else blame[boolT] l in
Assume(#alwaysTrue, true)
