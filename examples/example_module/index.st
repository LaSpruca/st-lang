
package
    name example_module 
    version "1.0.0"
    require std with "*"
    
    internal include other.st
end


func say_hello with begin
    "Hello world" say_msg
end
