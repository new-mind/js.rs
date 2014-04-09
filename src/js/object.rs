use js::function::{NativeFunc, NativeFunction};
use js::value::{ValueData, Value, VFunction, VUndefined, VObject, VInteger, VString, ResultValue};
use collections::treemap::TreeMap;
use serialize::json::;
use std::gc::Gc;
use std::cell::RefCell;
use std::fmt;

#[deriving(Clone)]
pub type ObjectData = TreeMap<~str, Value>;
impl fmt::Show for ObjectData {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		try!(f.buf.write_str("{ "));
		for (k, v) in self.iter() {
			try!(write!(f.buf, "{} = {}\n", k, v.borrow()));
		}
		f.buf.write_str("}")
	}
}
/// Create new object
pub fn make_object(this:Value, _:Value, _:Vec<Value>) -> ResultValue {
	Ok(Gc::new(VUndefined))
}
/// Get the prototype
pub fn get_proto_of(_:Value, _:Value, mut args:Vec<Value>) -> ResultValue {
	let obj = args.get(0);
	return Ok(obj.borrow().get_field(~"__proto__"));
}
/// Set the prototype
pub fn set_proto_of(_:Value, _:Value, mut args:Vec<Value>) -> ResultValue {
	let proto = args.get(1).clone();
	let obj = args.get(0);
	obj.borrow().set_field(~"__proto__", proto);
	return Ok(*obj);
}
/// To string
pub fn to_string(this:Value, _:Value, _:Vec<Value>) -> ResultValue {
	return Ok(Gc::new(VString(this.borrow().to_str())));
}
/// Create a new 'Object' object
pub fn _create() -> Value {
	let mut func = NativeFunction::new(make_object, 0);
	let mut prototype : ObjectData = TreeMap::new();
	prototype.insert(~"toString", Gc::new(VFunction(RefCell::new(NativeFunc(NativeFunction::new(to_string, 0))))));
	func.object.insert(~"length", Gc::new(VInteger(1)));
	func.object.insert(~"prototype", Gc::new(VObject(RefCell::new(prototype))));
	func.object.insert(~"setPrototypeOf", Gc::new(VFunction(RefCell::new(NativeFunc(NativeFunction::new(set_proto_of, 2))))));
	func.object.insert(~"getPrototypeOf", Gc::new(VFunction(RefCell::new(NativeFunc(NativeFunction::new(get_proto_of, 1))))));
	Gc::new(VFunction(RefCell::new(NativeFunc(func))))
}