// Conditionals - Used to check the condition of something and act on the result

pub fn run() {
  let age: u8 = 22;
  let check_id: bool = true;
  let knows_person_of_age = true;

  // If/Else
  let talk: String = if age >= 21 && check_id || knows_person_of_age {
    String::from("Bartender: What would you like to drink?")
  } else if age < 21 && check_id {
    "Bartender: Sorry, you have to leave".to_string()
  } else {
    "Bartender: I'll need to see your ID".to_string()
  };

  println!("{}", talk);

  // Shorthand If
  let is_of_age = if age >= 21 { true } else { false };
  println!("Is Of Age: {}", is_of_age)
}