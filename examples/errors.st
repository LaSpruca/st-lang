module
    name errors
    version "1.0.0"

    require std with "*"
end

using std/prelude

func is_funny String error String with 
    num Int
begin
    if num [420, 69] Array::contains begin
        [num] "%s is funny"
    else
        [num] "%d is not the meaning of life" fmt error
    end
end

func main with begin
    42 is_funny
    
    recover
        let issues Array{Format}
        *issues Array::push
        issues "That wasn't very cash money of me, the function says %s" fmt
    end putstrln
end
