use std::collections::{VecDeque, hash_map::DefaultHasher};
use std::hash::{Hash, Hasher};
use std::fmt::Debug;


/// This project is a simpler rewrite of QueueBot
/// which is a Discord bot used by CS 120 to keep track
/// of the queue for office hours.
/// An old version of the Python source code can be seen here: <https://github.com/benperumala/cs120-queuebot/>
///
/// This version is a recreation of that which does not connect to Discord.
/// Instead, it is a mockup that tries to roughly simulate
/// messages in the same way as Discord.
///
/// It is also simpler than the production bot as it
///  - Has a single queue instead of online and in-person queue
///  - Doesn't check if users are waiting in a voice channel
///  - Doesn't check if messages are from the correct channels
///  - Doesn't keep track of a config which allows for customization
///  - Doesn't display fancy Discord-specific embeds
///  - Doesn't notify TAs on the join in the queue

/// Used to prepend a warning, error, or success
/// emote to the beginning of a sent message.
enum MessageType {
    Warning,
    Success,
    Error
}

/// Implementation of a "trait"
/// Anyone who wants to implement this trait must create
/// the below methods.
trait User {
    /// Return a string which represents mentioning the given user
    /// Discord typically defines this as `<@XXXX>`
    /// where XXXX is the user's UUID
    /// However, since this is a mockup, `<@Username>` should be done instead
    /// to make it easier to read
    fn get_mention(&self) -> String;

    /// Returns a string which represents the usual
    /// way users share their account ID so other
    /// people can add them. Discord defines this as
    /// USERNAME#DDDD where USERNAME is their username
    /// and DDDD is four numbers
    fn get_tag(&self) -> String;

    /// Get the name of the user
    fn get_name(&self) -> String;

    /// Check if a user is a TA
    /// (Usually this involves checking all their roles
    /// but to simplify things, it's a field within the struct)
    fn is_ta(&self) -> bool;
}

/// An object which is used to represent a user
/// on Discord. If using a library to communicate with
/// Discord, that library would have it's own version of representing
/// users. This is a mock version of that.
/// Note for a production version, it wouldn't have an `is_ta` field.
/// That would instead be a method where you would have to check all of
/// the user's roles and see if any of them match a "TA Role" which would
/// be specified in some sort of config (not modeled in this program)
///
/// For more information about users see <https://discord.com/developers/docs/resources/user>
#[derive(Clone)]  // Allows Rust to create deep copies of the object
struct DiscordUser {
    uuid: u64,
    name: String,
    discriminator: String,
    is_ta: bool,
}

// Allow for printing the struct to the console
impl Debug for DiscordUser {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}#{}", self.name, self.discriminator)
    }
}

/// A representation of a Discord message. When using a library which
/// connects to Discord, that object would be used instead. This is a
/// mockup object which is used to show the bot is working without having
/// to set up and connect it to Discord.
///
/// The fields are based off discord.py's representation which keeps track
/// of the author, their mention, and any users they have mentioned.
/// The message will contain mentions (`<@XXX>`) but this can be annoying to
/// parse. As a result, Discord also returns a list of all users who have
/// been mentioned within the message (order is not guaranteed)
///
/// The <'a> is used to represent that author is a *reference* to a DiscordUser object
struct Message<'a> {
    author: &'a DiscordUser,
    message: String,
    mentions: Option<Vec<DiscordUser>>
}

// Almost like the __eq__ method within Python.
// This allows Rust to compare DiscordUsers and
// see if two structs represent the same object
impl PartialEq for DiscordUser {
    fn eq(&self, other: &Self) -> bool {
        self.uuid == other.uuid
    }
}

// Implementation of User contract methods specifically for `DiscordUser`
impl User for DiscordUser {
    fn get_mention(&self) -> String {
        format!("<@{}>", self.name)
    }

    fn get_tag(&self) -> String {
        format!("{}#{}", self.name, self.discriminator)
    }

    fn get_name(&self) -> String {
        self.name.to_string()
    }

    fn is_ta(&self) -> bool {
        self.is_ta
    }
}

/// Convert a string to a unsigned number
/// It's primarily used to generate uuids for users
/// Example taken from <https://doc.rust-lang.org/std/hash/index.html#examples>
fn calculate_hash<T: Hash>(t: &T) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}

/// Use the Factory Design Pattern to create DiscordUsers.
/// The UUID of the user is a random u64 based derived from the name
/// Arguments
/// * `name` - The name of the user
/// * `discriminator` - A string containing four digits
/// * `is_ta` - Whether or not the user is a TA
///
fn get_user(name: &str, discriminator: &str, is_ta: bool) -> DiscordUser {
    DiscordUser {
        uuid: calculate_hash(&name),
        name: String::from(name),
        discriminator: String::from(discriminator),
        is_ta,
    }
}

