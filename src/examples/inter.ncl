let prop = fun l => fun t =>  fun x => if t true then t x else blame[t] l in

let id = Assume( (Num -> Bool) /\ #prop , (fun x => x)) in
id 34