-- Learn to Code: the Kotlin track. Each level teaches one idea with a short lesson and a few
-- lines of code with blanks marked {1}, {2} and so on. A blank lists the answers that work,
-- a hint and the mistakes people often make, each with a message that explains it. The output
-- is what the finished code prints or does.
-- The lines run inside fun main unless a level shows otherwise.
-- Answers are compared inside the game; nothing typed is ever run.

return {
  language = [==[Kotlin]==],
  id = [==[kotlin]==],
  levels = {
    {
      name = [==[Hello, output]==], topic = [==[Output]==],
      lesson = { [==[println prints a line.]==] },
      code = { [==[println({1})]==] },
      blanks = {
        { answers = { [==["Hello, world"]==] }, hint = [==[The text in double quotes.]==], wrong = { { [==['Hello, world']==], [==[Single quotes hold one Char. Text needs double quotes.]==] }, { [==[Hello, world]==], [==[Text needs double quotes.]==] } } },
      },
      output = { [==[Hello, world]==] },
    },
    {
      name = [==[Can it change?]==], topic = [==[Variables]==],
      lesson = { [==[val cannot be given a new value. var can.]==] },
      code = { [==[{1} count = 0]==], [==[count += 1]==], [==[println(count)]==] },
      blanks = {
        { answers = { [==[var]==] }, hint = [==[The word for a variable that changes.]==], wrong = { { [==[val]==], [==[val cannot change, so count += 1 would fail.]==] } } },
      },
      output = { [==[1]==] },
    },
    {
      name = [==[Typed numbers]==], topic = [==[Types]==],
      lesson = { [==[Types follow a colon and start with a capital: Int, Double, String.]==] },
      code = { [==[val volts: {1} = 12]==], [==[println(volts / 2)]==] },
      blanks = {
        { answers = { [==[Int]==], [==[Long]==], [==[Short]==] }, hint = [==[The whole number type, with a capital.]==], wrong = { { [==[int]==], [==[Kotlin types start with a capital: Int.]==] } } },
      },
      output = { [==[6]==] },
    },
    {
      name = [==[If gives a value]==], topic = [==[Conditions]==],
      lesson = { [==[if is an expression in Kotlin, so it can give a value.]==] },
      code = { [==[val temp = 72]==], [==[val label = if (temp > 70) "hot" {1} "fine"]==], [==[println(label)]==] },
      blanks = {
        { answers = { [==[else]==] }, hint = [==[The other branch.]==], wrong = { { [==[?:]==], [==[?: is the Elvis operator. if needs else here.]==] } } },
      },
      output = { [==[hot]==] },
    },
    {
      name = [==[Choosing by value]==], topic = [==[Conditions]==],
      lesson = { [==[when picks the first branch that matches, a tidy switch.]==] },
      code = { [==[val mark = 65]==], [==[{1} (mark) {]==], [==[    in 70..100 -> println("First")]==], [==[    in 60..69 -> println("2:1")]==], [==[    else -> println("Lower")]==], [==[}]==] },
      blanks = {
        { answers = { [==[when]==] }, hint = [==[The keyword for picking a branch.]==], wrong = { { [==[switch]==], [==[switch is Java and C. Kotlin uses when.]==] } } },
      },
      output = { [==[2:1]==] },
    },
    {
      name = [==[Counting up]==], topic = [==[Loops]==],
      lesson = { [==[0 until 3 counts 0, 1 and 2. 0..3 includes 3.]==] },
      code = { [==[for (i in 0 {1} 3) println(i)]==] },
      blanks = {
        { answers = { [==[until]==] }, hint = [==[The word that stops before the end.]==], wrong = { { [==[..]==], [==[0..3 includes 3, printing four numbers.]==] } } },
      },
      output = { [==[0]==], [==[1]==], [==[2]==] },
    },
    {
      name = [==[Typed functions]==], topic = [==[Functions]==],
      lesson = { [==[A function's return type follows a colon after its parameters.]==] },
      code = { [==[fun twice(n: Int){1} Int = n * 2]==] },
      blanks = {
        { answers = { [==[:]==] }, hint = [==[One colon.]==], wrong = { { [==[->]==], [==[-> is for lambdas. A return type follows a colon.]==] } } },
      },
      output = { [==[twice(21) is 42]==] },
    },
    {
      name = [==[Lists]==], topic = [==[Collections]==],
      lesson = { [==[listOf makes a list that cannot change. mutableListOf one that can.]==] },
      code = { [==[val pins = {1}(2, 3, 5)]==], [==[println(pins.size)]==] },
      blanks = {
        { answers = { [==[listOf]==], [==[mutableListOf]==] }, hint = [==[list, then Of.]==], wrong = { { [==[arrayOf]==], [==[arrayOf makes an Array, not a List.]==] }, { [==[list]==], [==[The function is listOf.]==] } } },
      },
      output = { [==[3]==] },
    },
    {
      name = [==[Filling in text]==], topic = [==[Strings]==],
      lesson = { [==[A $ in a string fills in a value.]==] },
      code = { [==[val volts = 5]==], [==[println("{1}volts V")]==] },
      blanks = {
        { answers = { [==[$]==] }, hint = [==[One sign before the name.]==], wrong = { { [==[#]==], [==[Kotlin fills in with $.]==] } } },
      },
      output = { [==[5 V]==] },
    },
    {
      name = [==[Maybe nothing]==], topic = [==[Null safety]==],
      lesson = { [==[A type that may be null ends in ?. Kotlin will not let you use it without checking.]==] },
      code = { [==[val name: String{1} = null]==] },
      blanks = {
        { answers = { [==[?]==] }, hint = [==[One question mark.]==], wrong = { { [==[!]==], [==[! is not part of a type. Nullable types end in ?.]==] } } },
      },
      output = { [==[name may be null]==] },
    },
    {
      name = [==[Safe calls]==], topic = [==[Null safety]==],
      lesson = { [==[?. calls a member only when the value is not null; otherwise it gives null.]==] },
      code = { [==[val name: String? = null]==], [==[println(name{1}length)]==] },
      blanks = {
        { answers = { [==[?.]==] }, hint = [==[A question mark and a dot.]==], wrong = { { [==[.]==], [==[name.length would not compile: name might be null.]==] }, { [==[!!.]==], [==[!!. would crash on null. ?. is safe.]==] } } },
      },
      output = { [==[null]==] },
    },
    {
      name = [==[A default for null]==], topic = [==[Null safety]==],
      lesson = { [==[The Elvis operator ?: gives a value to use when the left side is null.]==] },
      code = { [==[val n = name?.length {1} 0]==], [==[println(n)]==] },
      blanks = {
        { answers = { [==[?:]==] }, hint = [==[A question mark and a colon.]==], wrong = { { [==[??]==], [==[?? is C# and JavaScript. Kotlin uses ?:.]==] } } },
      },
      output = { [==[0]==] },
    },
    {
      name = [==[Classes in one line]==], topic = [==[Classes and objects]==],
      lesson = { [==[Parameters marked val in the class header become properties.]==] },
      code = { [==[class Led({1} pin: Int)]==], [==[println(Led(13).pin)]==] },
      blanks = {
        { answers = { [==[val]==], [==[var]==] }, hint = [==[Make it a property.]==], wrong = { { [==[int]==], [==[The type already follows the colon. val makes pin a property.]==] } } },
      },
      output = { [==[13]==] },
    },
    {
      name = [==[Data classes]==], topic = [==[Classes and objects]==],
      lesson = { [==[A data class gets equals, hashCode, toString and copy for free.]==] },
      code = { [==[{1} class Part(val name: String, val volts: Double)]==], [==[println(Part("diode", 0.7))]==] },
      blanks = {
        { answers = { [==[data]==] }, hint = [==[The word before class.]==], wrong = { { [==[record]==], [==[record is Java. Kotlin uses data class.]==] } } },
      },
      output = { [==[Part(name=diode, volts=0.7)]==] },
    },
    {
      name = [==[Open to inherit]==], topic = [==[Inheritance]==],
      lesson = { [==[Kotlin classes are final unless marked open.]==] },
      code = { [==[{1} class Sensor { fun read() = 0 }]==], [==[class Thermometer : Sensor()]==] },
      blanks = {
        { answers = { [==[open]==], [==[abstract]==] }, hint = [==[The word that allows subclasses.]==], wrong = { { [==[public]==], [==[public is the default. A class must be open to be inherited.]==] } } },
      },
      output = { [==[Thermometer().read() is 0]==] },
    },
    {
      name = [==[Lambdas]==], topic = [==[Functions]==],
      lesson = { [==[map runs a lambda on every item. it names the single parameter.]==] },
      code = { [==[val doubled = listOf(2, 3).{1} { it * 2 }]==], [==[println(doubled)]==] },
      blanks = {
        { answers = { [==[map]==] }, hint = [==[The function that changes every item.]==], wrong = { { [==[forEach]==], [==[forEach gives back nothing. map gives the new list.]==] } } },
      },
      output = { [==[[4, 6]]==] },
    },
  },
}