/// The struct representing the QueueBot object
/// Currently it only holds a queue. In an actual
/// production implementation, it would also store
/// references to Discord Text, Voice channels, etc.
/// which are used to ensure commands are being sent
/// from the right locations and students are waiting
/// in the proper voice channel for a TA
struct QueueBot {
    /// Users are added to the back of the queue
    /// and are removed from the front
    /// (VecDeque is a double-ended queue that is implemented using a Vector)
    queue: VecDeque<DiscordUser>
}

/// This is an example of implementing a "class" without
/// a trait. To create a new object, use `QueueBot::new()`
impl QueueBot {
    /// Create a new QueueBot object with an empty queue
    pub fn new() -> QueueBot {
        QueueBot { queue: VecDeque::new() }
    }

    /// Event handler for Discord message events. This checks to see if the
    /// message starts with `!q`
    pub fn on_message(&mut self, msg: Message) {
        let lower_message = msg.message.to_ascii_lowercase();
        println!("[{}]: {}", msg.author.name, msg.message);
        if lower_message.starts_with("!q") {
            self.queue_command(msg);
        }
    }

    /// "Send" a message to Discord.
    /// In reality, this just prints the message to stdout
    fn send(&self, message: String, cmd_prefix: Option<MessageType>) {
        let prefix = match cmd_prefix {
            None => "",
            Some(t) => match t {
                MessageType::Warning => "⚠️ ",
                MessageType::Success => "✅ ",
                MessageType::Error => "‼️ ",
            },
        };
        println!("SEND> {}{}\n", prefix, message)
    }

    /// Given a message that starts with `!q`, send it to be handled by the appropriate command handler
    fn queue_command(&mut self, msg: Message) {
        let lower_message = msg.message.to_ascii_lowercase();
        let split_message = lower_message.split_ascii_whitespace().collect::<Vec<&str>>();

        if split_message.len() < 2 {
            self.send(
            format!("{} invalid syntax.", msg.author.get_mention()),
            Some(MessageType::Warning))
        }

        match *split_message.get(1).unwrap() {
            // A list of student commands
            "ping" => self.queue_ping(),
            "join" => self.q_join(msg.author),
            "leave" => self.q_leave(msg.author),
            "pos" => self.q_position(msg.author),
            "position" => self.q_position(msg.author),
            "list" => self.q_list(),

            // Check if they are a TA. If they are, compare against TA commands
            _ => match msg.author.is_ta() {
                false => self.send_invalid_syntax(msg.author),
                true => match *split_message.get(1).unwrap() {
                    "next" => self.q_next(msg.author),
                    "clear" => self.q_clear(msg.author),
                    "add" => self.q_add_other(msg.author, msg.mentions),
                    "remove" => self.q_remove_other(msg.author, msg.mentions),
                    _ => self.send_invalid_syntax(msg.author)
                },
            }
        }

    }

    /// Notify the user the command they typed was invalid
    fn send_invalid_syntax(&self, user: &DiscordUser) {
        self.send(
            format!("{} invalid format.", user.get_mention()),
            Some(MessageType::Warning));
    }

    /// Respond with a friendly "Pong!"
    fn queue_ping(&self) {
        self.send(String::from("Pong!"), None);
    }

    /// Add the given user to the queue (at the end)
    /// and send a response message
    fn q_join(&mut self, user: &DiscordUser) {
        if self.queue.contains(&user) {
            self.send(String::from("You are already in the queue!"), Some(MessageType::Warning));
            return;
        }
        self.queue.push_back(user.clone());
        self.send(format!("{} You have been added to the queue at position {}", user.get_mention(), self.queue.len()), Some(MessageType::Success));
    }

    /// Remove the given user from the queue
    /// and send a response message
    fn q_leave(&mut self, user: &DiscordUser) {
        // Get position of user within Queue via brute-force search
        match self.queue.iter().position(|r| r.uuid == user.uuid) {
            Some(i) => {
                self.queue.remove(i);
                self.send(format!("{} You have been removed from the queue", user.get_mention()), Some(MessageType::Success));
            },
            None => self.send(format!("{} You are not in the queue!", user.get_mention()), Some(MessageType::Warning)),
        }
    }

    /// Notify the user what position (index+1)
    /// they currently are at from within the queue
    fn q_position(&self, user: &DiscordUser) {
        match self.queue.iter().position(|r| r.uuid == user.uuid) {
            Some(i) => {
                self.send(format!("{} You are at position {}", user.get_mention(), i+1), None);
            },
            None => self.send(format!("{} You are not in the queue!", user.get_mention()), Some(MessageType::Warning)),
        }
    }

    /// List out all students within the queue
    fn q_list(&self) {
        self.send(format!("Queue: {:?}", self.queue), None)
    }

    /// Assumes the user is a TA. It pops the next person off the
    /// queue and notifies the TA of what student is next
    fn q_next(&mut self, user: &DiscordUser) {
        if self.queue.len() == 0 {
            self.send(format!("{} There is no one in the queue", user.get_mention()), None);
            return;
        }

        let student = self.queue.pop_front().unwrap();
        self.send(format!("The next person in line is {}", student.get_mention()), None)
    }

