use chrono::Datelike;
use rand::seq::SliceRandom;

use rand::thread_rng;

struct PhotoCreator {
    haircuts: Vec<char>,
    eyes: Vec<char>,
    noses: Vec<char>,
    mouths: Vec<char>,
}

impl PhotoCreator {
    fn new() -> Self {
        PhotoCreator {
            haircuts: vec!['/', '\\', '|', '-', '~', '^', '*', '#', '@', '!', ')', '(', '&', '?', '$'],
            eyes: vec!['o', 'O', '^', '-', 'x', 'X', '*', '+', '0', 'Q', '?', '$', '#', '\''],
            noses: vec!['C', 'v', '>', '<', '7', 'L', 'J', 'T', 'Y', 'U'],
            mouths: vec!['=', '_', '~', 'o', 'O', 'D', 'P', 'C', '@', '-', '<', '>', '3'],
        }
    }

    fn generate_photo(&self) -> Vec<String> {
        let mut rng = thread_rng();

        let haircut = self.haircuts.choose(&mut rng).unwrap();
        let eye = self.eyes.choose(&mut rng).unwrap();
        let nose = self.noses.choose(&mut rng).unwrap();
        let mouth = self.mouths.choose(&mut rng).unwrap();

        vec![
            format!("  {}  ", haircut.to_string().repeat(9)),
            format!("  | {0}   {0} |  ", eye),
            format!("  |   {}   |  ", nose),
            format!("   \\  {}  /   ", mouth),
            format!("/¯¯¯¯¯v¯¯¯¯¯\\"),
        ]
    }
}

#[derive(Debug, Clone)]
pub struct Document {
    role: Role,
    name: String,
    year_of_birth: usize,
    title: String,
    accomplishments: Vec<String>,
}

impl Document {
    pub fn new(role: Role, name: String, age: usize) -> Document {
        let current_year: usize = chrono::Utc::now().year().try_into().unwrap();
        let year_of_birth = current_year - age;

        let title = all_titles(&role)
            .choose(&mut rand::thread_rng())
            .unwrap()
            .to_string();
        let accomplishments = all_accomplishments(&role)
            .choose_multiple(&mut rand::thread_rng(), 3) // Choose 3 random accomplishments
            .map(|s| s.to_string())
            .collect();

        Document {
            role,
            name,
            year_of_birth,
            title,
            accomplishments,
        }
    }
}

pub fn all_titles(role: &Role) -> Vec<&'static str> {
    match role {
        Role::Killer => vec![
            "Master Assassin",
            "Stealth Operative",
            "Hitman",
            "Underworld Enforcer",
            "Silent Executioner",
            "Professional Eliminator",
        ],
        Role::Police => vec![
            "Chief Inspector",
            "Head of Forensics",
            "Detective Sergeant",
            "Special Investigator",
            "Undercover Operative",
            "Anti-Crime Unit Leader",
        ],
        Role::Doctor => vec![
            "Chief Surgeon",
            "Head of Cardiology",
            "Senior Consultant in Pediatrics",
            "Leading Medical Researcher",
            "Pharmaceutical Innovator",
            "Head of Emergency Medicine",
        ],
        Role::Janitor => vec![
            "Senior Custodian",
            "Head Janitor",
            "Facility Maintenance Manager",
            "Cleanup Specialist",
            "Sanitation Expert",
            "Biohazard Remediation Technician",
        ],
        Role::OldWoman => vec![
            "Neighborhood Watch Leader",
            "Veteran Community Observer",
            "Senior Intelligence Gatherer",
            "Local Historian",
            "Community Matriarch",
            "Elder Informant",
        ],
        Role::Swindler => vec![
            "Master Forger",
            "Con Artist",
            "Identity Thief",
            "Deception Specialist",
            "Information Manipulator",
            "Professional Liar",
        ],
        Role::Avenger => vec![
            "Vengeful Spirit",
            "Retribution Seeker",
            "Justice Enforcer",
            "Punisher",
            "Guardian of Vengeance",
            "Nemesis",
        ],
        Role::Judge => vec![
            "Chief Justice",
            "Senior Judge",
            "Verdict Deliverer",
            "Courtroom Authority",
            "Legal Arbiter",
            "Supreme Adjudicator",
        ],
        Role::Professor => vec![
            "Senior Scientist in Natural Sciences",
            "Head of Physics Department",
            "Lead Researcher in AI",
            "Distinguished Professor of Chemistry",
            "Renowned Astrophysicist",
            "Pioneer in Genetic Engineering",
        ],
        _ => panic!("Creation of a resident without a role!"),
    }
}

