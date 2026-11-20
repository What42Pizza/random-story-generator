use smart_read::{basics::YesNoInput, *};

/*
blank values:
%N: character name
%A: action (e.g. "throw a party")
%C: "current" action (e.g. "throwing a party")
%P: pronoun (e.g. "%P wanted to ..." -> "he wanted to ...") (doesn't work very well, just uses whatever character was last printed)
%p: pronoun (e.g. "%N wanted to eat with %p" -> "dave wanted to eat with him") (doesn't work very well, just uses whatever character was last printed)
*/

const STARTER_SENTENCES: &[&str] = &[
	"This is the story of %N.",
	"This all happened because %N thought it would be funny to %A.",
	"Okay, this is what happened.",
	"This is how it all went down.",
];

const MIDDLE_SENTENCES: &[&str] = &[
	"But then, %N got involved.",
	"Then they all wanted to %A.",
	"They all decided to %A.",
	"This made them all want to %A.",
	"This was turning into a death match.",
	"%P was just trying to %A.",
	"%N was trying to %A.",
	"%N tried to %A but failed.",
	"%N desperately tried to %A but failed.",
	"%N was desperately trying to %A.",
	"%N was thinking of %C.",
	"%N was clearly thinking of %C.",
	"%N broke down the door and rushed in.",
	"%P thought it would be fun.",
	"%P thought it would be funny.",
	"Maybe %C would be fun?",
	"But that didn't work, so instead %P tried to %A.",
	"This next part made it on the news.",
	"The next step was obviously to %A.",
	"%P was actually almost able to %A.",
	"%P didn't realize it's a crime to %A.",
	"%P tried to %A but %N wouldn't let %p.",
	"%N was well known for %C.",
	"%N was well known for trying to %A.",
	"%N thought this next part was hilarious.",
	"%N tired to %A but was too scared to.",
	"%P was intimidated by %N's attempt to %A.",
	"It really wasn't %p fault.",
	"So that's what %P's in jail for.",
	"%N came in to find %N %C.",
];

const ENDING_SENTENCES: &[&str] = &[
	"Then it all finally stopped.",
	"And somehow, everyone survived.",
	"And that was the last we heard of %p.",
];



const ACTIONS: &[&str] = &[
	"%A with %N's help",
	"%A with %N's help",
	"%A then %A",
	"%A then %A",
	"%A then %A",
	"%A without %C",
	"%A without %C",
	"%A in order to %A",
	"%A so that %N could %A",
	"%A (as a joke of course)",
	"%A at %N's house",
	"save %N from %N",
	"save %N's life",
	"give %N over to %N",
	"throw a party",
	"drive to the store",
	"drive %p to the store",
	"go to the hospital",
	"drive off a cliff",
	"throw %N off a cliff",
	"kill %N",
	"shoot %N",
	"punch %N",
	"kill %p",
	"shoot %p",
	"punch %p",
	"team up with %N",
	"play hunger games in real life",
	"play fnaf in real life",
	"play gta in real life",
	"flee the scene",
	"burn down %N's house",
	"eat %N's food",
	"steal %N's food",
	"eat %N's pizza",
	"kidnap %N",
	"steal a car",
	"steal %N's car",
	"throw a knife at %N",
];

const CURRENT_ACTIONS: &[&str] = &[
	"%C with %N's help",
	"%C with %N's help",
	"%C then %C",
	"%C then %C",
	"%C then %C",
	"%C without %C",
	"%C without %C",
	"%C in order to %A",
	"%C so that %N could %A",
	"%C (as a joke of course)",
	"%C at %N's house",
	"saving %N from %N",
	"giving %N over to %N",
	"throwing a party",
	"driving to the store",
	"driving %N to the store",
	"driving %p to the store",
	"going to the hospital",
	"driving off a cliff",
	"throwing %N off a cliff",
	"killing %N",
	"shooting %N",
	"punching %N",
	"killing %p",
	"shooting %p",
	"punching %p",
	"teaming up with %N",
	"playing hunger games in real life",
	"fleeing the scene",
	"burning down %N's house",
	"eating %N's food",
	"stealing %N's food",
	"eating %N's pizza",
	"kidnapping %N",
	"stealing a car",
	"stealing %N's car",
	"throwing a knife at %N",
];

