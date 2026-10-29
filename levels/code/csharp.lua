-- Learn to Code: the C# track. Each level teaches one idea with a short lesson and a few
-- lines of code with blanks marked {1}, {2} and so on. A blank lists the answers that work,
-- a hint and the mistakes people often make, each with a message that explains it. The output
-- is what the finished code prints or does.
-- The lines run inside Main with System, System.Collections.Generic and System.Linq in use.
-- Answers are compared inside the game; nothing typed is ever run.

return {
  language = [==[C#]==],
  id = [==[csharp]==],
  levels = {
    {
      name = [==[Hello, output]==], topic = [==[Output]==],
      lesson = { [==[Console.WriteLine prints a line.]==] },
      code = { [==[Console.{1}("Hello, world");]==] },
      blanks = {
        { answers = { [==[WriteLine]==] }, hint = [==[Write, then Line, each with a capital.]==], wrong = { { [==[writeline]==], [==[C# names are case sensitive: WriteLine.]==] }, { [==[Println]==], [==[Println is Go. C# uses WriteLine.]==] } } },
      },
      output = { [==[Hello, world]==] },
    },
    {
      name = [==[Let the compiler choose]==], topic = [==[Variables]==],
      lesson = { [==[var lets the compiler pick the type from the value.]==] },
      code = { [==[{1} volts = 12;]==], [==[Console.WriteLine(volts / 2);]==] },
      blanks = {
        { answers = { [==[var]==], [==[int]==], [==[long]==] }, hint = [==[The word that lets C# work out the type (or a whole number type).]==], wrong = { { [==[let]==], [==[let is JavaScript. C# uses var.]==] }, { [==[auto]==], [==[auto is C++. C# uses var.]==] } } },
      },
      output = { [==[6]==] },
    },
    {
      name = [==[Filling in text]==], topic = [==[Strings]==],
      lesson = { [==[A $ before a string fills in values in braces.]==] },
      code = { [==[int volts = 5;]==], [==[Console.WriteLine({1}"{volts} V");]==] },
      blanks = {
        { answers = { [==[$]==] }, hint = [==[One sign before the quote.]==], wrong = { { [==[f]==], [==[f-strings are Python. C# uses $.]==] } } },
      },
      output = { [==[5 V]==] },
    },
    {
      name = [==[Too hot?]==], topic = [==[Conditions]==],
      lesson = { [==[if runs its block when the test in brackets is true.]==] },
      code = { [==[int temp = 72;]==], [==[if (temp {1} 70) Console.WriteLine("Too hot");]==] },
      blanks = {
        { answers = { [==[>]==], [==[>=]==], [==[!=]==] }, hint = [==[Which sign tests for more than?]==], wrong = { { [==[=]==], [==[One = assigns. To compare use > here.]==] } } },
      },
      output = { [==[Too hot]==] },
    },
    {
      name = [==[Every pin]==], topic = [==[Loops]==],
      lesson = { [==[foreach visits every item: foreach (var p in pins).]==] },
      code = { [==[int[] pins = { 2, 3, 5 };]==], [==[{1} (var p in pins) Console.WriteLine(p);]==] },
      blanks = {
        { answers = { [==[foreach]==] }, hint = [==[One word: for each.]==], wrong = { { [==[for]==], [==[for needs a counter. foreach walks the items.]==] } } },
      },
      output = { [==[2]==], [==[3]==], [==[5]==] },
    },
    {
      name = [==[Twice as much]==], topic = [==[Methods]==],
      lesson = { [==[return hands a value back from a method.]==] },
      code = { [==[static int Twice(int n) { {1} n * 2; }]==] },
      blanks = {
        { answers = { [==[return]==] }, hint = [==[The keyword that gives a value back.]==], wrong = { { [==[Return]==], [==[Keywords are lower case: return.]==] } } },
      },
      output = { [==[Twice(21) is 42]==] },
    },
    {
      name = [==[A growing list]==], topic = [==[Collections]==],
      lesson = { [==[List<T> grows as items are added with Add.]==] },
      code = { [==[var pins = new List<int>();]==], [==[pins.{1}(13);]==], [==[Console.WriteLine(pins.Count);]==] },
      blanks = {
        { answers = { [==[Add]==] }, hint = [==[Add, with a capital A.]==], wrong = { { [==[add]==], [==[C# methods start with a capital: Add.]==] }, { [==[Append]==], [==[Append makes a new sequence in LINQ. List uses Add.]==] } } },
      },
      output = { [==[1]==] },
    },
    {
      name = [==[Parts by name]==], topic = [==[Collections]==],
      lesson = { [==[Dictionary<K, V> maps keys to values.]==] },
      code = { [==[var volts = new Dictionary<string, double> { ["diode"] = 0.7 };]==], [==[Console.WriteLine(volts[{1}]);]==] },
      blanks = {
        { answers = { [==["diode"]==] }, hint = [==[The key as text in double quotes.]==], wrong = { { [==[diode]==], [==[The keys are strings, so the key needs quotes.]==] } } },
      },
      output = { [==[0.7]==] },
    },
    {
      name = [==[When things go wrong]==], topic = [==[Exceptions]==],
      lesson = { [==[try runs code that might throw. catch handles the exception.]==] },
      code = { [==[try { int.Parse("five"); }]==], [==[{1} (FormatException) { Console.WriteLine("Not a number"); }]==] },
      blanks = {
        { answers = { [==[catch]==] }, hint = [==[The keyword that handles an exception.]==], wrong = { { [==[except]==], [==[except is Python. C# uses catch.]==] } } },
      },
      output = { [==[Not a number]==] },
    },
    {
      name = [==[Properties]==], topic = [==[Classes and objects]==],
      lesson = { [==[A property looks like a field but runs get and set code. { get; set; } makes a simple one.]==] },
      code = { [==[class Led {]==], [==[    public int Pin { {1}; set; }]==], [==[}]==] },
      blanks = {
        { answers = { [==[get]==] }, hint = [==[The part that reads the value.]==], wrong = { { [==[read]==], [==[C# properties use get and set.]==] } } },
      },
      output = { [==[new Led { Pin = 13 }.Pin is 13]==] },
    },
    {
      name = [==[A kind of sensor]==], topic = [==[Inheritance]==],
      lesson = { [==[A class inherits with a colon: class Thermometer : Sensor.]==] },
      code = { [==[class Sensor { public int Read() => 0; }]==], [==[class Thermometer {1} Sensor { }]==] },
      blanks = {
        { answers = { [==[:]==] }, hint = [==[One punctuation mark.]==], wrong = { { [==[extends]==], [==[extends is Java. C# uses a colon.]==] } } },
      },
      output = { [==[new Thermometer().Read() is 0]==] },
    },
    {
      name = [==[Many shapes, one call]==], topic = [==[Polymorphism]==],
      lesson = { [==[A virtual method can be replaced with override in a derived class.]==] },
      code = { [==[class Shape { public {1} double Area() => 0; }]==], [==[class Square : Shape { public override double Area() => 4; }]==] },
      blanks = {
        { answers = { [==[virtual]==], [==[abstract]==] }, hint = [==[The word that allows overriding.]==], wrong = { { [==[override]==], [==[override goes on the derived method. The base one is virtual.]==] } } },
      },
      output = { [==[Shape s = new Square(); s.Area() is 4]==] },
    },
    {
      name = [==[A promise to provide]==], topic = [==[Interfaces]==],
      lesson = { [==[An interface lists members a class must provide. Names start with I by custom.]==] },
      code = { [==[interface IShape { double Area(); }]==], [==[class Square : {1} { public double Area() => 4; }]==] },
      blanks = {
        { answers = { [==[IShape]==] }, hint = [==[The interface's name.]==], wrong = { { [==[Shape]==], [==[The interface here is called IShape.]==] } } },
      },
      output = { [==[Square is an IShape]==] },
    },
    {
      name = [==[Filtering with LINQ]==], topic = [==[LINQ]==],
      lesson = { [==[LINQ queries collections. Where keeps the items that pass a test.]==] },
      code = { [==[var pins = new[] { 2, 3, 5, 7 };]==], [==[var big = pins.{1}(p => p > 3);]==], [==[Console.WriteLine(string.Join(" ", big));]==] },
      blanks = {
        { answers = { [==[Where]==] }, hint = [==[Where, with a capital W.]==], wrong = { { [==[Select]==], [==[Select changes items. Where keeps or drops them.]==] }, { [==[filter]==], [==[filter is JavaScript. LINQ uses Where.]==] } } },
      },
      output = { [==[5 7]==] },
    },
    {
      name = [==[Changing with LINQ]==], topic = [==[LINQ]==],
      lesson = { [==[Select makes a new item from each one.]==] },
      code = { [==[var doubled = pins.{1}(p => p * 2);]==] },
      blanks = {
        { answers = { [==[Select]==] }, hint = [==[Select, with a capital S.]==], wrong = { { [==[map]==], [==[map is JavaScript and Rust. LINQ uses Select.]==] }, { [==[Where]==], [==[Where filters. Select changes.]==] } } },
      },
      output = { [==[4 6 10 14]==] },
    },
    {
      name = [==[Waiting without blocking]==], topic = [==[Async]==],
      lesson = { [==[Inside an async method, await waits for a Task without blocking the thread.]==] },
      code = { [==[async Task<string> Load() {]==], [==[    var text = {1} File.ReadAllTextAsync("notes.txt");]==], [==[    return text;]==], [==[}]==] },
      blanks = {
        { answers = { [==[await]==] }, hint = [==[The keyword that waits for a Task.]==], wrong = { { [==[async]==], [==[async marks the method. Inside it you await.]==] } } },
      },
      output = { [==[Load() gives a Task of the text]==] },
    },
    {
      name = [==[Maybe nothing]==], topic = [==[Null safety]==],
      lesson = { [==[?. reaches a member only when the object is not null; otherwise the result is null.]==] },
      code = { [==[Led? led = null;]==], [==[Console.WriteLine(led{1}Pin ?? -1);]==] },
      blanks = {
        { answers = { [==[?.]==] }, hint = [==[A question mark and a dot.]==], wrong = { { [==[.]==], [==[led.Pin would throw on null. ?. checks first.]==] } } },
      },
      output = { [==[-1]==] },
    },
  },
}
