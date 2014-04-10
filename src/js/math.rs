use js::value::{Value, VNumber, VFunction, VObject, ResultValue, to_value};
use js::function::{NativeFunction, NativeFunc};
use collections::treemap::TreeMap;
use rand::random;
use std::f64;
use std::gc::Gc;
use std::cell::RefCell;

/// Get the absolute value of a number
pub fn abs(_:Value, _:Value, args:Vec<Value>) -> ResultValue {
	Ok(Gc::new(VNumber(if args.len() >= 1 {
		args.get(0).borrow().to_num().abs()
	} else {
		f64::NAN
	})))
}
/// Get the arccos of a number
pub fn acos(_:Value, _:Value, args:Vec<Value>) -> ResultValue {
	Ok(Gc::new(VNumber(if args.len() >= 1 {
		args.get(0).borrow().to_num().acos()
	} else {
		f64::NAN
	})))
}
/// Get the arcsine of a number
pub fn asin(_:Value, _:Value, args:Vec<Value>) -> ResultValue {
	Ok(Gc::new(VNumber(if args.len() >= 1 {
		args.get(0).borrow().to_num().asin()
	} else {
		f64::NAN
	})))
}
/// Get the arctangent of a number
pub fn atan(_:Value, _:Value, args:Vec<Value>) -> ResultValue {
	Ok(Gc::new(VNumber(if args.len() >= 1 {
		args.get(0).borrow().to_num().atan()
	} else {
		f64::NAN
	})))
}
/// Get the arctangent of a numbers
pub fn atan2(_:Value, _:Value, args:Vec<Value>) -> ResultValue {
	Ok(Gc::new(VNumber(if args.len() >= 1 {
		args.get(0).borrow().to_num().atan2(&args.get(1).borrow().to_num())
	} else {
		f64::NAN
	})))
}
/// Get the cubic root of a number
pub fn cbrt(_:Value, _:Value, args:Vec<Value>) -> ResultValue {
	Ok(Gc::new(VNumber(if args.len() >= 1 {
		args.get(0).borrow().to_num().cbrt()
	} else {
		f64::NAN
	})))
}
/// Get lowest integer above a number
pub fn ceil(_:Value, _:Value, args:Vec<Value>) -> ResultValue {
	Ok(Gc::new(VNumber(if args.len() >= 1 {
		args.get(0).borrow().to_num().ceil()
	} else {
		f64::NAN
	})))
}
/// Get the cosine of a number
pub fn cos(_:Value, _:Value, args:Vec<Value>) -> ResultValue {
	Ok(Gc::new(VNumber(if args.len() >= 1 {
		args.get(0).borrow().to_num().cos()
	} else {
		f64::NAN
	})))
}
/// Get the power to raise the natural logarithm to get the number
pub fn exp(_:Value, _:Value, args:Vec<Value>) -> ResultValue {
	Ok(Gc::new(VNumber(if args.len() >= 1 {
		args.get(0).borrow().to_num().exp()
	} else {
		f64::NAN
	})))
}
/// Get the highest integer below a number
pub fn floor(_:Value, _:Value, args:Vec<Value>) -> ResultValue {
	Ok(Gc::new(VNumber(if args.len() >= 1 {
		args.get(0).borrow().to_num().floor()
	} else {
		f64::NAN
	})))
}
/// Get the natural logarithm of a number
pub fn log(_:Value, _:Value, args:Vec<Value>) -> ResultValue {
	Ok(Gc::new(VNumber(if args.len() >= 1 {
		args.get(0).borrow().to_num().log(&f64::consts::E)
	} else {
		f64::NAN
	})))
}
/// Get the maximum of several numbers
pub fn max(_:Value, _:Value, args:Vec<Value>) -> ResultValue {
	let mut max = f64::NEG_INFINITY;
	for arg in args.iter() {
		let num = arg.borrow().to_num();
		max = max.max(num);
	}
	Ok(to_value(max))
}
/// Get the minimum of several numbers
pub fn min(_:Value, _:Value, args:Vec<Value>) -> ResultValue {
	let mut max = f64::INFINITY;
	for arg in args.iter() {
		let num = arg.borrow().to_num();
		max = max.min(num);
	}
	Ok(to_value(max))
}
/// Raise a number to a power
pub fn pow(_:Value, _:Value, args:Vec<Value>) -> ResultValue {
	Ok(Gc::new(VNumber(if args.len() >= 2 {
		let num = args.get(0);
		let power = args.get(1);
		num.borrow().to_num().powf(&power.borrow().to_num())
	} else {
		f64::NAN
	})))
}
/// Generate a random floating-point number between 0 and 1
pub fn _random(_:Value, _:Value, args:Vec<Value>) -> ResultValue {
	Ok(Gc::new(VNumber(random())))
}
/// Round a number to the nearest integer
pub fn round(_:Value, _:Value, args:Vec<Value>) -> ResultValue {
	Ok(Gc::new(VNumber(if args.len() >= 1 {
		args.get(0).borrow().to_num().round()
	} else {
		f64::NAN
	})))
}
/// Get the sine of a number
pub fn sin(_:Value, _:Value, args:Vec<Value>) -> ResultValue {
	Ok(Gc::new(VNumber(if args.len() >= 1 {
		args.get(0).borrow().to_num().sin()
	} else {
		f64::NAN
	})))
}
/// Get the square root of a number
pub fn sqrt(_:Value, _:Value, args:Vec<Value>) -> ResultValue {
	Ok(Gc::new(VNumber(if args.len() >= 1 {
		args.get(0).borrow().to_num().sqrt()
	} else {
		f64::NAN
	})))
}
/// Get the tangent of a number
pub fn tan(_:Value, _:Value, args:Vec<Value>) -> ResultValue {
	Ok(Gc::new(VNumber(if args.len() >= 1 {
		args.get(0).borrow().to_num().tan()
	} else {
		f64::NAN
	})))
}
/// Create a new 'Math' object
pub fn _create() -> Value {
	let mut math = TreeMap::new();
	math.insert(~"E", to_value(f64::consts::E));
	math.insert(~"LN2", to_value(f64::consts::LN_2));
	math.insert(~"LN10", to_value(f64::consts::LN_10));
	math.insert(~"LOG2E", to_value(f64::consts::LOG2_E));
	math.insert(~"LOG10E", to_value(f64::consts::LOG10_E));
	math.insert(~"SQRT1_2", Gc::new(VNumber(0.5f64.sqrt())));
	math.insert(~"SQRT2", to_value(f64::consts::SQRT2));
	math.insert(~"PI", to_value(f64::consts::PI));
	math.insert(~"abs", to_value(abs));
	math.insert(~"acos", to_value(acos));
	math.insert(~"asin", to_value(asin));
	math.insert(~"atan", to_value(atan));
	math.insert(~"atan2", to_value(atan2));
	math.insert(~"cbrt", to_value(cbrt));
	math.insert(~"ceil", to_value(ceil));
	math.insert(~"cos", to_value(cos));
	math.insert(~"exp", to_value(exp));
	math.insert(~"floor", to_value(floor));
	math.insert(~"log", to_value(log));
	math.insert(~"max", to_value(max));
	math.insert(~"min", to_value(min));
	math.insert(~"pow", to_value(pow));
	math.insert(~"random", to_value(_random));
	math.insert(~"round", to_value(round));
	math.insert(~"sin", to_value(sin));
	math.insert(~"sqrt", to_value(sqrt));
	math.insert(~"tan", to_value(tan));
	to_value(math)
}