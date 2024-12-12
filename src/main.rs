use bsbo_01_22_team1_var13::{Account, Advertisement, Board, Description, Error, Item, Query};
use color_eyre::owo_colors::OwoColorize;

fn main() -> Result<(), eyre::Report> {
    color_eyre::install()?;
    setup_tracing()?;

    let mut board = Board::load()?;
    let user_uuid = Account::TEST_USER_UUID;

    // По умолчанию доска ограничит количество объявлений до 20,
    // для тестирования заставим её выводить *все* результаты.
    // P.S. `usize::MAX` это 18446744073709551615 на x86_64.
    board.page_length = usize::MAX;

    // Создадим новое "пользовательское" объявление на доске.
    // Считаем, что оно сразу же пройдёт процесс модерации.
    {
        let seller = Account::test_seller();
        let item = Item::create("User-created advertisement", 750);
        let description = Description::create("bla bla bla", vec![(), ()]);
        let mut advertisement = Advertisement::create(item, description, seller);
        advertisement.confirm_moderation();
        board.add_advertisement(advertisement);
    }

    // Просмотрим все объявления на доске.
    {
        eprint_title("ALL ADVERTISEMENTS:");
        for adv in board.view_advertisements() {
            eprintln!("{adv:#?}\n");
        }
    }

    // Выполним поиск по доске, добавим найденные объявления в корзину.
    {
        let search_string = String::from("user");
        //                               ^^^^^^
        // Должно найти "user-created advertisement" из кода выше.

        eprint_title(&format!("SEARCH RESULTS (pattern = {search_string:?}):"));
        let results = board
            .search_advertisements(&Query::new().search_string(search_string).build())
            .cloned()
            .collect::<Vec<_>>();
        for adv in &results {
            eprintln!("{adv:#?}\n");
        }

        // Добавляем в корзину юзера все объявления, которые подошли под поиск.
        board.extend_cart(user_uuid, results.iter().map(|ad| ad.item.clone()))?;
    }

    // Оформим заказ для пользователя.
    {
        board.place_order(user_uuid)?;

        // Увидим, что `Delivery` сохранился в архив заказов
        // пользователя вместе с соответствующим `Payment`.
        let user = board
            .get_user_mut(user_uuid)
            .ok_or(Error::UserNotFound(user_uuid))?;
        eprint_title("USER'S PAST ORDERS:");
        dbg!(&user.past_orders);

        eprintln!();
        eprint_title("TRACKING THE LAST ORDER:");
        match user.past_orders.iter().next() {
            None => unreachable!(),
            Some((delivery, _)) => {
                for (status, datetime) in delivery.track() {
                    eprintln!("{}: {}", datetime.bold(), status.bold().blue());
                }
            }
        };
    }

    Ok(())
}

fn eprint_title(text: &str) {
    let text = format!(" {text} ");
    eprintln!("{}", text.bold().black().on_magenta());
}

// Настройка и установка `tracing_subscriber` с захватом
// входа в Span каждой функции с `#[tracing::instrument]`.
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
