// Common TypeScript Type Checker Errors
// This file intentionally contains errors for testing

// TS2322: Type 'X' is not assignable to type 'Y'
// Name: Type is not assignable
let numberVar: number = "string value";
let boolVar: boolean = 123;

interface User {
	name: string;
	age: number;
}

let user: User = {
	name: "John",
	age: "thirty", // TS2322
};

// TS2339: Property 'X' does not exist on type 'Y'
// Name: Property does not exist
const obj = { foo: 1 };
console.log(obj.bar);

user.email; // TS2339

// TS2345: Argument of type 'X' is not assignable to parameter of type 'Y'
// Name: Argument type mismatch
function greet(name: string): void {
	console.log(`Hello, ${name}`);
}

greet(123); // TS2345

// TS2554: Expected X arguments, but got Y
// Name: Wrong number of arguments
function add(a: number, b: number): number {
	return a + b;
}

add(1); // TS2554
add(1, 2, 3); // TS2554

// TS2551: Property 'X' does not exist on type 'Y'. Did you mean 'Z'?
// Name: Property does not exist (with suggestion)
interface Person {
	firstName: string;
	lastName: string;
}

const person: Person = { firstName: "Jane", lastName: "Doe" };
console.log(person.fistName); // TS2551 (suggests firstName)

// TS2532: Object is possibly 'undefined'
// Name: Object is possibly undefined
function maybeGetUser(): User | undefined {
	return undefined;
}

const currentUser = maybeGetUser();
console.log(currentUser.name); // TS2532

// TS2531: Object is possibly 'null'
// Name: Object is possibly null
let nullableValue: string | null = null;
console.log(nullableValue.length); // TS2531

// TS2571: Object is of type 'unknown'
// Name: Object is of type unknown
function processData(data: unknown) {
	console.log(data.value); // TS2571
}

// TS2769: No overload matches this call
// Name: No matching overload
const arr = [1, 2, 3];
arr.map((x, y, z, extra) => x); // TS2769

// TS2790: The operand of a 'delete' operator must be optional
// Name: Delete operand must be optional
interface Required {
	id: number;
	name: string;
}

const req: Required = { id: 1, name: "test" };
delete req.name; // TS2790

// TS2564: Property 'X' has no initializer and is not definitely assigned in the constructor
// Name: Property has no initializer
class MyClass {
	prop: string; // TS2564

	constructor() {
		// prop not initialized
	}
}

// TS2741: Property 'X' is missing in type 'Y' but required in type 'Z'
// Name: Property is missing in type
interface Config {
	apiKey: string;
	endpoint: string;
	timeout: number;
}

const config: Config = {
	apiKey: "abc123",
	endpoint: "https://api.example.com",
	// missing timeout - TS2741
};

// TS7053: Element implicitly has an 'any' type because expression of type 'string' can't be used to index type 'X'
// Name: No index signature
interface FixedKeys {
	foo: string;
	bar: number;
}

function getValue(obj: FixedKeys, key: string) {
	return obj[key]; // TS7053
}

// TS2538: Type 'X' cannot be used as an index type
// Name: Cannot be used as index type
const obj2 = {};
const key = { toString: () => "key" };
obj2[key]; // TS2538

// TS2349: This expression is not callable
// Name: Expression is not callable
const notAFunction = 123;
notAFunction(); // TS2349

// TS2367: This condition will always return 'X' since the types 'Y' and 'Z' have no overlap
// Name: Condition always returns constant
function checkType(value: string) {
	if (typeof value === "number") {
		// TS2367
		console.log("This will never run");
	}
}

// TS2355: A function whose declared type is neither 'void' nor 'any' must return a value
// Name: Function must return a value
function getNumber(): number {
	// no return statement - TS2355
}

// TS2304: Cannot find name 'X'
// Name: Cannot find name
console.log(undefinedVariable); // TS2304

// TS2365: Operator '+' cannot be applied to types 'X' and 'Y'
// Name: Operator cannot be applied
const result = "string" - 5; // TS2365

// TS2802: Type 'X' can only be iterated through when using the '--downlevelIteration' flag
// Name: Needs downlevel iteration (less common)

// TS2416: Property 'X' in type 'Y' is not assignable to the same property in base type 'Z'
// Name: Property not assignable to base type
class Base {
	value: string = "base";
}

class Derived extends Base {
	value: number = 123; // TS2416
}

// TS2420: Class 'X' incorrectly implements interface 'Y'
// Name: Class incorrectly implements interface
interface IService {
	connect(): void;
	disconnect(): void;
}

class Service implements IService {
	connect() {
		console.log("connected");
	}
	// missing disconnect - TS2420
}

// TS2540: Cannot assign to 'X' because it is a read-only property
// Name: Cannot assign to readonly property
interface ReadonlyInterface {
	readonly id: number;
}

const readonlyObj: ReadonlyInterface = { id: 1 };
readonlyObj.id = 2; // TS2540

// TS2589: Type instantiation is excessively deep and possibly infinite
// Name: Type instantiation too deep (advanced)

// TS2307: Cannot find module 'X' or its corresponding type declarations
// Name: Cannot find module
import { something } from "non-existent-module"; // TS2307

// TS2451: Cannot redeclare block-scoped variable 'X'
// Name: Cannot redeclare variable
let duplicateVar = 1;
let duplicateVar = 2; // TS2451

// TS2366: Function lacks ending return statement and return type does not include 'undefined'
// Name: Function lacks ending return statement
function maybeReturn(condition: boolean): number {
	if (condition) {
		return 42;
	}
	// TS2366 - no return on else path
}

// TS2394: Overload signature is not compatible with function implementation
// Name: Overload signature not compatible
function overloadExample(x: string): string;
function overloadExample(x: number): number;
function overloadExample(x: boolean): string {
	// TS2394
	return x.toString();
}

// TS2363: The right-hand side of an arithmetic operation must be of type 'any', 'number', 'bigint' or an enum type
// Name: Right-hand side must be numeric
const calc = 10 * "string"; // TS2363

// TS2556: A spread argument must either have a tuple type or be passed to a rest parameter
// Name: Invalid spread argument
function takesThreeArgs(a: number, b: number, c: number) {}
const arr2 = [1, 2, 3, 4, 5];
takesThreeArgs(...arr2); // TS2556

// TS2352: Conversion of type 'X' to type 'Y' may be a mistake
// Name: Type assertion may be a mistake
const num = 123;
const str = num as unknown as string; // Generally OK with unknown intermediate
const directCast = 123 as string; // TS2352 - direct cast mistake

export {};
