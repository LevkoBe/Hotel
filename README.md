# **Hotel Management system**

**The Hotel: A Tale of Intrigue and Survival**

**Navigation**
- [Introduction](#introduction)
- [Setup](#setting-up-the-hotel)
- [Guest Management](#welcoming-guests)
- [Navigation](#navigating-the-hotel)
- [Game Mechanics](#embarking-on-the-game)
  - [Rules](#the-rule-of-the-game)
  - [Roles](#supported-roles)
  - [Victory Conditions](#the-quest-for-victory)
- [Design Patterns](#initial-design)
- [Diagrams](#diagrams)
- [Showcase](#showcase)

## *Introduction*

Welcome to The Hotel, where luxury meets mystery and every guest becomes a player in a thrilling game of survival and strategy. In this documentation, we'll delve into the intricacies of The Hotel system, from its initial setup to the heart- pounding climax of The Game.

## *Setting Up The Hotel*

The Hotel stands tall and imposing, ready to welcome its guests into a world of excitement and uncertainty. To bring this world to life, the system requires several key inputs:

- **Number of R ooms:** The foundation of The Hotel, dictating its capacity and potential for intrigue.

- **Capital:** The lifeblood of The Hotel, fueling its operations and determining its fate.

- **ID** A unique identifier for The Hotel, ensuring its distinct presence in the digital realm.

- **Optional: Number of R ooms per Story:** A customizable parameter, allowing for greater control over The Hotel's layout and dynamics.

With these inputs in hand, the system conjures a simple yet elegant graphical visualization of The Hotel, laying the groundwork for the adventures to come.

## *Welcoming Guests*

As The Hotel springs to life, guests flock to its doors, eager to partake in the excitement that awaits within. The system stands ready to accommodate their needs, processing commands such as  add resident with precision and efficiency.

**Supported Commands:**

- `add resident [name] [age] [account balance]` Adds a new resident to The Hotel.
- `get room [apartment number]` Retrieves the floor and room number of a specific apartment.
- `residents settled` Fills remaining rooms with bots and issues keys and 

  documents to them.

Each guest provides their details, including their name, age, and account balance. The system meticulously verifies this information, ensuring that every guest meets the age requirement and possesses the necessary funds to secure their stay.

Once approved, guests are granted keys to their designated rooms, their accounts duly charged for the privilege. In addition to keys, each guest receives a meticulously crafted document, detailing their role within The Hotel and providing a glimpse into the persona they'll embody during The Game.

### *Navigating The Hotel*

Armed with keys and documents, guests embark on their journey through The Hotel, navigating its halls with purpose and intrigue. The system stands as a steadfast guide, offering insights into the layout of The Hotel and the location of specific rooms.

In the meantime, the system carefully stores all the information about residents of The Hotel, including their type, documents, keys, status, and much more.

## *Embarking on The Game*

As night falls and shadows lengthen, The Game begins in earnest, casting The Hotel into a realm of mystery and suspense. Guests assume their roles with gusto, each playing a vital part in the unfolding drama.

### **The Rule of The Game:**

*By day, roam free in halls of grace,*

*In sunlight's warmth, find your place. From room to room, let wanderlust lead, But do no harm in thought or deed.*

*When night descends with velvet veil, One visit only, paths unveil.*

*Within your role, your actions bind, Mysteries of the night to find.*

### **Supported Roles:**

- **Killer:** Can eliminate other players or demand favors.
- **Policeman:** Investigates other players, collects evidence, and sends suspects to trial.
- **Doctor:** Can heal players or cause harm with potions.
- **Janitor:** Cleans rooms and has access to information about dead bodies.
- **Old Woman:** Can gather information about other players.
- **Swindler:** Can steal documents and manipulate information.
- **Avenger:** Seeks revenge and can incapacitate or eliminate players.
- **Judge:** Reviews evidence, delivers verdicts, and decides the fate of suspects.
- **Professor:** Conducts experiments and influences other players.

### *The Quest for Victory*

As The Game unfolds, players strive for victory through various means, each pursuing their own path to triumph:

- **Survival:** Some seek to outlast their rivals, clinging to life amidst the chaos of The Hotel's halls.
- **Achievement:** Others pursue specific objectives, whether it be eliminating a rival or achieving a lofty goal.
- **Bankruptcy:** As The Hotel's resources dwindle, players must adapt or face the consequences, navigating the perilous waters of financial ruin.

## *Conclusion*

As dawn breaks and The Game draws to a close, players emerge from The Hotel, their fates sealed and their stories etched into its halls. With feedback in hand, The Hotel looks to the future, ready to embark on new adventures fueled by the memories of those who dared to enter its doors.

And thus concludes the tale of The Hotel, where every guest becomes a player and every moment is a step toward destiny.

*Potential Growth Areas for the Application:*

- **Enhanced Interaction:** Introduce new commands and interactions to deepen player engagement.
- **Dynamic Events:** Implement unpredictable events throughout The Game to challenge players.
- **Customizable Avatars:** Allow players to personalize their avatars for a stronger sense of identity.
- **Advanced Role Abilities:** Introduce new powers for each role to add depth to gameplay.
- **Role Progression:** Implement a system where players unlock abilities as they progress.
- **Player Alliances:** Enable players to form alliances for collaborative strategies.
- **Multiple roles:** Allow each player to have a few roles simultaneously.
- **Additional Locations:** Introduce new areas within The Hotel for exploration and discovery.
- **Seasonal Events:** Introduce limited-time events for variety and excitement.
- **AI Interactions:** Incorporate AI-driven interactions with NPCs for immersion.
- **Expanded Narrative:** Develop branching storylines with multiple endings for player agency.
Hotel Management syst em 4

## Initial Design

### Design Patterns
1. Singleton Pattern
For unique instances of `Hotel` and `GameFlow`.
Ensure there's only one instance managing the game state and hotel setup.
2. Strategy Pattern
For role-specific behaviors and actions.
Each role can have its own strategy for actions during the night cycle.
3. Factory Pattern
For creating instances of `Hotel`, `Apartment`, and `Resident`.
Encapsulates the creation logic and can handle different building types,
apartment configurations, and resident roles.
### Diagrams
#### Class Diagram
![Class](https://github.com/user-attachments/assets/8a9dc7c9-aaf2-47b1-91da-9b9647276929)
#### Sequence Diagram
![Sequence](https://github.com/user-attachments/assets/9844be02-7337-4c3a-aad1-2ac0e548dac3)
#### State Diagram
![State](https://github.com/user-attachments/assets/638708ea-5239-4eaa-beb8-2bea0cb3f2bc)

## Showcase

### Configurable Hotel Display
<img width="920" height="912" alt="image" src="https://github.com/user-attachments/assets/8efcf96b-4057-4719-8738-b4eb33f5245d" />

### Auto-generated Player Documents
<img width="805" height="908" alt="image" src="https://github.com/user-attachments/assets/07b61a1c-a24d-4db6-9733-cb551b5f3c7d" />
