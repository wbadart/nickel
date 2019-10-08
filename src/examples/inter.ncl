let id = Assume( (Bool -> Bool) /\ (Num -> Num), (fun x => x)) in
if id false then 34 else id 344
