package
    name using_modules
    version "1.0.0"
    require example_module with "./example_module"
end

func main with begin
    example_module/say_hello
end