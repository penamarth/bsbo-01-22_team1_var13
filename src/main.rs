use bsbo_01_22_team1_var13::Account;

fn main() -> Result<(), eyre::Report> {
    color_eyre::install()?;

    let guest = Account::unauthenticated();
    let _ = guest.view_board();
    let _ = guest.search_board("stuff");

    let _user = guest.authenticate_as_user()?;

    // Do stuff as a user?

    let _moderator = Account::unauthenticated().authenticate_as_moderator()?;

    // Do stuff as a moderator?

    Ok(())
}
