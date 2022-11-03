package
    name functions
    require std with "*"
end

using std/prelude

func sum with
    numbers Array{Int}
begin
    let total Int

    loop element with numbers
        total *element + *total Array::push
    end

    *total
end

func main with begin
    [1, 2, 3, 4, 5, 6, 7] sum putiln
end