pub fn all_accomplishments(role: &Role) -> Vec<&'static str> {
    match role {
        Role::Killer => vec![
            "Completed 50+ high-profile assassinations",
            "Trained in 5 different martial arts",
            "Expert in stealth and evasion",
            "Master of disguise",
            "Successfully evaded capture for 10+ years",
            "Known for precision and efficiency",
        ],
        Role::Police => vec![
            "Solved 100+ cases",
            "Awarded Officer of the Year",
            "Led major anti-drug operation",
            "Specialized in cybercrime investigation",
            "Received bravery commendation",
            "Keynote speaker at law enforcement conferences",
        ],
        Role::Doctor => vec![
            "Invented a new heart surgery technique",
            "Published groundbreaking research on cancer treatment",
            "Developed a new vaccine",
            "Awarded Medical Innovator of the Year",
            "Authored 10+ medical textbooks",
            "Led international health missions",
        ],
        Role::Janitor => vec![
            "Recognized for exceptional cleanliness standards",
            "Specialized in hazardous waste disposal",
            "Trained in emergency response procedures",
            "Implemented new sanitation protocols",
            "Awarded Employee of the Year",
            "Expert in facility management",
        ],
        Role::OldWoman => vec![
            "Gathered crucial information leading to arrests",
            "Recognized as a pillar of the community",
            "Collected extensive local history records",
            "Helped resolve neighborhood disputes",
            "Awarded Community Service Medal",
            "Known for keen observation skills",
        ],
        Role::Swindler => vec![
            "Successfully forged 100+ documents",
            "Executed large-scale identity theft",
            "Manipulated key information in major cases",
            "Trained in advanced deception techniques",
            "Awarded Master of Deception",
            "Known for impeccable forgeries",
        ],
        Role::Avenger => vec![
            "Avenged 50+ wronged individuals",
            "Trained in hand-to-hand combat",
            "Expert in tracking and surveillance",
            "Known for delivering swift justice",
            "Awarded Protector of the Innocent",
            "Feared by wrongdoers",
        ],
        Role::Judge => vec![
            "Presided over 500+ trials",
            "Authored influential legal opinions",
            "Recognized for judicial fairness",
            "Keynote speaker at legal conferences",
            "Awarded Judge of the Year",
            "Known for upholding justice",
        ],
        Role::Professor => vec![
            "Calmness syrop inventor",
            "Exceptional achievements in human brain research",
            "Nobel Prize in Physics",
            "Published 50+ scientific papers",
            "Keynote speaker at international conferences",
            "Patented 10+ innovations",
        ],
        _ => panic!("Creation of a resident without a role!"),
    }
}

use std::fmt;

use crate::{roles::Role, text_formatters::format_to_length};

impl fmt::Display for Document {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let photo = PhotoCreator::new().generate_photo();
        const DOCUMENT_WIDTH: usize = 50;
        const PHOTO_WIDTH: usize = 13;
        const BORDERS_WIDTH: usize = 3;
        const TEXT_WIDTH: usize = DOCUMENT_WIDTH - BORDERS_WIDTH - PHOTO_WIDTH;
        let horizontal_line = &format!("+{:-^width$}+", "DOCUMENT", width = DOCUMENT_WIDTH - 2);

        let mut output = String::new();
        output.push_str(&format!("{}\n", horizontal_line));
        output.push_str(&format!(
            "|{:^}|{}|\n",
            photo[0],
            format_to_length(&self.name, TEXT_WIDTH)
        ));
        output.push_str(&format!(
            "|{:^}|{:^TEXT_WIDTH$}|\n",
            photo[1], &self.year_of_birth
        ));
        output.push_str(&format!(
            "|{:^}|{}|\n",
            photo[2],
            format_to_length(&format!("{:?}", self.role), TEXT_WIDTH)
        ));
        output.push_str(&format!(
            "|{:^}|{}|\n",
            photo[3],
            format_to_length(&self.title, TEXT_WIDTH)
        ));
        output.push_str(&format!(
            "|{:^}|{}|\n",
            photo[4],
            format_to_length(&format!("{:?}", self.accomplishments[0]), TEXT_WIDTH)
        ));
        output.push_str(&format!(
            "|{:_^PHOTO_WIDTH$}|{}|\n",
            "",
            format_to_length(&format!("{:?}", self.accomplishments[1]), TEXT_WIDTH)
        ));
        output.push_str(&format!(
            "|{:^PHOTO_WIDTH$}|{}|\n",
            "",
            format_to_length(&format!("{:?}", self.accomplishments[2]), TEXT_WIDTH)
        ));
        output.push_str(&format!("{}\n", horizontal_line));
        write!(f, "{}", output)
    }
}
