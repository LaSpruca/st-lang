package
    name lp
    version "0.1.0"
    require std with "*"
end
using std/prelude

func main with begin
    let numbers Array{Int}

    [1, 2, 3, 4, 5, 6, 7] *numbers set

    loop_over number with *numbers begin
        *number putintln
    end
end