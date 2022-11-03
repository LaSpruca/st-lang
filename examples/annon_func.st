package
    name annon_func
    require std with "*"
end

using std/prelude

func main with begin
    [1, 2, 3, 4, 5, 6, 7]
    func _ Int with item Int begin 2 $item pow end 
    Iter{Int}::map
    "%d %d %d %d %d %d %d" println
end
