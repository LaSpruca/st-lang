trait Printable begin
    func print with end
end

struct Person{ T: Core/fmt/Format } with Printable begin