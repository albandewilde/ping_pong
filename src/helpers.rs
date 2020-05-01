use serenity::model::user::User;

pub fn discord_username(user: &User) -> String {
    String::from(&user.name) + "#" + &format!("{:04}", &user.discriminator)
}
