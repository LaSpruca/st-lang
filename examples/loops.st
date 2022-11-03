package
    name loops
    version "1.0.0"
    require std with "*"
end
using std/prelude

func main with begin
    -- Print all numbers in an array
    let numbers Array{Int}
    [1, 2, 3, 4, 5, 6, 7] *numbers set

    loop_over number with *numbers begin
        *number putintln
    end

    -- Count to 20
    let count Int
    0 *count set

    loop 20 *count < begin
        count putintln
        *count 1 + *count set
    end
end