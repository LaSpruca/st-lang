struct Person
  name String
  age int

  func new
    name String
    age int
  begin
    *name $name set
    *age $age set
  end

  func print
  begin
      "{} is {} years old" $name $age print
  end
end

-- The main function
func main
    let person Person
    "Nathan" 16 Person::new *person set

    "Josh" 17 Person::new Person::print
    *person Person::print
end