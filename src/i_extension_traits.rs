// Imagine you have an outcome enum like this.

#[derive(Clone, PartialEq)]
pub enum Outcome {
	Ok,
	SomethingWentWrong,
	IDontKnow,
}

// A function takes some arbitrary input that's a collection of `T`, and processes each item
// individually. Each process can be an `Outcome`. We return `Vec<Outcome>`.

// pub fn process_stuff<T>(input: impl Iterator<Item = T>) -> Vec<Outcome> {
// 	unimplemented!("You are not expected to implement this function");
// }
pub fn process_stuff<T: std::fmt::Debug>(input: impl Iterator<Item = T>) -> Vec<Outcome> 
where 
T: std::fmt::Debug + std::cmp::PartialEq + std::ops::Rem<Output = T> + From<u8> + Copy {
    let mut outcomes = Vec::new();
    
    for item in input {
        let i: T = item.into();

        match i {
            num if num % 2.into() == 0.into() => outcomes.push(Outcome::Ok),
            num if num == 3.into() => outcomes.push(Outcome::SomethingWentWrong),
            _ => outcomes.push(Outcome::IDontKnow),
        }
    }

	outcomes
}

// What we want to achieve is a quick way (in terms of lines of code) to scan the output and
// determine how many were okay, how many were error, etc.
//
// A boring solution follows 🫣:

pub fn ok_count(outcomes: Vec<Outcome>) -> usize {
	// todo!();
    outcomes.iter().filter(|&outcome| outcome.eq(&Outcome::Ok)).count()
}
pub fn something_went_wrong_count(outcomes: Vec<Outcome>) -> usize {
	// todo!();
	outcomes.iter().filter(|&outcome| outcome.eq(&Outcome::SomethingWentWrong)).count()
}
pub fn i_dont_know_count(outcomes: Vec<Outcome>) -> usize {
	// todo!();
	outcomes.iter().filter(|&outcome| outcome.eq(&Outcome::IDontKnow)).count()
}

// This is quite lame. We want to be able to call these methods directly on the `Vec<Outcome>`. But
// how do we do this? We can't add a function to type `Vec`. This type is part of the standard
// library!
//
// Correct, but we can define a trait in the current module, and implement that for `Vec<_>`.
//
// This is a very common approach, and is called an "extension trait".

pub trait OutcomeCount {
	fn ok_count(&self) -> usize;
	fn something_went_wrong_count(&self) -> usize;
	fn i_dont_know_count(&self) -> usize;
}

// First, implement this trait.

impl OutcomeCount for Vec<Outcome> {
	fn ok_count(&self) -> usize {
		self.iter().filter(|&o| o.eq(&Outcome::Ok)).count()
	}
	fn i_dont_know_count(&self) -> usize {
		self.iter().filter(|&o| o.eq(&Outcome::IDontKnow)).count()
	}
	fn something_went_wrong_count(&self) -> usize {
		self.iter().filter(|&o| o.eq(&Outcome::SomethingWentWrong)).count()
	}
}

// Now we can call these functions directly on `Vec<Outcome>`.

/// This function is not graded. It is just for collecting feedback.
/// On a scale from 0 - 255, with zero being extremely easy and 255 being extremely hard,
/// how hard did you find this section of the exam.
pub fn how_hard_was_this_section() -> u8 {
	todo!()
}

/// This function is not graded. It is just for collecting feedback.
/// How much time (in hours) did you spend on this section of the exam?
pub fn how_many_hours_did_you_spend_on_this_section() -> u8 {
	todo!()
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn simple_functions() {
		let x = vec![Outcome::Ok, Outcome::Ok, Outcome::IDontKnow];

		assert_eq!(ok_count(x.clone()), 2);
		assert_eq!(i_dont_know_count(x.clone()), 1);
		assert_eq!(something_went_wrong_count(x), 0);
	}

	#[test]
	fn extension_trait() {
		let x = vec![Outcome::Ok, Outcome::Ok, Outcome::IDontKnow];

		assert_eq!(x.ok_count(), 2);
		assert_eq!(x.i_dont_know_count(), 1);
		assert_eq!(x.something_went_wrong_count(), 0);
	}
}
