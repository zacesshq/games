-- Learn to Code: the Go track. Each level teaches one idea with a short lesson and a few
-- lines of code with blanks marked {1}, {2} and so on. A blank lists the answers that work,
-- a hint and the mistakes people often make, each with a message that explains it. The output
-- is what the finished code prints or does.
-- The lines run inside func main with fmt imported unless a level shows otherwise.
-- Answers are compared inside the game; nothing typed is ever run.

return {
  language = [==[Go]==],
  id = [==[go]==],
  levels = {
    {
      name = [==[Hello, output]==], topic = [==[Output]==],
      lesson = { [==[fmt.Println prints its arguments and ends the line.]==] },
      code = { [==[fmt.{1}("Hello, world")]==] },
      blanks = {
        { answers = { [==[Println]==] }, hint = [==[Print line, with a capital P.]==], wrong = { { [==[println]==], [==[Exported names start with a capital: Println.]==] }, { [==[Print]==], [==[Print would leave the line open.]==] } } },
      },
      output = { [==[Hello, world]==] },
    },
    {
      name = [==[Short declarations]==], topic = [==[Variables]==],
      lesson = { [==[:= declares a variable and gives it a value. Go works out the type.]==] },
      code = { [==[volts {1} 12]==], [==[fmt.Println(volts / 2)]==] },
      blanks = {
        { answers = { [==[:=]==] }, hint = [==[A colon and an equals sign.]==], wrong = { { [==[=]==], [==[= assigns to a variable that already exists. := declares a new one.]==] } } },
      },
      output = { [==[6]==] },
    },
    {
      name = [==[Named types]==], topic = [==[Types]==],
      lesson = { [==[var name type = value declares with an explicit type.]==] },
      code = { [==[var volts {1} = 12]==] },
      blanks = {
        { answers = { [==[int]==], [==[int32]==], [==[int64]==], [==[uint8]==], [==[uint]==] }, hint = [==[A whole number type.]==], wrong = { { [==[Int]==], [==[Go types are lower case: int.]==] } } },
      },
      output = { [==[volts is an int]==] },
    },
    {
      name = [==[Too hot?]==], topic = [==[Conditions]==],
      lesson = { [==[if needs no brackets around its test, but always braces around its block.]==] },
      code = { [==[temp := 72]==], [==[if temp {1} 70 {]==], [==[    fmt.Println("Too hot")]==], [==[}]==] },
      blanks = {
        { answers = { [==[>]==], [==[>=]==], [==[!=]==] }, hint = [==[Which sign tests for more than?]==], wrong = { { [==[=]==], [==[One = assigns. To compare use > here.]==] } } },
      },
      output = { [==[Too hot]==] },
    },
    {
      name = [==[The only loop]==], topic = [==[Loops]==],
      lesson = { [==[Go has one loop keyword, for, used for counting loops and while-style loops alike.]==] },
      code = { [==[{1} i := 0; i < 3; i++ {]==], [==[    fmt.Println(i)]==], [==[}]==] },
      blanks = {
        { answers = { [==[for]==] }, hint = [==[Go's only loop keyword.]==], wrong = { { [==[while]==], [==[Go has no while. for does every loop.]==] } } },
      },
      output = { [==[0]==], [==[1]==], [==[2]==] },
    },
    {
      name = [==[Typed functions]==], topic = [==[Functions]==],
      lesson = { [==[The result type follows the parameter list.]==] },
      code = { [==[func twice(n int) {1} {]==], [==[    return n * 2]==], [==[}]==] },
      blanks = {
        { answers = { [==[int]==] }, hint = [==[The type of n * 2.]==], wrong = { { [==[-> int]==], [==[Go writes the result type straight after the brackets, with no arrow.]==] } } },
      },
      output = { [==[twice(21) is 42]==] },
    },
    {
      name = [==[Two results]==], topic = [==[Functions]==],
      lesson = { [==[A function can return more than one value. List the types in brackets.]==] },
      code = { [==[func divmod(a, b int) {1} {]==], [==[    return a / b, a % b]==], [==[}]==] },
      blanks = {
        { answers = { [==[(int, int)]==], [==[(int,int)]==] }, hint = [==[Two ints, in brackets.]==], wrong = { { [==[int, int]==], [==[Several result types go in brackets: (int, int).]==] } } },
      },
      output = { [==[divmod(17, 5) is 3, 2]==] },
    },
    {
      name = [==[Growing a slice]==], topic = [==[Collections]==],
      lesson = { [==[A slice grows with append, which gives back the new slice.]==] },
      code = { [==[pins := []int{2, 3}]==], [==[pins = {1}(pins, 5)]==], [==[fmt.Println(pins)]==] },
      blanks = {
        { answers = { [==[append]==] }, hint = [==[The built-in that adds to a slice.]==], wrong = { { [==[push]==], [==[Go has no push. Use pins = append(pins, 5).]==] } } },
      },
      output = { [==[[2 3 5]]==] },
    },
    {
      name = [==[Parts by name]==], topic = [==[Collections]==],
      lesson = { [==[A map holds keys with values: map[string]float64.]==] },
      code = { [==[volts := {1}[string]float64{"diode": 0.7}]==], [==[fmt.Println(volts["diode"])]==] },
      blanks = {
        { answers = { [==[map]==] }, hint = [==[The keyword for a map type.]==], wrong = { { [==[dict]==], [==[dict is Python. Go uses map.]==] } } },
      },
      output = { [==[0.7]==] },
    },
    {
      name = [==[Errors are values]==], topic = [==[Error handling]==],
      lesson = { [==[Functions return an error as their last result. Check it straight away.]==] },
      code = { [==[n, err := strconv.Atoi("five")]==], [==[if err {1} nil {]==], [==[    fmt.Println("Not a number")]==], [==[}]==] },
      blanks = {
        { answers = { [==[!=]==] }, hint = [==[Not equal to nil means there was an error.]==], wrong = { { [==[==]==], [==[err == nil means it worked. There is an error when err != nil.]==] } } },
      },
      output = { [==[Not a number]==] },
    },
    {
      name = [==[Grouping fields]==], topic = [==[Structs]==],
      lesson = { [==[A struct groups named fields.]==] },
      code = { [==[type Led {1} {]==], [==[    Pin int]==], [==[}]==] },
      blanks = {
        { answers = { [==[struct]==] }, hint = [==[The keyword for a group of fields.]==], wrong = { { [==[class]==], [==[Go has no classes. Use struct.]==] } } },
      },
      output = { [==[Led{Pin: 13}]==] },
    },
    {
      name = [==[Methods]==], topic = [==[Methods]==],
      lesson = { [==[A method has a receiver before its name: func (l Led) On().]==] },
      code = { [==[func (l {1}) Describe() string {]==], [==[    return fmt.Sprint("LED on pin ", l.Pin)]==], [==[}]==] },
      blanks = {
        { answers = { [==[Led]==], [==[*Led]==] }, hint = [==[The type the method belongs to.]==], wrong = { { [==[self]==], [==[Go names the receiver in brackets with its type: (l Led).]==] } } },
      },
      output = { [==[LED on pin 13]==] },
    },
    {
      name = [==[Shared behaviour]==], topic = [==[Interfaces]==],
      lesson = { [==[An interface lists methods. Any type with those methods satisfies it, with no declaration needed.]==] },
      code = { [==[type Light {1} {]==], [==[    On() bool]==], [==[}]==] },
      blanks = {
        { answers = { [==[interface]==] }, hint = [==[The keyword for a set of methods.]==], wrong = { { [==[trait]==], [==[trait is Rust. Go uses interface.]==] } } },
      },
      output = { [==[any type with On() bool is a Light]==] },
    },
    {
      name = [==[Doing two things at once]==], topic = [==[Goroutines]==],
      lesson = { [==[go before a call runs it in a new goroutine, at the same time as the rest.]==] },
      code = { [==[{1} worker()]==] },
      blanks = {
        { answers = { [==[go]==] }, hint = [==[Two letters.]==], wrong = { { [==[async]==], [==[Go starts concurrent work with go.]==] } } },
      },
      output = { [==[worker runs concurrently]==] },
    },
    {
      name = [==[Talking between goroutines]==], topic = [==[Channels]==],
      lesson = { [==[A channel passes values between goroutines. make creates one and <- sends or receives.]==] },
      code = { [==[ch := make(chan int)]==], [==[go func() { ch {1} 5 }()]==], [==[fmt.Println(<-ch)]==] },
      blanks = {
        { answers = { [==[<-]==] }, hint = [==[The arrow that points into the channel.]==], wrong = { { [==[->]==], [==[Go's channel arrow is <-.]==] } } },
      },
      output = { [==[5]==] },
    },
    {
      name = [==[Clean up later]==], topic = [==[Defer]==],
      lesson = { [==[defer runs a call when the surrounding function returns, such as closing a file.]==] },
      code = { [==[f, _ := os.Open("notes.txt")]==], [==[{1} f.Close()]==] },
      blanks = {
        { answers = { [==[defer]==] }, hint = [==[Put off until the function ends.]==], wrong = { { [==[finally]==], [==[Go has no finally. defer does the job.]==] } } },
      },
      output = { [==[the file closes when the function ends]==] },
    },
  },
}
