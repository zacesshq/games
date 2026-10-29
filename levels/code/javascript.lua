-- Learn to Code: the JavaScript track. Each level teaches one idea with a short lesson and a few
-- lines of code with blanks marked {1}, {2} and so on. A blank lists the answers that work,
-- a hint and the mistakes people often make, each with a message that explains it. The output
-- is what the finished code prints or does.
-- Single and double quotes are the same here and spaces outside quotes never matter.
-- Answers are compared inside the game; nothing typed is ever run.

return {
  language = [==[JavaScript]==],
  id = [==[javascript]==],
  quotes_equal = true,
  levels = {
    {
      name = [==[Hello, output]==], topic = [==[Output]==],
      lesson = { [==[console.log prints a value.]==] },
      code = { [==[console.{1}("Hello, world");]==] },
      blanks = {
        { answers = { [==[log]==] }, hint = [==[Three letters: what a ship keeps.]==], wrong = { { [==[print]==], [==[print is Python. JavaScript uses console.log.]==] }, { [==[println]==], [==[println is Java. JavaScript uses console.log.]==] } } },
      },
      output = { [==[Hello, world]==] },
    },
    {
      name = [==[Values that change]==], topic = [==[Variables]==],
      lesson = { [==[let makes a variable that can change. const makes one that cannot be given a new value.]==] },
      code = { [==[{1} pins = 4;]==], [==[pins = 5;]==], [==[console.log(pins);]==] },
      blanks = {
        { answers = { [==[let]==], [==[var]==] }, hint = [==[The word for a variable that can change.]==], wrong = { { [==[const]==], [==[const cannot be given a new value, so pins = 5 would fail. let can.]==] }, { [==[int]==], [==[JavaScript does not name types. Use let.]==] } } },
      },
      output = { [==[5]==] },
    },
    {
      name = [==[What kind of value?]==], topic = [==[Types]==],
      lesson = { [==[typeof tells you a value's type: "number", "string", "boolean" and so on.]==] },
      code = { [==[console.log(typeof {1});]==] },
      blanks = {
        { answers = { [==["5"]==] }, hint = [==[Write 5 so that it is text.]==], wrong = { { [==[5]==], [==[5 without quotes is a number, so typeof gives number.]==] } } },
      },
      output = { [==[string]==] },
    },
    {
      name = [==[Strict equality]==], topic = [==[Operators]==],
      lesson = { [==[== converts types before comparing, so 5 == "5" is true. === compares without converting.]==] },
      code = { [==[console.log(5 {1} "5");]==] },
      blanks = {
        { answers = { [==[===]==] }, hint = [==[Three equals signs.]==], wrong = { { [==[==]==], [==[== converts "5" to 5 first, so it gives true.]==] } } },
      },
      output = { [==[false]==] },
    },
    {
      name = [==[Too hot?]==], topic = [==[Conditions]==],
      lesson = { [==[if runs its block when the test in brackets is true.]==] },
      code = { [==[const temp = 72;]==], [==[if (temp {1} 70) {]==], [==[  console.log("Too hot");]==], [==[}]==] },
      blanks = {
        { answers = { [==[>]==], [==[>=]==], [==[!==]==] }, hint = [==[Which sign tests for more than?]==], wrong = { { [==[=]==], [==[One = stores a value. To compare use > here (or === for equal).]==] } } },
      },
      output = { [==[Too hot]==] },
    },
    {
      name = [==[Counting up]==], topic = [==[Loops]==],
      lesson = { [==[A for loop declares its counter with let.]==] },
      code = { [==[for ({1} i = 0; i < 3; i++) {]==], [==[  console.log(i);]==], [==[}]==] },
      blanks = {
        { answers = { [==[let]==] }, hint = [==[The word that declares a changing variable.]==], wrong = { { [==[int]==], [==[JavaScript does not name types. Use let.]==] }, { [==[const]==], [==[const cannot change, so i++ would fail.]==] } } },
      },
      output = { [==[0]==], [==[1]==], [==[2]==] },
    },
    {
      name = [==[Arrow functions]==], topic = [==[Functions]==],
      lesson = { [==[An arrow function is a short way to write a function: n => n * 2.]==] },
      code = { [==[const double = n {1} n * 2;]==], [==[console.log(double(21));]==] },
      blanks = {
        { answers = { [==[=>]==] }, hint = [==[An equals sign and a greater than sign.]==], wrong = { { [==[->]==], [==[JavaScript arrows are written =>.]==] } } },
      },
      output = { [==[42]==] },
    },
    {
      name = [==[A row of pins]==], topic = [==[Arrays]==],
      lesson = { [==[push adds an item to the end of an array.]==] },
      code = { [==[const pins = [2, 3, 5];]==], [==[pins.{1}(7);]==], [==[console.log(pins.length);]==] },
      blanks = {
        { answers = { [==[push]==] }, hint = [==[The method that adds to the end.]==], wrong = { { [==[append]==], [==[append is Python. Arrays use push.]==] }, { [==[add]==], [==[add is for a Set. Arrays use push.]==] } } },
      },
      output = { [==[4]==] },
    },
    {
      name = [==[Doubling every pin]==], topic = [==[Arrays]==],
      lesson = { [==[map makes a new array by running a function on every item.]==] },
      code = { [==[const pins = [2, 3];]==], [==[console.log(pins.{1}(n => n * 2));]==] },
      blanks = {
        { answers = { [==[map]==] }, hint = [==[The method that transforms every item.]==], wrong = { { [==[forEach]==], [==[forEach runs the function but gives back nothing. map gives the new array.]==] }, { [==[filter]==], [==[filter keeps or drops items. map changes them.]==] } } },
      },
      output = { [==[[ 4, 6 ]]==] },
    },
    {
      name = [==[Filling in text]==], topic = [==[Strings]==],
      lesson = { [==[A template literal uses backticks and puts values in with ${...}.]==] },
      code = { [==[const volts = 5;]==], [==[console.log(`{1}{volts} V`);]==] },
      blanks = {
        { answers = { [==[$]==] }, hint = [==[One sign before the opening brace.]==], wrong = { { [==[#]==], [==[JavaScript fills in with ${...}.]==] }, { [==[f]==], [==[f-strings are Python. JavaScript uses ${...} inside backticks.]==] } } },
      },
      output = { [==[5 V]==] },
    },
    {
      name = [==[Parts as objects]==], topic = [==[Objects]==],
      lesson = { [==[An object holds named properties. A dot reaches one.]==] },
      code = { [==[const part = { name: "diode", volts: 0.7 };]==], [==[console.log(part.{1});]==] },
      blanks = {
        { answers = { [==[volts]==] }, hint = [==[The property that holds 0.7.]==], wrong = { { [==["volts"]==], [==[After a dot the name has no quotes. part["volts"] would work too.]==] } } },
      },
      output = { [==[0.7]==] },
    },
    {
      name = [==[When things go wrong]==], topic = [==[Error handling]==],
      lesson = { [==[try runs code that might throw. catch receives the error.]==] },
      code = { [==[try {]==], [==[  JSON.parse("{");]==], [==[} {1} (e) {]==], [==[  console.log("Bad JSON");]==], [==[}]==] },
      blanks = {
        { answers = { [==[catch]==] }, hint = [==[The keyword that catches an error.]==], wrong = { { [==[except]==], [==[except is Python. JavaScript uses catch.]==] } } },
      },
      output = { [==[Bad JSON]==] },
    },
    {
      name = [==[An LED object]==], topic = [==[Classes]==],
      lesson = { [==[A class has a constructor that runs when new makes an object.]==] },
      code = { [==[class Led {]==], [==[  {1}(pin) { this.pin = pin; }]==], [==[}]==], [==[console.log(new Led(13).pin);]==] },
      blanks = {
        { answers = { [==[constructor]==] }, hint = [==[The special method that sets up a new object.]==], wrong = { { [==[__init__]==], [==[__init__ is Python. JavaScript uses constructor.]==] }, { [==[init]==], [==[JavaScript's setup method is constructor.]==] } } },
      },
      output = { [==[13]==] },
    },
    {
      name = [==[A kind of sensor]==], topic = [==[Inheritance]==],
      lesson = { [==[A class extends another to get its methods.]==] },
      code = { [==[class Sensor { read() { return 0; } }]==], [==[class Thermometer {1} Sensor {}]==], [==[console.log(new Thermometer().read());]==] },
      blanks = {
        { answers = { [==[extends]==] }, hint = [==[The keyword for inheriting.]==], wrong = { { [==[implements]==], [==[JavaScript classes only extend.]==] }, { [==[:]==], [==[A colon is C++ and C#. JavaScript uses extends.]==] } } },
      },
      output = { [==[0]==] },
    },
    {
      name = [==[Waiting for data]==], topic = [==[Promises]==],
      lesson = { [==[fetch gives back a promise. then runs a function once the promise settles.]==] },
      code = { [==[fetch("/volts").{1}(r => r.json());]==] },
      blanks = {
        { answers = { [==[then]==] }, hint = [==[What happens next, in one word.]==], wrong = { { [==[await]==], [==[await goes before the call, inside an async function. With a promise chain use then.]==] } } },
      },
      output = { [==[the parsed JSON once it arrives]==] },
    },
    {
      name = [==[Async and await]==], topic = [==[Async]==],
      lesson = { [==[Inside an async function, await pauses until a promise settles and gives its value.]==] },
      code = { [==[async function load() {]==], [==[  const r = {1} fetch("/volts");]==], [==[  return r.json();]==], [==[}]==] },
      blanks = {
        { answers = { [==[await]==] }, hint = [==[The keyword that waits for a promise.]==], wrong = { { [==[then]==], [==[then chains a function. Here the call is waited for with await.]==] } } },
      },
      output = { [==[load() gives back a promise of the JSON]==] },
    },
    {
      name = [==[Sharing code]==], topic = [==[Modules]==],
      lesson = { [==[import takes names from another module that exported them.]==] },
      code = { [==[{1} { add } from "./math.js";]==], [==[console.log(add(2, 3));]==] },
      blanks = {
        { answers = { [==[import]==] }, hint = [==[The keyword that brings in code from another file.]==], wrong = { { [==[require]==], [==[require is the older CommonJS style. With from use import.]==] }, { [==[include]==], [==[include is C. JavaScript uses import.]==] } } },
      },
      output = { [==[5]==] },
    },
  },
}
