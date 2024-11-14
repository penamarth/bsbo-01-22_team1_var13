use bsbo_01_22_team1_var13::Account;

fn main() -> Result<(), ()> {
    let guest = Account::unauthenticated();

    // Do stuff as a guest?

    let _user = guest.authenticate_as_user()?;

    // Do stuff as a user?

    let _moderator = Account::unauthenticated().authenticate_as_moderator()?;

    // Do stuff as a moderator?

    Ok(())
}
