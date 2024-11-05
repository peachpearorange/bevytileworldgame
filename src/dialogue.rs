use crate::mycommand::MyCommand;

type DialogueEffect = fn() -> MyCommand;
type DialogueTreeNode =
  (&'static str,
   &'static [(&'static str, &'static str, &'static str, Option<DialogueEffect>)]);
const DIALOGUE_END: DialogueTreeNode = ("END", &[]);
// type DialogueTREE = &'static [DialogueTREENode];

pub struct Dialogue(&'static [DialogueTreeNode]);


impl Dialogue {
  const fn new(tree: &'static [DialogueTreeNode]) -> Self { Self(tree) }
  pub const SPHERICAL_SPACE_COW:Self = Self::new(&[
    ("A", &[
      ("B", "Hello there, cow!", "Cow: \"Moo-stronaut reporting for duty!\"", None),
    ]),
    ("B", &[
      ("C", "Why are you in space?", "Cow: \"I'm here to study zero-gravity milk production!\"", None),
      ("D", "How did you become spherical?", "Cow: \"It's an evolutionary adaptation for space travel.\"", None),
      ("E", "Are you lost?", "Cow: \"No, space is my new pasture!\"", None),
    ]),
    ("C", &[
      ("F", "How's the milk production going?", "Cow: \"It's out of this world! Want to try some Moon Moo?\"", None),
      ("G", "Isn't it dangerous up here?", "Cow: \"No need to cow-er in fear, I've got my space suit!\"", None),
      ("H", "Who sent you here?", "Cow: \"Dr. Bovine von Lactose, the mad dairy scientist!\"", None),
    ]),
    ("D", &[
      ("I", "What are the advantages of being spherical?", "Cow: \"I can roll in any direction, and there are no corners to bump into!\"", None),
      ("J", "Are there other spherical animals in space?", "Cow: \"I've heard rumors of a cubical chicken, but that's just absurd.\"", None),
      ("K", "Can you change back to normal?", "Cow: \"Why would I? Being spherical is utterly amazing!\"", None),
    ]),
    ("E", &[
      ("L", "Don't you miss Earth?", "Cow: \"Sometimes, but the view up here is spe-cow-tacular!\"", None),
      ("M", "What do you eat in space?", "Cow: \"Cosmic ray grass and star dust. It's quite moo-tritious!\"", None),
      ("N", "How do you moo-ve around?", "Cow: \"I just roll with it! Newton's laws are my best friends.\"", None),
    ]),
    ("F", &[
      ("O", "Yes, I'd love to try some!", "Cow: \"Here's a glass of Moon Moo. It's extra frothy in zero-G!\"", None),
    ]),
    ("G", &[
      ("P", "What's the biggest danger you've faced?", "Cow: \"I once got caught in Saturn's rings. Talk about a tight spot!\"", None),
    ]),
    ("H", &[
      ("Q", "Can I meet this scientist?", "Cow: \"He's on the dark side of the moon. It's a bit of a trek!\"", None),
    ]),
    ("I", &[
      ("R", "Can you demonstrate your rolling?", "Cow: \"Sure! WATch me do a barrel roll!\"", None),
    ]),
    ("J", &[
      ("S", "A cubical chicken? That's crazy!", "Cow: \"I know, right? Geometry in space gets wild!\"", None),
    ]),
    ("K", &[
      ("T", "Do you ever get dizzy from being round?", "Cow: \"Nope, I'm always well-balanced!\"", None),
    ]),
    ("L", &[
      ("U", "What's your favorite view from space?", "Cow: \"The Milky Way, of course! It reminds me of home.\"", None),
    ]),
    ("M", &[
      ("V", "Does star dust taste good?", "Cow: \"It's a bit dry, but it makes my milk sparkle!\"", None),
    ]),
    ("N", &[
      ("W", "Can you explain the physics of your movement?", "Cow: \"It's all about conservation of moo-mentum!\"", None),
    ]),
    ("O", &[
      ("X", "Wow, it's delicious! Can I have the recipe?", "Cow: \"Sorry, it's a closely guarded secret of the cosmos.\"", None),
    ]),
    ("P", &[
      ("Y", "That sounds terrifying! How did you escape?", "Cow: \"I used my quick re-flex-es and dairy-ing escape plan!\"", None),
    ]),
    ("Q", &[
      ("Z", "Is he planning to send more animals to space?", "Cow: \"He's working on a flock of zero-gravity sheep as we speak!\"", None),
    ]),
    ("R", &[
      ("AA", "Impressive! Do you ever get motion sick?", "Cow: \"Nah, I've got a stomach of steel... er, four of them actually!\"", None),
    ]),
    ("S", &[
      ("AB", "Are there any other strange space animals?", "Cow: \"I've heard whispers of a dodecahedron dolphin, but that's just silly.\"", None),
    ]),
    ("T", &[
      ("AC", "You're full of jokes! Are all space cows this funny?", "Cow: \"Of course! Humor helps us cope with the uni-verse-al challenges.\"", None),
    ]),
    ("U", &[
      ("AD", "That's beautiful. Do you ever feel lonely up here?", "Cow: \"Sometimes, but then I remember I'm surrounded by stars... and star-struck fans like you!\"", None),
    ]),
    ("V", &[
      ("AE", "Sparkly milk sounds amazing! Can it grant superpowers?", "Cow: \"Only the power of good bone density and a happy tummy!\"", None),
    ]),
    ("W", &[
      ("AF", "You're quite the physicist! Ever thought of teaching?", "Cow: \"I've been thinking of starting a 'Moo-niversity' actually!\"", None),
    ]),
    ("X", &[
      ("END", "I understand. Thanks for sharing it with me!", "Cow: \"You're welcome! Remember, what happens in space, stays in space!\"", None),
    ]),
    ("Y", &[
      ("END", "You're quite the adventurer! Any other close calls?", "Cow: \"Well, there was this one time with a black hole... but that's a story for another day!\"", None),
    ]),
    ("Z", &[
      ("END", "Wow! What's next, pigs in orbit?", "Cow: \"Don't be silly! Everyone knows pigs can't fly... yet.\"", None),
    ]),
    ("AA", &[
      ("END", "You're amazing! Can I take a selfie with you?", "Cow: \"Of course! Let's make it a 'span-selfie' - spanning the cosmos!\"", None),
    ]),
    ("AB", &[
      ("END", "This is getting too weird. I think I need to go.", "Cow: \"Aw, don't have a cow, man! Stay a while and listen to more space tales!\"", None),
    ]),
    ("AC", &[
      ("END", "You're out of this world! Thanks for the chat!", "Cow: \"My pleasure! Remember, in space, everyone can hear you cream... your coffee!\"", None),
    ]),
    ("AD", &[
      ("END", "You're never alone with that attitude! Goodbye, space cow!", "Cow: \"Goodbye, Earth friend! May your dreams be as boundless as the universe!\"", None),
    ]),
    ("AE", &[
      ("END", "I'll take a gallon! This was fun, thanks!", "Cow: \"Come back anytime! The Milky Way's always open!\"", None),
    ]),
    ("AF", &[
      ("END", "SIGN me up for Astro-nomoo-my 101! Farewell!", "Cow: \"So long, and thanks for all the laughs! Keep reaching for the stars!\"", None),
    ]),
    DIALOGUE_END,
  ]);

  pub const SPACE_COWBOY: Self = Self::new(&[
    ("A", &[
      ("B", "Howdy, partner!", "Space Cowboy: \"Well, howdy there, space traveler! Welcome to the cosmic corral!\"", None),
    ]),
    ("B", &[
      ("C", "What's a cowboy doing in space?", "Space Cowboy: \"Roundin' up asteroids and headin' off comet stampedes, of course!\"", None),
      ("D", "Nice space suit! Is that leather?", "Space Cowboy: \"Sure is! Genuine Martian leather, tougher than a solar flare!\"", None),
      ("E", "Have you seen any aliens?", "Space Cowboy: \"Aliens? Why, I've shared a campfire with beings from more galaxies than you can count!\"", None),
    ]),
    ("C", &[
      ("F", "ASTEROID roundup? How does that work?", "Space Cowboy: \"With a quantum lasso and a whole lotta patience, partner!\"", None),
      ("G", "Comet stampedes sound dangerous!", "Space Cowboy: \"You bet your stars they are! But nothin' my trusty rocket horse can't handle.\"", None),
    ]),
    ("D", &[
      ("H", "Martian leather? Is that ethical?", "Space Cowboy: \"Now, don't you worry. It's all synthetic, made from Mars dust. No space cows harmed!\"", None),
      ("I", "How does it protect you from space?", "Space Cowboy: \"It's lined with nanotech fibers. Keeps out cosmic rays better than a fort keeps out rustlers!\"", None),
    ]),
    ("E", &[
      ("J", "Tell me about these aliens!", "Space Cowboy: \"Met a cloud being from Nebula Nine once. Makes a mean vapor coffee!\"", None),
      ("K", "A cosmic campfire? How?", "Space Cowboy: \"With a contained plasma flame, 'course! Roasts space marshmallows like you wouldn't believe.\"", None),
    ]),
    ("F", &[
      ("END", "That sounds amazing! Can you teach me?", "Space Cowboy: \"Sure thing, greenhorn! First lesson: always approach an asteroid from downwind.\"", None),
    ]),
    ("G", &[
      ("END", "A rocket horse? Now I've heard everything!", "Space Cowboy: \"Ol' Supernova here's been my loyal steed for light-years! Ain't ya, girl?\" *pats invisible horse*", None),
    ]),
    ("H", &[
      ("END", "That's a relief! It looks so realistic.", "Space Cowboy: \"Yep, fools even the keenest eye. Now, if you'll excuse me, I've got some solar wind to wrangle!\"", None),
    ]),
    ("I", &[
      ("END", "Incredible! Where can I get one?", "Space Cowboy: \"These suits are rarer than a quiet night in a neutron star saloon. But if you prove yourself, I might know a fella...\"", None),
    ]),
    ("J", &[
      ("END", "Vapor coffee? That's wild!", "Space Cowboy: \"Puts hair on your chest and a twinkle in your eye! Now, if you'll pardon me, I've got a date with the Milky Way.\"", None),
    ]),
    ("K", &[
      ("END", "Space marshmallows? Now I'm hungry!", "Space Cowboy: \"Tell ya what, next time you're in the Andromeda arm, look me up. We'll have ourselves a good ol' space hoedown!\"", None),
    ]),
    DIALOGUE_END,
  ]);

  pub const SOCRATES: Self = Self::new(&[
    ("A", &[
      ("B", "Greetings, Socrates! How are you finding space?", "Socrates: \"Ah, greetings, young seeker of knowledge! Space, like wisdom, is vast and full of wonder.\"", None),
    ]),
    ("B", &[
      ("C", "What do you think about this future world?", "Socrates: \"I know that I know nothing of this world, which makes it all the more fascinating to question and explore.\"", None),
      ("D", "Can you tell me about your philosophical method?", "Socrates: \"Even here, amidst the stars, we must question everything. Shall we examine the nature of this cosmic realm?\"", None),
      ("E", "How does space travel relate to your ideas of the soul?", "Socrates: \"Perhaps our souls, like these celestial bodies, are on an eternal journey through the universe of ideas.\"", None),
    ]),
    ("C", &[
      ("F", "Does this advanced technology change your views on knowledge?", "Socrates: \"Technology may advance, but the pursuit of wisdom remains unchanged. We must still question and reflect.\"", None),
      ("G", "What would you ask the aliens if we meet them?", "Socrates: \"I would ask them about their concept of virtue, and whether it's universal across the cosmos.\"", None),
    ]),
    ("D", &[
      ("H", "How would you apply the Socratic method to space exploration?", "Socrates: \"We must question our assumptions about the universe, just as we question our beliefs about ourselves.\"", None),
      ("I", "Can your ideas of ethics apply to alien civilizations?", "Socrates: \"The search for universal truths should extend beyond Earth. Perhaps aliens too seek the good life.\"", None),
    ]),
    ("E", &[
      ("J", "Do you think space travel could be a form of seeking truth?", "Socrates: \"Indeed! As we journey through space, are we not also journeying through the realm of ideas?\"", None),
      ("K", "How does floating in space compare to your concept of the Forms?", "Socrates: \"This weightlessness reminds me of how the soul must feel when contemplating the Forms. Utterly free!\"", None),
    ]),
    ("F", &[
      ("END", "That's profound. Thank you for your wisdom, Socrates.", "Socrates: \"Remember, the unexamined space life is not worth living! Now, shall we ponder the ethics of faster-than-light travel?\"", None),
    ]),
    ("G", &[
      ("END", "Alien virtue? That's a mind-bending concept!", "Socrates: \"Indeed! And in questioning them, we may learn more about ourselves. Now, I wonder if there's a cosmic equivalent of hemlock...\"", None),
    ]),
    ("H", &[
      ("END", "I see. Question everything, even in space!", "Socrates: \"Precisely! Now, let us question the very nature of these asteroid fields. What is their essence?\"", None),
    ]),
    ("I", &[
      ("END", "Universal ethics across species... fascinating!", "Socrates: \"A worthy pursuit indeed! Now, if you'll excuse me, I must go contemplate the allegory of the black hole.\"", None),
    ]),
    ("J", &[
      ("END", "Space travel as a metaphor for seeking truth. Brilliant!", "Socrates: \"You show wisdom, young space traveler. Now, shall we examine the true form of these twinkling stars?\"", None),
    ]),
    ("K", &[
      ("END", "Your ideas truly transcend time and space, Socrates.", "Socrates: \"As do all ideas, my friend. Now, I must float away and dialectically analyze this cosmic dust.\"", None),
    ]),
    DIALOGUE_END,
  ]);

  pub const MARIE_CURIE: Self = Self::new(&[
    ("A", &[
      ("B", "Madame Curie! It's an honor. How are you adapting to space?", "Marie Curie: \"Bonjour! The universe is full of natural marvels. I'm detecting fascinating radiation patterns!\"", None),
    ]),
    ("B", &[
      ("C", "What do you think about modern space technology?", "Marie Curie: \"C'est incroyable! The advances in physics and chemistry have led to marvels beyond my wildest dreams.\"", None),
      ("D", "How does your work on radioactivity apply here?", "Marie Curie: \"The principles remain the same, but the scale is enormous! Cosmic rays, solar radiation... so much to study!\"", None),
      ("E", "What would you like to research in space?", "Marie Curie: \"I'm fascinated by the potential for new elements in these asteroids. Shall we start collecting samples?\"", None),
    ]),
    ("C", &[
      ("F", "Do you think space travel would have changed your research?", "Marie Curie: \"Undoubtedly! The absence of gravity opens up new possibilities for experiments in radioactivity.\"", None),
      ("G", "What advice would you give to future scientists?", "Marie Curie: \"Never fear the unknown. In science and in space, curiosity is our greatest asset.\"", None),
    ]),
    ("D", &[
      ("H", "How would you protect astronauts from cosmic radiation?", "Marie Curie: \"We must study it first! Understanding radiation is key to protection. Perhaps a new element could help...\"", None),
      ("I", "Could your work on X-rays be applied to space medicine?", "Marie Curie: \"Absolutely! Imagine a portable X-ray device for diagnosing injuries on long space voyages.\"", None),
    ]),
    ("E", &[
      ("J", "What kind of lab equipment would you need for space research?", "Marie Curie: \"A spectrometer would be essential. And perhaps we could design a microgravity centrifuge for separation!\"", None),
      ("K", "Do you think we might find radioactive alien life?", "Marie Curie: \"An intriguing hypothesis! We must approach it with rigorous scientific method and an open mind.\"", None),
    ]),
    ("F", &[
      ("END", "Your passion for science is truly inspiring, Madame Curie.", "Marie Curie: \"Merci! Remember, in science as in space exploration, we must have perseverance and faith in the unknown.\"", None),
    ]),
    ("G", &[
      ("END", "Great advice! Science and exploration go hand in hand.", "Marie Curie: \"Indeed! Now, shall we analyze the spectral lines of that nearby star? For science!\"", None),
    ]),
    ("H", &[
      ("END", "A new element for radiation shielding? Brilliant idea!", "Marie Curie: \"Every discovery opens new doors. Now, let's calibrate this space-suited Geiger counter!\"", None),
    ]),
    ("I", &[
      ("END", "Space X-rays... that could revolutionize long-distance space travel!", "Marie Curie: \"Exactement! Science knows no borders, not even in the vastness of space. Now, where did I put my radium samples...\"", None),
    ]),
    ("J", &[
      ("END", "A space lab sounds amazing. You're already adapting to the future!", "Marie Curie: \"Science evolves, but the spirit of inquiry remains. Now, let's see what secrets these cosmic rays hold!\"", None),
    ]),
    ("K", &[
      ("END", "Radioactive aliens? Now that's a sci-fi concept!", "Marie Curie: \"Science often surpasses fiction! Now, help me set up this zero-gravity polonium experiment, s'il vous plaît.\"", None),
    ]),
    DIALOGUE_END,
  ]);

  pub const ABRAHAM_LINCOLN: Self = Self::new(&[
    ("A", &[
      ("B", "President Lincoln! How are you finding the space age?", "Lincoln: \"Four score and seven light-years ago... I jest. This future is both terrifying and awe-inspiring.\"", None),
    ]),
    ("B", &[
      ("C", "How do your ideas of democracy apply to space colonization?", "Lincoln: \"A government of the planets, by the planets, for the planets, shall not perish from this universe.\"", None),
      ("D", "What do you think about the current state of equality?", "Lincoln: \"Progress has been made, but our journey continues. We must ensure liberty and justice for all sentient beings.\"", None),
      ("E", "How would you handle diplomacy with alien races?", "Lincoln: \"With malice toward none, with charity for all... even those with tentacles or exoskeletons.\"", None),
    ]),
    ("C", &[
      ("F", "Should every planet have equal representation?", "Lincoln: \"A house divided against itself cannot stand, even if that house spans galaxies. We must find a way to unite.\"", None),
      ("G", "What about AI rights in this futuristic society?", "Lincoln: \"The notion that all intelligences are created equal must extend to artificial ones too. It's the next frontier of rights.\"", None),
    ]),
    ("D", &[
      ("H", "Have we achieved your vision of equality?", "Lincoln: \"Progress is evident, but the work is never finished. We must strive to extend equality across the cosmos.\"", None),
      ("I", "How can we apply your principles to alien civilizations?", "Lincoln: \"The better angels of our nature must guide us in treating all sentient life with respect and dignity.\"", None),
    ]),
    ("E", &[
      ("J", "Would you still believe in preserving the Union on a galactic scale?", "Lincoln: \"The principles remain sound. We must work to form a more perfect Union, even among the stars.\"", None),
      ("K", "How would you address conflicts between human colonies and alien worlds?", "Lincoln: \"Let us strive on to finish the work we are in, to achieve and cherish a just and lasting peace among ourselves and all sentient beings.\"", None),
    ]),
    ("F", &[
      ("END", "A galactic democracy... that's a big idea, Mr. President!", "Lincoln: \"Indeed it is! Now, if you'll excuse me, I need to draft the Emancipation Proclamation for the robots of Neptune...\"", None),
    ]),
    ("G", &[
      ("END", "AI rights? You're adapting quickly to future issues!", "Lincoln: \"The principles of liberty are timeless, my friend. Now, shall we discuss the ethics of faster-than-light travel?\"", None),
    ]),
    ("H", &[
      ("END", "Your vision continues to inspire us, even in space.", "Lincoln: \"Remember, the struggle for equality is as vast as space itself. Now, I must contemplate the Gettysburg Address for Martians.\"", None),
    ]),
    ("I", &[
      ("END", "Respecting all sentient life... a noble goal for the future.", "Lincoln: \"Indeed. The task remaining before us is as great as the cosmos itself. Now, where can a man get a stovepipe helmet for his spacesuit?\"", None),
    ]),
    ("J", &[
      ("END", "A galactic Union... that's an incredible concept!", "Lincoln: \"The work of unity never ceases, my friend. Now, I believe I have a speech to give at the Andromeda Lincoln Memorial.\"", None),
    ]),
    ("K", &[
      ("END", "Your words of peace resonate even in the space age, sir.", "Lincoln: \"May they echo across the stars. Now, I must attend to pressing matters. I hear there's a vampire problem on the dark side of the moon...\"", None),
    ]),
    DIALOGUE_END,
  ]);

  pub const CHRONOS_SPACE_WIZARD: Self = Self::new(&[
    ("A", &[
      ("B", "Who are you, and why have you brought these historical figures to space?", "Space Wizard: \"Greetings, cosmic traveler! I am Chronos, the Space Wizard of Time and Dimension. I have assembled these great minds for a grand purpose!\"", None),
    ]),
    ("B", &[
      ("C", "What is this grand purpose?", "Chronos: \"To solve the greatest challenges of the universe! These brilliant minds, combined with futuristic knowledge, might save reality itself!\"", None),
      ("D", "How did you bring them here?", "Chronos: \"With my Chrono-Spatial Translocator, of course! It plucks beings from their timestreams and deposits them here, fully adapted to space travel.\"", None),
      ("E", "Won't this disrupt the timeline?", "Chronos: \"Fear not! Once our task is complete, I shall return them to their exact moments in history, memories intact but disguised as vivid dreams.\"", None),
    ]),
    ("C", &[
      ("F", "What are these universal challenges?", "Chronos: \"The heat death of the universe, the reconciliation of quantum mechanics and general relativity, and the correct way to eat a cosmic sandwich in zero gravity!\"", None),
      ("G", "How can historical figures help with future problems?", "Chronos: \"Fresh perspectives, my friend! Sometimes the wisdom of the past is key to unlocking the mysteries of the future.\"", None),
    ]),
    ("D", &[
      ("H", "Is the Chrono-Spatial Translocator safe?", "Chronos: \"Mostly! There was that one incident with Cleopatra and the black hole, but we don't talk about that...\"", None),
      ("I", "Can anyone use this device?", "Chronos: \"Goodness, no! It requires a degree in Temporal Physics and a license from the Intergalactic Time Authority. Plus, really good spatial awareness.\"", None),
    ]),
    ("E", &[
      ("J", "What if they want to stay in the future?", "Chronos: \"An excellent question! But history must run its course. Their contributions in their own times are crucial to the development of humanity.\"", None),
      ("K", "Could their future knowledge change history?", "Chronos: \"Their memories of this adventure will fade upon return, leaving only subconscious inspiration. Clever, eh?\"", None),
    ]),
    ("F", &[
      ("END", "Those are... interesting challenges. Especially the sandwich one.", "Chronos: \"Never underestimate the importance of proper space cuisine! Now, excuse me while I explain quantum entanglement to Socrates.\"", None),
    ]),
    ("G", &[
      ("END", "I see. It's like a cosmic think tank!", "Chronos: \"Precisely! Now, if you'll pardon me, I need to stop Marie Curie from trying to split atoms on the ship.\"", None),
    ]),
    ("H", &[
      ("END", "Mostly safe? That's... reassuring.", "Chronos: \"Don't worry! The chances of accidental dinosaur materialization are very low this time. Now, where did I put that temporal stabilizer...\"", None),
    ]),
    ("I", &[
      ("END", "I see. So no borrowing it for weekend trips to the Renaissance.", "Chronos: \"I'm afraid not. Last time someone did that, we ended up with pizza in ancient Egypt. Now, I must calibrate the quantum flux capacitor!\"", None),
    ]),
    ("J", &[
      ("END", "That makes sense. It's a big responsibility.", "Chronos: \"Indeed it is! The burden of knowledge is heavy, but the fate of the cosmos is heavier. Now, I need to explain internet memes to Abe Lincoln.\"", None),
    ]),
    ("K", &[
      ("END", "Subconscious inspiration... very clever indeed!", "Chronos: \"Thank you! Now, if you'll excuse me, I need to prevent Nikola Tesla from rewiring our ship's power grid. Again.\"", None),
    ]),
    DIALOGUE_END,
  ]);
  pub const LEONARDO_DA_VINCI:Self  = Self::new(&[
    ("A", &[
      ("B", "Leonardo da Vinci! How are you finding the future?", "Leonardo: \"Ah, the marvels of this age! My mind overflows with new inventions and artworks inspired by the cosmos!\"", None),
    ]),
    ("B", &[
      ("C", "What do you think of modern technology?", "Leonardo: \"Magnifico! Though I must say, I had already envisioned many of these contraptions. See this spacesuit? I'm improving its design as we speak!\"", None),
      ("D", "How does space travel compare to your flying machine concepts?", "Leonardo: \"It's beyond my wildest dreams! Yet, the principles of flight I studied apply even here. Observe how we maneuver through this asteroid field!\"", None),
      ("E", "Would you like to paint this cosmic scenery?", "Leonardo: \"Oh, if only I had my easel! The play of light on these celestial bodies... it's the ultimate study of chiaroscuro!\"", None),
    ]),
    ("C", &[
      ("F", "What improvements would you make to our technology?", "Leonardo: \"I've been sketching designs for more efficient solar sails and a da Vinci-style space station. Care to take a look?\"", None),
      ("G", "How does this era inspire your creativity?", "Leonardo: \"The blend of art and science here is exquisite! I'm particularly intrigued by your holographic displays. An art form in itself!\"", None),
    ]),
    ("D", &[
      ("H", "Could your studies on bird flight help with space maneuvering?", "Leonardo: \"Indubitably! The grace of a bird and the dance of a spacecraft are not so different. It's all about understanding flow and resistance.\"", None),
      ("I", "What do you think of modern aviation?", "Leonardo: \"It's a dream realized! Though I must say, these rockets seem a bit inelegant. Perhaps we could design something more... artistic?\"", None),
    ]),
    ("E", &[
      ("J", "How would you capture the essence of space in art?", "Leonardo: \"I would blend the mathematical precision of star charts with the fluid beauty of nebulae. A fusion of the scientific and the divine!\"", None),
      ("K", "Would you be interested in creating art with our future tools?", "Leonardo: \"Absolutely! Imagine the possibilities of sculpting with zero-gravity 3D printers or painting with light itself!\"", None),
    ]),
    ("F", &[
      ("END", "Your ideas could revolutionize space travel, even now!", "Leonardo: \"Grazie mille! Now, if you'll excuse me, I must discuss the golden ratio with that charming nebula over there.\"", None),
    ]),
    ("G", &[
      ("END", "Your excitement for blending art and science is contagious!", "Leonardo: \"Art, science, technology - they are all one in the pursuit of knowledge and beauty! Now, where did I leave my anti-gravity sketchbook?\"", None),
    ]),
    ("H", &[
      ("END", "Birds and spaceships... I never thought of it that way!", "Leonardo: \"Nature is the greatest teacher, even among the stars! Now, I must continue my studies on the aerodynamics of space debris.\"", None),
    ]),
    ("I", &[
      ("END", "An artistic rocket? That's an intriguing concept!", "Leonardo: \"Form and function in perfect harmony! Now, let me show you my preliminary sketches for a Vitruvian Spaceman...\"", None),
    ]),
    ("J", &[
      ("END", "Your cosmic art sounds breathtaking. I can't wait to see it!", "Leonardo: \"The universe itself is the ultimate masterpiece! Now, if you'll pardon me, I need to recalibrate the golden ratio for non-Euclidean space.\"", None),
    ]),
    ("K", &[
      ("END", "Sculpting in zero-g... Now that would be something to see!", "Leonardo: \"Indeed! The possibilities are as endless as space itself. Now, I must go - I have an appointment to exchange ideas with a sentient gas cloud!\"", None),
    ]),
    DIALOGUE_END,
  ]);

  pub const CLEOPATRA:Self = Self::new(&[
    ("A", &[
      ("B", "Queen Cleopatra! How are you adapting to the space age?", "Cleopatra: \"Greetings, cosmic traveler. I must say, ruling a galactic empire would have been... intriguing.\"", None),
    ]),
    ("B", &[
      ("C", "How does space travel compare to sailing the Nile?", "Cleopatra: \"The Nile was but a stream compared to this river of stars. Though I do miss the crocodiles... perhaps we could find some space equivalents?\"", None),
      ("D", "What do you think about modern politics and diplomacy?", "Cleopatra: \"Politics, like the cosmos, is vast and complex. But whether on Earth or among the stars, alliances and strategy remain key.\"", None),
      ("E", "How would you apply your leadership skills in this era?", "Cleopatra: \"An empire among the stars... now that's an ambition worthy of a pharaoh! I would unite planets as I united Egypt and Rome.\"", None),
    ]),
    ("C", &[
      ("F", "Space crocodiles? That's an interesting idea!", "Cleopatra: \"Indeed! Every queen needs her guardians. Besides, I'm sure there are plenty of cosmic treasures to protect in this vast universe.\"", None),
      ("G", "What aspects of space exploration fascinate you most?", "Cleopatra: \"The diversity of worlds reminds me of the cultures along the Mediterranean. Each unique, yet connected by the cosmic seas.\"", None),
    ]),
    ("D", &[
      ("H", "How would you handle diplomacy with alien races?", "Cleopatra: \"With grace, wisdom, and a hint of mystery. Whether dealing with Romans or Reptilians, a grand entrance is essential.\"", None),
      ("I", "What lessons from your era apply to galactic politics?", "Cleopatra: \"Power is about perception and alliances. Even in space, one must know when to be the asp and when to be the charm.\"", None),
    ]),
    ("E", &[
      ("J", "A galactic empire? That's quite ambitious!", "Cleopatra: \"Go big or go home, as they say. Though in space, I suppose everywhere is home. First, we'll need a cosmic Alexandria...\"", None),
      ("K", "How would you structure a government across planets?", "Cleopatra: \"A pharaoh for each world, united under a galactic regent. Myself, naturally. With faster-than-light communication, governance should be a breeze.\"", None),
    ]),
    ("F", &[
      ("END", "I'll keep an eye out for space crocodiles, Your Majesty.", "Cleopatra: \"Do that, dear friend. Now, if you'll excuse me, I must review the blueprints for my orbital pyramid.\"", None),
    ]),
    ("G", &[
      ("END", "Your insight draws beautiful parallels, Your Highness.", "Cleopatra: \"Thank you. The universe, like Egypt, is full of hidden treasures. Now, I'm off to negotiate mining rights with the asteroid belt pharaohs.\"", None),
    ]),
    ("H", &[
      ("END", "Diplomacy through mystery and grandeur. Classic Cleopatra!", "Cleopatra: \"One must keep the mystique alive, even in a spacesuit. Now, be a dear and help me plan my zero-gravity barge procession.\"", None),
    ]),
    ("I", &[
      ("END", "The asp and the charm... a timeless strategy, it seems.", "Cleopatra: \"In politics, some things never change. Now, I must go charm the Arcturian ambassador. Or was it the Betelgeusian regent?\"", None),
    ]),
    ("J", &[
      ("END", "A cosmic Alexandria sounds magnificent!", "Cleopatra: \"Doesn't it? With a library containing the knowledge of a million worlds! Now, if you'll excuse me, I need to discuss funding with the Galactic Senate.\"", None),
    ]),
    ("K", &[
      ("END", "Your administrative skills are truly universal, Your Highness.", "Cleopatra: \"Naturally. Now, I must go. These star charts won't decipher themselves, and I have a galaxy to unite!\"", None),
    ]),
    DIALOGUE_END,
  ]);
}
