use std::fs;
use std::option::Option;
use std::fmt;

pub fn run(input: &str) -> usize {
  let contents = fs::read_to_string(input)
        .expect("Something went wrong reading the file");
  split_passports(&contents).into_iter().map(Passport::parse).filter(Passport::validate1).count()
}

pub fn run_2(input: &str) -> usize {
  let contents = fs::read_to_string(input)
        .expect("Something went wrong reading the file");
        split_passports(&contents).into_iter().map(Passport::parse).filter(Passport::validate2).count()
}

fn split_passports(batch: &String) -> Vec<String> {
  let batch = batch.split("\n\n").collect::<Vec<&str>>();
  let batch = batch.into_iter().map(|s: &str| {
    String::from(s)
  }).collect();
  batch
}

struct Passport {
  birth_year: String,
  issue_year: String,
  expiration_year: String,
  height: String,
  hair_color: String,
  eye_color: String,
  passport_id: String,
  country_id: String,
}

impl fmt::Display for Passport {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
      write!(f, "Passport (\n  birth_year: {},\n  issue_year: {},\n  expiration_year: {},\n  height: {},\n  hair_color: {},\n  eye_color: {},\n  passport_id: {},\n  country_id: {}\n)", 
        self.birth_year, self.issue_year, self.expiration_year, self.height, self.hair_color, self.eye_color, self.passport_id, self.country_id)
  }
}

impl Passport {
  pub fn validate1(passport: &Passport) -> bool {
    !passport.birth_year.is_empty() 
      && !passport.issue_year.is_empty() 
      && !passport.expiration_year.is_empty() 
      && !passport.height.is_empty() 
      && !passport.hair_color.is_empty() 
      && !passport.eye_color.is_empty() 
      && !passport.passport_id.is_empty()
  }
  pub fn validate2(passport: &Passport) -> bool {
    if passport.birth_year.len() == 4 {
      let year = match passport.birth_year.parse::<usize>() {
        Ok(num) => num,
        Err(_) => return false
      };
      if year < 1920 || year > 2002 {
        return false
      }
    } else {
      return false
    }

    if passport.issue_year.len() == 4 {
      let year = match passport.issue_year.parse::<usize>() {
        Ok(num) => num,
        Err(_) => return false
      };
      if year < 2010 || year > 2020 {
        return false
      }
    } else {
      return false
    }

    if passport.expiration_year.len() == 4 {
      let year = match passport.expiration_year.parse::<usize>() {
        Ok(num) => num,
        Err(_) => return false
      };
      if year < 2020 || year > 2030 {
        return false
      }
    } else {
      return false
    }

    if EyeColor::value_of(&passport.eye_color).is_none() {
      return false
    }

    if passport.passport_id.len() == 9 {
      let valid = passport.passport_id.chars().fold(true, |valid, ch: char| {
        valid && ch.is_digit(10)
      });
      if !valid {
        return false
      }
    } else {
      return false
    }

    if passport.hair_color.len() == 7 {
      let mut chars = passport.hair_color.chars();
      let ch = match chars.next() {
        Some(c) => c,
        None => return false
      };
      if ch != '#' {
        return false
      }
      for _ in 1..6 {
        let ch = match chars.next() {
          Some(c) => c,
          None => return false
        };
        if !ch.is_digit(16) {
          return false
        }
      }
    } else {
      return false
    }
    
    if passport.height.len() == 4 || passport.height.len() == 5 {
      let chars = passport.height.chars();
      let mut num = String::new();
      let mut sufx = String::new();
      for ch in chars {
        if ch.is_digit(10) {
          num.push(ch);
        } else {
          sufx.push(ch)
        }
      }
      let num = match num.parse::<usize>() {
        Ok(num) => num,
        Err(_) => return false
      };
      if sufx == "in" {
        if num < 59 || num > 76 {
          return false
        }
      } else if sufx == "cm" {
        if num < 150 || num > 193 {
          return false
        }
      } else {
        return false
      }
    } else {
      return false
    }

    true
  }