const NAMES: &[(&str, G)] = &[
	("Jonathan", G::M),
	("Ryan", G::M),
	("Johnny", G::M),
	("Dottie", G::F),
	("Mary Belle", G::F),
	("Parker", G::M),
	("James", G::M),
	("Rebecca", G::F),
	("David", G::M),
	("Benjamin", G::M),
	("Samuel", G::M),
	("Seth", G::M),
	("Katie", G::F),
	("Jason", G::M),
	("Fred", G::M),
	("Kevin", G::M),
	("the police", G::M),
	("the firemen", G::M),
	("the IRS", G::M),
	("the government", G::M),
	("the president", G::M),
	("the dog", G::M),
	("a dog", G::M),
	("a random dog", G::M),
	("a cow", G::M),
	("a group of children", G::M),
	("a random stranger", G::M),
	("%N's mom", G::F),
	("%N's dad", G::M),
	("a rubber duck", G::M),
];

// genders
#[derive(Copy, Clone)]
enum G {
	M,
	F,
}



fn main() {
	loop {
		println!();
		generate();
		println!();
		if !prompt!("Again? "; [true] YesNoInput) { break; }
	}
}

fn generate() {
	
	let story_middle_len = rand::random_range(10..=16);
	let num_of_characters = rand::random_range(4..=6);
	
	let mut characters = NAMES.to_vec();
	while characters.len() > num_of_characters {
		characters.swap_remove(rand::random_range(0..characters.len()));
	}
	
	let mut last_gender = *random_value(&[G::M, G::F]);
	let mut story = vec![];
	story.push(fill_values(random_value(STARTER_SENTENCES), &characters, &mut last_gender));
	for i in 0..story_middle_len {
		story.push(fill_values(random_value(MIDDLE_SENTENCES), &characters, &mut last_gender));
		if i % 5 == 0 {
			characters.push(*random_value(NAMES));
		}
	}
	story.push(fill_values(random_value(ENDING_SENTENCES), &characters, &mut last_gender));
	
	for line in &mut story {
		let first_char = line.chars().next().expect("story sentence cannot be empty");
		assert_eq!((first_char as u8) as u32, first_char as u32, "first char must be ascii");
		unsafe {
			// safety: the first char must be a single byte because of the assert above
			line.as_bytes_mut()[0] = first_char.to_ascii_uppercase() as u8;
		}
	}
	
	for line in story {
		print!("        ");
		let mut column = 0;
		for word in line.split(' ') {
			column += word.len();
			if column >= 80 {
				print!("\n            ");
				column = 4;
			}
			print!("{word} ");
		}
		println!("");
	}
	
}



fn fill_values(template: impl AsRef<str>, characters: &[(&str, G)], last_gender: &mut G) -> String {
	let mut template_chars = template.as_ref().chars();
	let mut output = String::new();
	loop {
		
		let Some(c) = template_chars.next() else { break; };
		
		if c != '%' {
			output.push(c);
			continue;
		}
		
		let new_value_template = match template_chars.next().expect("There must always be another character after a '%' in a template") {
			
			'N' => {
				let (name, gender) = random_value(characters);
				*last_gender = *gender;
				name
			}
			'A' => random_value(ACTIONS),
			'C' => random_value(CURRENT_ACTIONS),
			'P' => match last_gender {
				G::M => "he",
				G::F => "she",
			}
			'p' => match last_gender {
				G::M => "him",
				G::F => "her",
			}
			'%' => "%",
			
			c => panic!("Unknown template variable '{c}''"),
			
		};
		let new_value = fill_values(new_value_template, characters, last_gender);
		
		output += &new_value;
		
	}
	output
}



pub fn random_value<T>(list: &[T]) -> &T {
	&list[rand::random_range(0..list.len())]
}
