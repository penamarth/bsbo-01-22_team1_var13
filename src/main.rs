use bsbo_01_22_team1_var13::{Account, Advertisement, Board, Description, Error, Item, Query};
use color_eyre::owo_colors::OwoColorize;
use itertools::Itertools;

fn main() -> Result<(), eyre::Report> {
    color_eyre::install()?;
    setup_tracing()?;

    let mut board = Board::load();
    let user_uuid = Account::TEST_USER_UUID;

    // По умолчанию доска ограничит количество результатов до 20, для тестирования заставим её выводить *все* результаты.
    board.page_length = usize::MAX;

    // Создадим новое "пользовательское" объявление на доске.
    {
        let seller = Account::test_seller();
        let item = Item::create("User-created advertisement", 750);
        let description = Description::create("bla bla bla", vec![(), ()]);
        let advertisement = Advertisement::create(item, description, seller);
        board.add_advertisement(advertisement);
    }

    // Просмотрим все объявления на доске.
    {
        eprintln!("{}", "BOARD CONTENTS:".bold().black().on_magenta());
        for adv in board.view_advertisements() {
            eprintln!("{adv}\n");
        }
    }

    // Выполним поиск по доске, добавим найденные объявления в корзину.
    {
        let search_string = String::from("user");
        eprintln!(
            "{}",
            format!("SEARCH RESULTS (pattern = {search_string:?}):")
                .bold()
                .black()
                .on_magenta()
        );
        let query = Query { search_string };
        let results = board.search_advertisements(&query).cloned().collect_vec();
        for adv in &results {
            eprintln!("{adv}\n");
        }

        board.extend_cart(user_uuid, results.iter().map(|ad| ad.item.clone()))?;
    }

    // Оформим заказ для пользователя.
    {
        board.checkout_cart(user_uuid)?;
        let _user = board
            .get_user_mut(user_uuid)
            .ok_or(Error::UserNotFound(user_uuid))?;
    }

    Ok(())
}

fn setup_tracing() -> Result<(), eyre::Report> {
    use tracing_error::ErrorLayer;
    use tracing_subscriber::fmt::format::FmtSpan;
    use tracing_subscriber::{fmt, prelude::*, EnvFilter};

    let filter_layer = EnvFilter::try_from_default_env().or_else(|_| EnvFilter::try_new("info"))?;
    let format_layer = fmt::layer()
        .without_time()
        .with_target(false)
        .with_span_events(FmtSpan::ENTER);

    tracing_subscriber::registry()
        .with(filter_layer)
        .with(format_layer)
        .with(ErrorLayer::default())
        .try_init()?;

    Ok(())
}
