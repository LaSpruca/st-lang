package
    name structs
    version "1.0.0"
    require std with "*"
end

using std/prelude

-- Person Struct
struct Person with begin
    name String
    internal age Int

    func new with
        name String
        age Int
    begin
        name $name set
        age $age set
    end

    func print with begin
        if $age 1 = begin
            [$name] "%s is 1 year old" println
        else
            [$name, $age] "%s is %d years old" println
        end
    end
end

func main with begin
    "Nathan" 16 Person::new Person::print
    "Daniel" 1 Person::new Person::print
end
