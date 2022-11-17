-- A simple hello world package

package
    name hello_world
    require std with "*"
end

using std/prelude

func main with begin
    "Hello world" putsln
end