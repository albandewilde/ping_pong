use serenity::model::user::User;

pub fn discord_username(user: &User) -> String {
    String::from(&user.name) + "#" + &format!("{:04}", &user.discriminator)
}

pub fn format_msg(msg: &str) -> String {
    // Remove the prefix of the message and lowercase it
    let mut v = Vec::new(); // Vector to store our characters
    v.extend(msg.chars());
    let chrs = match v.get(1..) {
        Some(chrs) => chrs,
        None => &[],
    };
    chrs.iter().collect::<String>().to_lowercase()
}
