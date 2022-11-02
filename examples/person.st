struct Person
  name core/String
  age core/Int

  func new
    name core/String
    age core/Int
  begin
    *name $name core/set
    *age $age core/set
  end

  func print
  begin
      $name $age "%s is %d years old" std/io::println
  end
end

-- The main function
func main
    let person Person
    "Nathan" 16 Person::new *person set

    "Josh" 17 Person::new Person::print
    *person Person::print
end