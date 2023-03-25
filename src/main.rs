use teloxide::{
    prelude::*,
    utils::command::BotCommands,
    types::*
};
use rand::{Rng, thread_rng};
use dotenv::var;
use os_info::get;

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    log::info!("Starting command bot...");

    let bot = Bot::new(var("TOKEN").unwrap());

    Command::repl(bot, answer).await;
}

#[derive(BotCommands, Clone)]
#[command(rename_rule = "lowercase", description = "Все команды:")]
enum Command {
    #[command(description = "обычный старт.")]
    Start,
    #[command(description = "показывает этот текст.")]
    Help,
    #[command(description = "данные о системе.")]
    Sys,
    #[command(description = "рандомное место на карте.")]
    RandCoord
}

async fn answer(bot: Bot, msg: Message, cmd: Command) -> ResponseResult<()> {
    match cmd {
        Command::Start => {
            let st = InputFile::file_id(String::from("CAACAgIAAxkBAAEIS7NkHfz_W7aRHE3-LYCeXrCuCeGeawACsSYAAlpVkEpU6wfG25Y6fy8E"));
            bot.send_sticker(msg.chat.id, st).await?;
        }
        Command::Help => {
            bot.send_message(msg.chat.id, Command::descriptions().to_string()).await?;
        }
        Command::Sys => {
            let info = get();
            bot.send_message(msg.chat.id, info.to_string()).await?;
        },
        Command::RandCoord => {
            let coord: (f64, f64) = async {
                let mut rng = thread_rng();
                let latitude: f64 = rng.gen_range(-90.0..90.0);
                let longitude: f64 = rng.gen_range(-90.0..90.0);
                (latitude, longitude)
            }.await;
            bot.send_location(msg.chat.id, coord.0, coord.1).await?;
        }
    };

    Ok(())
}