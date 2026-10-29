-- Learn to Code: the Python track. Each level teaches one idea with a short lesson and a few
-- lines of code with blanks marked {1}, {2} and so on. A blank lists the answers that work,
-- a hint and the mistakes people often make, each with a message that explains it. The output
-- is what the finished code prints or does.
-- In Python single and double quotes are the same and spaces outside quotes never matter.
-- Answers are compared inside the game; nothing typed is ever run.

return {
  language = [==[Python]==],
  id = [==[python]==],
  quotes_equal = true,
  levels = {
    {
      name = [==[Hello, output]==], topic = [==[Output]==],
      lesson = { [==[print shows a value on the screen.]==], [==[Text is called a string and goes inside quotes.]==] },
      code = { [==[print({1})]==] },
      blanks = {
        { answers = { [==["Hello, world"]==] }, hint = [==[Put the text Hello, world inside quotes.]==], wrong = { { [==[Hello, world]==], [==[Text needs quotes around it. Without them Python reads it as names.]==] } } },
      },
      output = { [==[Hello, world]==] },
    },
    {
      name = [==[Numbers in boxes]==], topic = [==[Variables and types]==],
      lesson = { [==[A variable is a name for a value: volts = 12 stores 12 in volts.]==], [==[12 is an int (a whole number). 0.5 is a float. An int times a float gives a float.]==] },
      code = { [==[volts = {1}]==], [==[amps = 0.5]==], [==[watts = volts * amps]==], [==[print(watts)]==] },
      blanks = {
        { answers = { [==[12]==], [==[12.0]==] }, hint = [==[The output is 6.0 and amps is 0.5. What times 0.5 makes 6?]==], wrong = { { [==["12"]==], [==[In quotes 12 is text. Text times a number is an error, so leave the quotes off.]==] } } },
      },
      output = { [==[6.0]==] },
    },
    {
      name = [==[Text to number]==], topic = [==[Types]==],
      lesson = { [==[Values have types. int() turns text such as "42" into a whole number.]==], [==[str() turns a number back into text.]==] },
      code = { [==[n = {1}("42")]==], [==[print(n + 1)]==] },
      blanks = {
        { answers = { [==[int]==] }, hint = [==[The function named after the whole number type.]==], wrong = { { [==[str]==], [==[str keeps it as text. Text plus 1 is an error.]==] }, { [==[float]==], [==[float gives 42.0, so the output would be 43.0.]==] } } },
      },
      output = { [==[43]==] },
    },
    {
      name = [==[Asking a question]==], topic = [==[Input]==],
      lesson = { [==[input() shows a prompt and waits for a line to be typed. It always gives back text.]==], [==[Here the person types Ada.]==] },
      code = { [==[name = {1}("Name? ")]==], [==[print("Hi", name)]==] },
      blanks = {
        { answers = { [==[input]==] }, hint = [==[The built-in function that reads a line from the keyboard.]==], wrong = { { [==[read]==], [==[Python reads a line with input().]==] }, { [==[scanf]==], [==[scanf is C. Python reads a line with input().]==] } } },
      },
      output = { [==[Name? Ada]==], [==[Hi Ada]==] },
    },
    {
      name = [==[Remainders]==], topic = [==[Operators]==],
      lesson = { [==[+ - * and / do sums. // divides and drops the remainder. % gives just the remainder.]==] },
      code = { [==[print(17 {1} 5)]==] },
      blanks = {
        { answers = { [==[%]==] }, hint = [==[17 shared into 5s leaves 2 over. Which operator gives what is left over?]==], wrong = { { [==[/]==], [==[17 / 5 is 3.4. The remainder comes from %.]==] }, { [==[//]==], [==[// gives the whole part, 3. The remainder comes from %.]==] } } },
      },
      output = { [==[2]==] },
    },
    {
      name = [==[Too hot?]==], topic = [==[Conditions]==],
      lesson = { [==[if runs its block only when the test is true. else runs when it is not.]==], [==[Tests compare values: > greater, < less, >= at least and == equal.]==] },
      code = { [==[temp = 72]==], [==[if temp {1} 70:]==], [==[    print("Too hot")]==], [==[else:]==], [==[    print("Fine")]==] },
      blanks = {
        { answers = { [==[>]==], [==[>=]==], [==[!=]==] }, hint = [==[72 is more than 70. Which sign tests for more than?]==], wrong = { { [==[=>]==], [==[Python writes greater than or equal as >=, with the = second.]==] }, { [==[=]==], [==[One = stores a value. To compare use > here (or == for equal).]==] }, { [==[<]==], [==[72 < 70 is false, so the else block would print Fine.]==] } } },
      },
      output = { [==[Too hot]==] },
    },
    {
      name = [==[Grade bands]==], topic = [==[Conditions]==],
      lesson = { [==[elif adds another test between if and else. The first true test wins.]==] },
      code = { [==[mark = 65]==], [==[if mark >= 70:]==], [==[    print("First")]==], [==[{1} mark >= 60:]==], [==[    print("2:1")]==], [==[else:]==], [==[    print("Lower")]==] },
      blanks = {
        { answers = { [==[elif]==] }, hint = [==[Python's short word for else if.]==], wrong = { { [==[else if]==], [==[Python joins them into one word: elif.]==] } } },
      },
      output = { [==[2:1]==] },
    },
    {
      name = [==[Counting up]==], topic = [==[Loops]==],
      lesson = { [==[for runs its block once for each value. range(n) gives 0, 1 and so on up to n - 1.]==] },
      code = { [==[for i in range({1}):]==], [==[    print(i)]==] },
      blanks = {
        { answers = { [==[3]==], [==[0,3]==] }, hint = [==[The output stops at 2. range stops one before its number.]==], wrong = { { [==[2]==], [==[range(2) gives 0 and 1 only: it stops one before the number.]==] }, { [==[4]==], [==[range(4) gives 0, 1, 2 and 3: one too many.]==] } } },
      },
      output = { [==[0]==], [==[1]==], [==[2]==] },
    },
    {
      name = [==[Countdown]==], topic = [==[Loops]==],
      lesson = { [==[while repeats its block as long as its test stays true.]==], [==[Change the value it tests inside the loop. Otherwise it never ends.]==] },
      code = { [==[count = 3]==], [==[while count {1} 0:]==], [==[    print(count)]==], [==[    count = count {2} 1]==], [==[print("Go")]==] },
      blanks = {
        { answers = { [==[>]==], [==[!=]==] }, hint = [==[Keep going while count is still above 0.]==], wrong = { { [==[>=]==], [==[With >= the loop also prints 0 before Go.]==] } } },
        { answers = { [==[-]==] }, hint = [==[The count goes down by one each time.]==], wrong = { { [==[+]==], [==[Adding makes count grow for ever, so the loop never ends.]==] } } },
      },
      output = { [==[3]==], [==[2]==], [==[1]==], [==[Go]==] },
    },
    {
      name = [==[Twice as much]==], topic = [==[Functions]==],
      lesson = { [==[def makes a function: a named block you can call with values.]==], [==[return hands a value back to whoever called the function.]==] },
      code = { [==[def double(n):]==], [==[    {1} n * 2]==], [==[]==], [==[print(double(21))]==] },
      blanks = {
        { answers = { [==[return]==] }, hint = [==[The function must give its answer back to print.]==], wrong = { { [==[print]==], [==[print would show 42 but give back nothing, so the outer print shows None. return hands the value back.]==] }, { [==[Return]==], [==[Python keywords are lower case: return.]==] } } },
      },
      output = { [==[42]==] },
    },
    {
      name = [==[A friendly default]==], topic = [==[Functions]==],
      lesson = { [==[A parameter can have a default value, used when the caller leaves it out.]==] },
      code = { [==[def greet(name, greeting={1}):]==], [==[    return greeting + ", " + name]==], [==[]==], [==[print(greet("Ada"))]==] },
      blanks = {
        { answers = { [==["Hello"]==] }, hint = [==[The output starts with Hello, so that is the default text.]==], wrong = { { [==[Hello]==], [==[The default is text, so it needs quotes.]==] } } },
      },
      output = { [==[Hello, Ada]==] },
    },
    {
      name = [==[A row of pins]==], topic = [==[Lists]==],
      lesson = { [==[A list holds values in order: pins = [2, 3, 5, 7].]==], [==[append adds to the end. Indexes start at 0 and -1 means the last item.]==] },
      code = { [==[pins = [2, 3, 5, 7]]==], [==[pins.{1}(11)]==], [==[print(pins[{2}])]==] },
      blanks = {
        { answers = { [==[append]==] }, hint = [==[The method that adds one item to the end of a list.]==], wrong = { { [==[add]==], [==[Lists have no add method. Sets do; lists use append.]==] }, { [==[push]==], [==[push is JavaScript. Python lists use append.]==] } } },
        { answers = { [==[4]==], [==[-1]==] }, hint = [==[11 is now the fifth item. Counting from 0, what is its index?]==], wrong = { { [==[5]==], [==[Indexes start at 0, so the fifth item is at 4. Index 5 is past the end.]==] } } },
      },
      output = { [==[11]==] },
    },
    {
      name = [==[Slices]==], topic = [==[Lists]==],
      lesson = { [==[xs[a:b] gives a new list from index a up to but not including index b.]==] },
      code = { [==[pins = [2, 3, 5, 7, 11]]==], [==[print(pins[{1}])]==] },
      blanks = {
        { answers = { [==[1:3]==] }, hint = [==[3 is at index 1 and 5 at index 2. The end index is one past the last item you want.]==], wrong = { { [==[1:2]==], [==[A slice stops before its end index, so 1:2 gives only [3].]==] }, { [==[2:3]==], [==[Indexes start at 0, so index 2 is 5. 3 is at index 1.]==] } } },
      },
      output = { [==[[3, 5]]==] },
    },
    {
      name = [==[Shouting text]==], topic = [==[Strings]==],
      lesson = { [==[Strings have methods: name.upper() gives a new string in capitals.]==], [==[len counts the characters in a string.]==] },
      code = { [==[name = "led"]==], [==[print(name.{1}())]==], [==[print(len(name))]==] },
      blanks = {
        { answers = { [==[upper]==] }, hint = [==[The method's name is the opposite of lower.]==], wrong = { { [==[toUpperCase]==], [==[That is Java. Python's method is upper.]==] }, { [==[capitalize]==], [==[capitalize makes only the first letter a capital: Led.]==] } } },
      },
      output = { [==[LED]==], [==[3]==] },
    },
    {
      name = [==[Filling in text]==], topic = [==[Strings]==],
      lesson = { [==[An f-string puts values straight into text: f"{volts} V".]==], [==[The f goes just before the opening quote.]==] },
      code = { [==[volts = 5]==], [==[print({1}"{volts} V")]==] },
      blanks = {
        { answers = { [==[f]==], [==[F]==] }, hint = [==[One letter before the quote turns on the filling in.]==], wrong = { { [==[$]==], [==[$ is how JavaScript and C# fill in text. Python puts an f before the quote.]==] } } },
      },
      output = { [==[5 V]==] },
    },
    {
      name = [==[Parts by name]==], topic = [==[Dictionaries]==],
      lesson = { [==[A dictionary maps keys to values: {"name": "diode"}.]==], [==[Look a value up with its key in square brackets.]==] },
      code = { [==[part = {"name": "diode", "volts": 0.7}]==], [==[print(part[{1}])]==] },
      blanks = {
        { answers = { [==["volts"]==] }, hint = [==[The key whose value is 0.7, written as it is in the dictionary.]==], wrong = { { [==[volts]==], [==[The keys here are strings, so the key needs its quotes too.]==] }, { [==[1]==], [==[Dictionaries are looked up by key, not by position.]==] } } },
      },
      output = { [==[0.7]==] },
    },
    {
      name = [==[When things go wrong]==], topic = [==[Error handling]==],
      lesson = { [==[try runs code that might fail. If it raises an error the matching except block runs instead of the program stopping.]==] },
      code = { [==[try:]==], [==[    n = int("five")]==], [==[{1} ValueError:]==], [==[    print("Not a number")]==] },
      blanks = {
        { answers = { [==[except]==] }, hint = [==[The keyword that catches an error in Python.]==], wrong = { { [==[catch]==], [==[catch is Java and JavaScript. Python uses except.]==] }, { [==[Except]==], [==[Python keywords are lower case: except.]==] } } },
      },
      output = { [==[Not a number]==] },
    },
    {
      name = [==[Reading a file]==], topic = [==[Files]==],
      lesson = { [==[open() opens a file. with closes it again when the block ends, even after an error.]==], [==[Here notes.txt starts with the line: Buy resistors.]==] },
      code = { [==[with {1}("notes.txt") as f:]==], [==[    print(f.readline().strip())]==] },
      blanks = {
        { answers = { [==[open]==] }, hint = [==[The built-in function that opens a file.]==], wrong = { { [==[file]==], [==[Python 3 opens files with open(). file() was removed.]==] }, { [==[read]==], [==[read is a method of an open file. First open it with open().]==] } } },
      },
      output = { [==[Buy resistors]==] },
    },
    {
      name = [==[An LED object]==], topic = [==[Classes and objects]==],
      lesson = { [==[A class describes a kind of object. __init__ runs when an object is made.]==], [==[Inside a method the object itself is called self.]==] },
      code = { [==[class Led:]==], [==[    def __init__(self, pin):]==], [==[        {1}.pin = pin]==], [==[]==], [==[red = Led(13)]==], [==[print(red.pin)]==] },
      blanks = {
        { answers = { [==[self]==] }, hint = [==[The name a method uses for its own object.]==], wrong = { { [==[this]==], [==[this is Java and C++. Python names the object self.]==] }, { [==[Led]==], [==[Led.pin would set one value shared by the whole class. Each object keeps its own on self.]==] } } },
      },
      output = { [==[13]==] },
    },
    {
      name = [==[A kind of sensor]==], topic = [==[Inheritance]==],
      lesson = { [==[A class can inherit from another by naming it in brackets. It gets all of its methods.]==] },
      code = { [==[class Sensor:]==], [==[    def read(self):]==], [==[        return 0]==], [==[]==], [==[class Thermometer({1}):]==], [==[    pass]==], [==[]==], [==[print(Thermometer().read())]==] },
      blanks = {
        { answers = { [==[Sensor]==] }, hint = [==[The class Thermometer is a kind of.]==], wrong = { { [==[sensor]==], [==[Class names are case sensitive: Sensor.]==] }, { [==[object]==], [==[Inheriting from object would leave out read, so the call would fail.]==] } } },
      },
      output = { [==[0]==] },
    },
    {
      name = [==[Many shapes, one call]==], topic = [==[Polymorphism]==],
      lesson = { [==[Different classes can answer the same method call in their own way.]==] },
      code = { [==[class Square:]==], [==[    def area(self):]==], [==[        return 4]==], [==[class Circle:]==], [==[    def area(self):]==], [==[        return 3]==], [==[for s in [Square(), Circle()]:]==], [==[    print(s.{1}())]==] },
      blanks = {
        { answers = { [==[area]==] }, hint = [==[The method both classes have.]==], wrong = { { [==[area()]==], [==[The brackets are already there after the blank.]==] } } },
      },
      output = { [==[4]==], [==[3]==] },
    },
    {
      name = [==[Squares in one line]==], topic = [==[Comprehensions]==],
      lesson = { [==[A list comprehension builds a list in one line: [expression for item in items].]==] },
      code = { [==[squares = [n * n {1} n in range(4)]]==], [==[print(squares)]==] },
      blanks = {
        { answers = { [==[for]==] }, hint = [==[The same keyword a loop starts with.]==], wrong = { { [==[in]==], [==[in is already there after n. The comprehension starts its loop with for.]==] } } },
      },
      output = { [==[[0, 1, 4, 9]]==] },
    },
  },
}
