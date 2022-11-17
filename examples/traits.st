package
    name traits
    version "1.0.0"
    require std with "*"
end

using std/prelude

trait Animal begin
    func name String with end
end

trait Greets with Animal begin
    func greet with name String end
end

struct Human with Greets begin
    name String

    func new with name String begin
        *name $name set
    end

    func Animal::name String with begin
        $name
    end

    func Greets::greet with name String begin
        [$name, *name] "%s says hello to %s" println
    end
end

struct Dog with Animal begin
    name String
   
    func new with name String begin
        *name $name set
    end

    func Animal::name String with name String begin 
        $name
    end
end

func main with begin
    let animal Animal
    "Josh" new Person *animal set

    -- Works
    animal Animal::name putstrln

    -- Typechecker has a stroke
    "Taine" animal Greets::greet

    -- Typechecker also has a stroke
    let person Person
    *animal *person set

    -- Typechecker happy happy
    *animal as Person *person set
end