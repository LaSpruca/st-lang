using Std::CLI;

data Args {
    all: Bool;
}

func fib (Int32) (Int32) {
    |? < 1 -> 1
    |? _ -> |= n
            n - 1
            |> fib
            |> + n
    \?
}


func main () () {
    Std::CLI::parse_args |= args |! panic;
    println "Enter a number";
    read_int |= n;
    args
    |? { all: true } -> 0..n |> map func {
            |= i
            |> fib
            |> println "The #{i}th fibonacci number is #{.}"
        }
    |? _ -> fib n
        |> println "The #{n}th fibonacci number is #{.}"
    \?;

    ()
}