  pub fn parse(s: String) -> Passport {
    let mut birth_year = String::from("");
    let mut issue_year = String::from("");
    let mut expiration_year = String::from("");
    let mut height = String::from("");
    let mut hair_color = String::from("");
    let mut eye_color = String::from("");
    let mut passport_id = String::from("");
    let mut country_id = String::from("");

    let mut token = String::from("");
    let mut value = String::from("");

    let mut s = s.clone();
    s.push(' ');

    for ch in s.chars() {
      if ch.is_whitespace() {
        let field_name = match CredentialField::value_of(&token) {
          Some(f) => f,
          None => continue
        };
        match field_name {
          CredentialField::BirthYear => {
            birth_year = value.clone();
          },
          CredentialField::IssueYear =>  {
            issue_year = value.clone();
          },
          CredentialField::ExpirationYear =>  {
            expiration_year = value.clone();
          },
          CredentialField::Height =>  {
            height = value.clone();
          },
          CredentialField::HairColor =>  {
            hair_color = value.clone();
          },
          CredentialField::EyeColor =>  {
            eye_color = value.clone();
          },
          CredentialField::PassportId =>  {
            passport_id = value.clone();
          },
          CredentialField::CountryId =>  {
            country_id = value.clone();
          },
        }

        token = String::from("");
        value = String::from("");
      } else if ch == ':' {
        // don't use the delimiter
      } else if token.len() == 3 {
        value.push(ch);
      } else {
        token.push(ch);
      }
    }

    Passport {
      birth_year: birth_year,
      issue_year: issue_year,
      expiration_year: expiration_year,
      height: height,
      hair_color: hair_color,
      eye_color: eye_color,
      passport_id: passport_id,
      country_id: country_id
    }
  } 
}

enum EyeColor {
  Amber,
  Blue,
  Brown,
  Gray,
  Green,
  Hazel,
  Other
}

impl EyeColor {
  pub fn value(&self) -> &str {
    match self {
      EyeColor::Amber => "amb",
      EyeColor::Blue => "blu",
      EyeColor::Brown => "brn",
      EyeColor::Gray => "gry",
      EyeColor::Green => "grn",
      EyeColor::Hazel => "hzl",
      EyeColor::Other => "oth",
    }
  }
  pub fn value_of(s: &str) -> Option<EyeColor> {
    match s {
      "amb" => Some(EyeColor::Amber),
      "blu" => Some(EyeColor::Blue),
      "brn" => Some(EyeColor::Brown),
      "gry" => Some(EyeColor::Gray),
      "grn" => Some(EyeColor::Green),
      "hzl" => Some(EyeColor::Hazel),
      "oth" => Some(EyeColor::Hazel),
      _ => None
    }
  }
}

enum CredentialField {
  BirthYear,
  IssueYear,
  ExpirationYear,
  Height,
  HairColor,
  EyeColor,
  PassportId,
  CountryId,
}

impl CredentialField {
  pub fn value(&self) -> &str {
    match self {
      CredentialField::BirthYear => "byr",
      CredentialField::IssueYear => "iyr",
      CredentialField::ExpirationYear => "eyr",
      CredentialField::Height => "hgt",
      CredentialField::HairColor => "hcl",
      CredentialField::EyeColor => "ecl",
      CredentialField::PassportId => "pid",
      CredentialField::CountryId => "cid",
    }
  }
  pub fn value_of(s: &str) -> Option<CredentialField> {
    match s {
      "byr" => Some(CredentialField::BirthYear),
      "iyr" => Some(CredentialField::IssueYear),
      "eyr" => Some(CredentialField::ExpirationYear),
      "hgt" => Some(CredentialField::Height),
      "hcl" => Some(CredentialField::HairColor),
      "ecl" => Some(CredentialField::EyeColor),
      "pid" => Some(CredentialField::PassportId),
      "cid" => Some(CredentialField::CountryId),
      _ => None
    }
  }
}