    /// Assumes the user is a TA.
    /// Completely empty the queue content
    /// Within actual QueueBot, a confirmation message
    /// is sent but this is omitted to make things simpler
    fn q_clear(&mut self, user: &DiscordUser) {
        self.queue.clear();
        self.send(format!("{} The queue has been cleared", user.get_mention()), Some(MessageType::Success));
    }

    /// Assumes the user is a TA.
    /// The user must mention a single person and that person
    /// will then be added to the queue
    fn q_add_other(&mut self, user: &DiscordUser, mentions: Option<Vec<DiscordUser>>) {
        match mentions {
            None => self.send(format!("{} You must `@mention` a user!", user.get_mention()), Some(MessageType::Warning)),
            Some(v) => if v.len() > 1 {
                self.send(format!("{} you must mention a single user!", user.get_mention()), Some(MessageType::Warning))
            } else {
                let student = v.get(0).unwrap();
                if self.queue.contains(student) {
                    self.send(format!("{} That student is already in the queue!", user.get_mention()), Some(MessageType::Warning));
                } else {
                    self.queue.push_back(student.clone());
                    self.send(format!("{} The student has been added to the queue!", user.get_mention()), Some(MessageType::Success));
                }
            },
        }
    }

    /// Assumes the user is a TA.
    /// The user must mention a single person and that person
    /// will then be removed from the queue
    fn q_remove_other(&mut self, user: &DiscordUser, mentions: Option<Vec<DiscordUser>>) {
        match mentions {
            None => self.send(format!("{} You must `@mention` a user!", user.get_mention()), Some(MessageType::Warning)),
            Some(v) => if v.len() > 1 {
                self.send(format!("{} you must mention a single user!", user.get_mention()), Some(MessageType::Warning))
            } else {
                let student = v.get(0).unwrap();
                if !self.queue.contains(student) {
                    self.send(format!("{} The specified student is not in the queue!", user.get_mention()), Some(MessageType::Warning));
                    return;
                }
                let pos = self.queue.iter().position(|r| r.uuid == student.uuid).unwrap();
                self.queue.remove(pos);
                self.send(format!("{} The student has been removed from the queue!", user.get_mention()), Some(MessageType::Success));
            },
        }
    }
}


fn main() {
    let ta = get_user("Ben", "0001", true);
    let students = vec![
        get_user("Kapua", "0002", false),
        get_user("Bennett", "0003", false),
        get_user("Russ", "0004", false),
        get_user("Jordan", "0003", false),
    ];

    let mut bot = QueueBot::new();

    // TA pings the bot to make sure it's working (responds with "Pong!")
    bot.on_message(Message {
        author: &ta, message: String::from("!q ping"), mentions: None
    });

    // Every student joins the queue
    for student in students.iter() {
        bot.on_message(Message {
            author: &student, message: String::from("!q join"), mentions: None
        });
    }

    // A list of all the students in the queue
    bot.on_message(Message {
        author: &ta, message: String::from("!q list"), mentions: None
    });

    // TA should not be removed from the queue (since he never joined)
    bot.on_message(Message {
        author: &ta, message: String::from("!q leave"), mentions: None
    });

    // Russ decides to leave the queue
    bot.on_message(Message {
        author: &students.get(2).unwrap(), message: String::from("!q leave"), mentions: None
    });

    // Russ checks to make sure he's not in the queue
    bot.on_message(Message {
        author: &students.get(2).unwrap(), message: String::from("!q pos"), mentions: None
    });

    // Jordan checks his position in the queue
    bot.on_message(Message {
        author: &students.get(3).unwrap(), message: String::from("!q position"), mentions: None
    });

    // TA lists the queue again after Russ leaves
    bot.on_message(Message {
        author: &ta, message: String::from("!q list"), mentions: None
    });

    // TA grabs pops next person off the queue
    bot.on_message(Message {
        author: &ta, message: String::from("!q next"), mentions: None
    });

    // TA clears the queue
    bot.on_message(Message {
        author: &ta, message: String::from("!q clear"), mentions: None
    });

    // Russ lists the queue in confusion
    bot.on_message(Message {
        author: &students.get(2).unwrap(), message: String::from("!q list"), mentions: None
    });

    // TA adds Kapua, Bennett, Russ, and Jordan back into the queue
    for i in 0..students.len() {
        let student = students.get(i).unwrap();
        bot.on_message(Message {
            author: &ta, message: format!("!q add {}", student.get_mention()), mentions: Some(vec![student.clone()])
        });
    }

    // Russ lists queue again
    bot.on_message(Message {
        author: &students.get(2).unwrap(), message: String::from("!q list"), mentions: None
    });


    // TA removes (still confused) Russ from the queue
    let russ = students.get(2).unwrap();
    bot.on_message(Message {
        author: &ta, message: format!("!q remove {}", russ.get_mention()), mentions: Some(vec![russ.clone()])
    });

    // Kapua lists queue
    bot.on_message(Message {
        author: &students.get(0).unwrap(), message: String::from("!q list"), mentions: None
    });

}
