func main with
begin
    -- Create and store an anomus function
    let print_hello core/Function{ Tuple{}, Tuple{} }
    func with name core/String begin *name "Hello %s" print end *print_hello set
    
    -- Call said function
    *print_hello call
end