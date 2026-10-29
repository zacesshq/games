-- Learn to Code: the TypeScript track. Each level teaches one idea with a short lesson and a few
-- lines of code with blanks marked {1}, {2} and so on. A blank lists the answers that work,
-- a hint and the mistakes people often make, each with a message that explains it. The output
-- is what the finished code prints or does.
-- TypeScript is JavaScript with types. Single and double quotes are the same here.
-- Answers are compared inside the game; nothing typed is ever run.

return {
  language = [==[TypeScript]==],
  id = [==[typescript]==],
  quotes_equal = true,
  levels = {
    {
      name = [==[A typed number]==], topic = [==[Types]==],
      lesson = { [==[A type annotation follows a colon: let volts: number = 12.]==] },
      code = { [==[let volts: {1} = 12;]==] },
      blanks = {
        { answers = { [==[number]==] }, hint = [==[TypeScript's single type for numbers, in lower case.]==], wrong = { { [==[int]==], [==[TypeScript has one number type: number.]==] }, { [==[Number]==], [==[Number is the wrapper object. Use lower case number.]==] } } },
      },
      output = { [==[volts can only hold numbers]==] },
    },
    {
      name = [==[Typed text]==], topic = [==[Types]==],
      lesson = { [==[Text has the type string.]==] },
      code = { [==[let name: {1} = "Ada";]==] },
      blanks = {
        { answers = { [==[string]==] }, hint = [==[The type for text, in lower case.]==], wrong = { { [==[String]==], [==[String is the wrapper object. Use lower case string.]==] }, { [==[str]==], [==[str is Python. TypeScript uses string.]==] } } },
      },
      output = { [==[name can only hold text]==] },
    },
    {
      name = [==[Typed arrays]==], topic = [==[Arrays]==],
      lesson = { [==[An array type is the item type followed by [].]==] },
      code = { [==[let pins: {1}[] = [2, 3, 5];]==] },
      blanks = {
        { answers = { [==[number]==] }, hint = [==[The type of each item.]==], wrong = { { [==[int]==], [==[TypeScript has one number type: number.]==] } } },
      },
      output = { [==[pins can only hold numbers]==] },
    },
    {
      name = [==[What a function gives back]==], topic = [==[Functions]==],
      lesson = { [==[The return type comes after the parameter list and a colon.]==] },
      code = { [==[function twice(n: number): {1} {]==], [==[  return n * 2;]==], [==[}]==] },
      blanks = {
        { answers = { [==[number]==] }, hint = [==[n * 2 is a number.]==], wrong = { { [==[void]==], [==[void means the function gives back nothing. This one returns a number.]==] } } },
      },
      output = { [==[twice(21) is 42, typed as number]==] },
    },
    {
      name = [==[Optional parameters]==], topic = [==[Functions]==],
      lesson = { [==[A ? after a parameter's name makes it optional.]==] },
      code = { [==[function greet(name{1}: string) {]==], [==[  return "Hi " + (name ?? "there");]==], [==[}]==], [==[console.log(greet());]==] },
      blanks = {
        { answers = { [==[?]==] }, hint = [==[One punctuation mark that asks whether it is there.]==], wrong = { { [==[!]==], [==[! is the non-null assertion. ? makes a parameter optional.]==] } } },
      },
      output = { [==[Hi there]==] },
    },
    {
      name = [==[This or that]==], topic = [==[Union types]==],
      lesson = { [==[A union type allows one of several types: string | number.]==] },
      code = { [==[let id: string {1} number = 7;]==] },
      blanks = {
        { answers = { [==[|]==] }, hint = [==[The bar between two types.]==], wrong = { { [==[||]==], [==[|| is logical or for values. Types use a single |.]==] }, { [==[or]==], [==[Types are joined with |.]==] } } },
      },
      output = { [==[id can be text or a number]==] },
    },
    {
      name = [==[The shape of an object]==], topic = [==[Interfaces]==],
      lesson = { [==[An interface describes the properties an object must have.]==] },
      code = { [==[{1} Part {]==], [==[  name: string;]==], [==[  volts: number;]==], [==[}]==], [==[const d: Part = { name: "diode", volts: 0.7 };]==] },
      blanks = {
        { answers = { [==[interface]==] }, hint = [==[The keyword that declares an object's shape.]==], wrong = { { [==[class]==], [==[A class makes objects. interface only describes their shape.]==] }, { [==[struct]==], [==[struct is C and Go. TypeScript uses interface.]==] } } },
      },
      output = { [==[d has both properties, checked by the compiler]==] },
    },
    {
      name = [==[Naming a type]==], topic = [==[Type aliases]==],
      lesson = { [==[type gives a name to any type.]==] },
      code = { [==[{1} Pin = number;]==], [==[const red: Pin = 13;]==] },
      blanks = {
        { answers = { [==[type]==] }, hint = [==[The keyword for a type alias.]==], wrong = { { [==[typedef]==], [==[typedef is C. TypeScript uses type.]==] } } },
      },
      output = { [==[Pin is another name for number]==] },
    },
    {
      name = [==[Only these words]==], topic = [==[Literal types]==],
      lesson = { [==[A literal type allows only exact values: "on" | "off".]==] },
      code = { [==[let mode: "on" | "off" = {1};]==] },
      blanks = {
        { answers = { [==["on"]==], [==["off"]==] }, hint = [==[One of the two allowed words, in quotes.]==], wrong = { { [==[on]==], [==[The allowed values are strings, so they need quotes.]==] }, { [==["dim"]==], [==["dim" is not one of the allowed values, so the compiler rejects it.]==] } } },
      },
      output = { [==[mode can only be on or off]==] },
    },
    {
      name = [==[Any type, kept]==], topic = [==[Generics]==],
      lesson = { [==[A generic function names a type parameter in angle brackets and uses it.]==] },
      code = { [==[function first<{1}>(xs: T[]): T {]==], [==[  return xs[0];]==], [==[}]==] },
      blanks = {
        { answers = { [==[T]==] }, hint = [==[The type parameter's name used in the rest of the line.]==], wrong = { { [==[any]==], [==[any turns checking off. A type parameter keeps the real type.]==] } } },
      },
      output = { [==[first([2, 3]) is typed as number]==] },
    },
    {
      name = [==[Fixed once set]==], topic = [==[Readonly]==],
      lesson = { [==[readonly means a property can be set when the object is made but never changed after.]==] },
      code = { [==[interface Led {]==], [==[  {1} pin: number;]==], [==[}]==] },
      blanks = {
        { answers = { [==[readonly]==] }, hint = [==[One word: read only.]==], wrong = { { [==[const]==], [==[const is for variables. Properties use readonly.]==] }, { [==[final]==], [==[final is Java. TypeScript uses readonly.]==] } } },
      },
      output = { [==[led.pin = 2 is a compile error]==] },
    },
    {
      name = [==[Hidden fields]==], topic = [==[Classes]==],
      lesson = { [==[private keeps a class member out of reach from outside the class.]==] },
      code = { [==[class Led {]==], [==[  {1} pin = 13;]==], [==[  getPin() { return this.pin; }]==], [==[}]==] },
      blanks = {
        { answers = { [==[private]==], [==[#]==] }, hint = [==[The access word that hides a member.]==], wrong = { { [==[public]==], [==[public lets anyone change pin. private hides it.]==] } } },
      },
      output = { [==[new Led().pin is a compile error]==] },
    },
    {
      name = [==[Named choices]==], topic = [==[Enums]==],
      lesson = { [==[An enum names a fixed set of choices.]==] },
      code = { [==[{1} Colour { Red, Green }]==], [==[const c = Colour.Green;]==] },
      blanks = {
        { answers = { [==[enum]==] }, hint = [==[The keyword for a set of named choices.]==], wrong = { { [==[Enum]==], [==[Keywords are lower case: enum.]==] } } },
      },
      output = { [==[c is Colour.Green]==] },
    },
    {
      name = [==[Narrowing]==], topic = [==[Type guards]==],
      lesson = { [==[typeof in an if lets TypeScript narrow a union to one type inside the block.]==] },
      code = { [==[function show(id: string | number) {]==], [==[  if (typeof id === {1}) {]==], [==[    return id.toUpperCase();]==], [==[  }]==], [==[  return id;]==], [==[}]==] },
      blanks = {
        { answers = { [==["string"]==] }, hint = [==[The typeof result for text, in quotes.]==], wrong = { { [==[string]==], [==[typeof gives back text, so compare with "string" in quotes.]==] }, { [==["String"]==], [==[typeof gives lower case names: "string".]==] } } },
      },
      output = { [==[inside the if, id is a string]==] },
    },
    {
      name = [==[Typed promises]==], topic = [==[Async]==],
      lesson = { [==[An async function returns a Promise of its value's type.]==] },
      code = { [==[async function name(): Promise<{1}> {]==], [==[  return "Ada";]==], [==[}]==] },
      blanks = {
        { answers = { [==[string]==] }, hint = [==[The type of "Ada".]==], wrong = { { [==[Promise]==], [==[The Promise is already there. Inside the brackets goes the value's type.]==] } } },
      },
      output = { [==[await name() is typed as string]==] },
    },
  },
}
