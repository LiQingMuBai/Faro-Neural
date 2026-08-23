use reqwest::Client;
use serde::Deserialize;
use serde_json::{Value, json};
use std::{collections::HashMap, env, fs, sync::Arc, time::Duration};
use tokio::sync::RwLock;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

#[derive(Clone)]
struct Bot {
    client: Client,
    api: String,
    site: String,
    customer_service_url: String,
    languages: Arc<RwLock<HashMap<i64, Language>>>,
}

#[derive(Clone, Copy)]
enum Language {
    En,
    Es,
}

#[derive(Deserialize)]
struct ApiResponse<T> {
    ok: bool,
    result: Option<T>,
    description: Option<String>,
}

#[derive(Deserialize)]
struct Update {
    update_id: i64,
    message: Option<Message>,
    callback_query: Option<CallbackQuery>,
}

#[derive(Deserialize)]
struct Message {
    chat: Chat,
    from: Option<User>,
    text: Option<String>,
}

#[derive(Deserialize)]
struct Chat {
    id: i64,
}

#[derive(Deserialize)]
struct User {
    id: i64,
}

#[derive(Deserialize)]
struct CallbackQuery {
    id: String,
    from: User,
    message: Option<Message>,
    data: Option<String>,
}

fn load_env() {
    let Ok(raw) = fs::read_to_string(".env") else {
        return;
    };
    for line in raw
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty() && !line.starts_with('#'))
    {
        let Some((key, value)) = line.split_once('=') else {
            continue;
        };
        if env::var_os(key.trim()).is_none() {
            unsafe { env::set_var(key.trim(), value.trim().trim_matches(['\'', '"'])) };
        }
    }
}

fn text(lang: Language, key: &str) -> &'static str {
    match (lang, key) {
        (Language::En, "welcome") => {
            "🤖 <b>Numerai Community Assistant</b>\n\nExplore the AI data-science experience, official Numerai resources, videos, and community links.\n\n⚠️ This independent bot is not Numerai and does not provide financial or investment advice."
        }
        (Language::Es, "welcome") => {
            "🤖 <b>Asistente de la comunidad Numerai</b>\n\nExplora la experiencia de ciencia de datos con IA, recursos oficiales de Numerai, vídeos y enlaces de la comunidad.\n\n⚠️ Este bot independiente no es Numerai y no ofrece asesoramiento financiero ni de inversión."
        }
        (Language::En, "help") => {
            "Choose an option or use:\n/start — Main menu\n/socials — Official links\n/join — Join our team and get $10 mobile credit\n/legal — Legal notice\n/language — English / Español"
        }
        (Language::Es, "help") => {
            "Elige una opción o usa:\n/start — Menú principal\n/socials — Enlaces oficiales\n/join — Únete al equipo y recibe $10 de saldo móvil\n/legal — Aviso legal\n/language — English / Español"
        }
        (Language::En, "legal") => {
            "🚫 <b>ZERO TOLERANCE FOR ILLEGAL ACTIVITY</b>\n\nIllegal use is strictly and absolutely prohibited. Do not use this bot, website, code, data, models, links, or infrastructure to commit, facilitate, plan, conceal, promote, or assist any unlawful activity.\n\nAny unlawful conduct is undertaken solely by the person involved, without this bot’s or website’s authorization, participation, endorsement, or benefit, and is unrelated to them. The responsible person bears sole responsibility for all consequences. Access may be blocked, relevant records preserved, and competent authorities assisted where required or permitted by law.\n\n🌍 <b>REGIONAL ACCESS RESTRICTIONS</b>\n\nThis service is not available to users located in Mainland China, Hong Kong, Macao, North Korea, Russia, Iran, Syria, Cuba, or Belarus. Accessing or offering access to this service from these locations is prohibited. This list may be updated to reflect applicable laws, sanctions, and service-availability requirements.\n\n<b>If you intend to engage in illegal activity or are located in a restricted region, do not use this bot or website.</b>"
        }
        (Language::Es, "legal") => {
            "🚫 <b>TOLERANCIA CERO FRENTE A ACTIVIDADES ILEGALES</b>\n\nEl uso ilegal está estricta y absolutamente prohibido. No utilices este bot, sitio, código, datos, modelos, enlaces o infraestructura para cometer, facilitar, planificar, ocultar, promover o ayudar en ninguna actividad ilícita.\n\nToda conducta ilegal es realizada exclusivamente por la persona implicada, sin autorización, participación, aprobación ni beneficio del bot o del sitio, y no guarda relación con ellos. La persona responsable asume íntegramente todas las consecuencias. Se podrá bloquear el acceso, conservar registros y colaborar con las autoridades cuando la ley lo exija o permita.\n\n🌍 <b>RESTRICCIONES REGIONALES DE ACCESO</b>\n\nEste servicio no está disponible para usuarios ubicados en China continental, Hong Kong, Macao, Corea del Norte, Rusia, Irán, Siria, Cuba o Bielorrusia. Está prohibido acceder u ofrecer acceso a este servicio desde estas ubicaciones. Esta lista puede actualizarse para reflejar las leyes, sanciones y requisitos de disponibilidad aplicables.\n\n<b>Si pretendes realizar una actividad ilegal o te encuentras en una región restringida, no utilices este bot ni el sitio.</b>"
        }
        (Language::En, "choose") => "Please choose an option:",
        (Language::Es, "choose") => "Elige una opción:",
        (Language::En, "website") => "🌐 Website",
        (Language::Es, "website") => "🌐 Sitio web",
        (Language::En, "socials") => "💬 Official socials",
        (Language::Es, "socials") => "💬 Redes oficiales",
        (Language::En, "videos") => "▶️ Official videos",
        (Language::Es, "videos") => "▶️ Vídeos oficiales",
        (Language::En, "join_team") => "🎁 Join our team",
        (Language::Es, "join_team") => "🎁 Únete a nuestro equipo",
        (Language::En, "join_promo") => {
            "🎁 <b>Join our team and get $10 in mobile credit!</b>\n\nBecome part of our community and explore AI data science with us. Contact customer service to join and claim your $10 mobile credit."
        }
        (Language::Es, "join_promo") => {
            "🎁 <b>¡Únete a nuestro equipo y recibe $10 de saldo móvil!</b>\n\nForma parte de nuestra comunidad y explora la ciencia de datos con IA. Contacta con atención al cliente para unirte y solicitar tus $10 de saldo móvil."
        }
        (Language::En, "contact_service") => "Contact customer service",
        (Language::Es, "contact_service") => "Contactar con atención al cliente",
        (Language::En, "legal_btn") => "⚠️ Legal notice",
        (Language::Es, "legal_btn") => "⚠️ Aviso legal",
        (Language::En, "language") => "🌍 Language",
        (Language::Es, "language") => "🌍 Idioma",
        (Language::En, "back") => "⬅️ Main menu",
        (Language::Es, "back") => "⬅️ Menú principal",
        _ => "",
    }
}

fn main_keyboard(lang: Language, site: &str) -> Value {
    json!({"inline_keyboard":[
        [{"text":text(lang,"website"),"url":site},{"text":text(lang,"socials"),"callback_data":"socials"}],
        [{"text":text(lang,"videos"),"callback_data":"videos"},{"text":text(lang,"legal_btn"),"callback_data":"legal"}],
        [{"text":text(lang,"join_team"),"callback_data":"join_team"}],
        [{"text":text(lang,"language"),"callback_data":"language"}]
    ]})
}

fn back_keyboard(lang: Language) -> Value {
    json!({"inline_keyboard":[[{"text":text(lang,"back"),"callback_data":"home"}]]})
}

fn join_keyboard(lang: Language, customer_service_url: &str) -> Value {
    json!({"inline_keyboard":[
        [{"text":text(lang,"contact_service"),"url":customer_service_url}],
        [{"text":text(lang,"back"),"callback_data":"home"}]
    ]})
}

fn socials() -> &'static str {
    "🌐 <b>Official Numerai links</b>\n\n• <a href=\"https://numer.ai\">Website</a>\n• <a href=\"https://blog.numer.ai\">Blog</a>\n• <a href=\"https://x.com/numerai\">X / Twitter</a>\n• <a href=\"https://discord.gg/numerai\">Discord</a>\n• <a href=\"https://github.com/numerai\">GitHub</a>\n• <a href=\"https://forum.numer.ai\">Forum</a>\n• <a href=\"https://numer.ai/privacy\">Privacy Policy</a>\n• <a href=\"https://numer.ai/terms\">Terms of Service</a>"
}

fn videos() -> &'static str {
    "▶️ <b>Official Numerai videos</b>\n\n• <a href=\"https://www.youtube.com/watch?v=dhJnt0N497c\">The Hardest Data Science Tournament</a>\n• <a href=\"https://vimeo.com/205032211\">Introducing Numeraire</a>\n• <a href=\"https://www.youtube.com/watch?v=GWeC2PK4yXQ\">Numerai Signals</a>"
}

impl Bot {
    async fn request<T: for<'de> Deserialize<'de>>(&self, method: &str, body: Value) -> Result<T> {
        let response = self
            .client
            .post(format!("{}/{method}", self.api))
            .json(&body)
            .send()
            .await?;
        let result: ApiResponse<T> = response.json().await?;
        if !result.ok {
            return Err(result
                .description
                .unwrap_or_else(|| "Telegram API error".into())
                .into());
        }
        result
            .result
            .ok_or_else(|| "Telegram API returned no result".into())
    }

    async fn send(&self, chat_id: i64, message: &str, keyboard: Value) -> Result<()> {
        let _: Value = self.request("sendMessage", json!({"chat_id":chat_id,"text":message,"parse_mode":"HTML","disable_web_page_preview":true,"reply_markup":keyboard})).await?;
        Ok(())
    }

    async fn language(&self, user_id: i64) -> Language {
        *self
            .languages
            .read()
            .await
            .get(&user_id)
            .unwrap_or(&Language::En)
    }

    async fn home(&self, chat_id: i64, lang: Language) -> Result<()> {
        self.send(
            chat_id,
            &format!("{}\n\n{}", text(lang, "welcome"), text(lang, "choose")),
            main_keyboard(lang, &self.site),
        )
        .await
    }

    async fn handle(&self, update: Update) -> Result<()> {
        if let Some(callback) = update.callback_query {
            let _: Value = self
                .request(
                    "answerCallbackQuery",
                    json!({"callback_query_id":callback.id}),
                )
                .await?;
            let Some(message) = callback.message else {
                return Ok(());
            };
            let chat_id = message.chat.id;
            let user_id = callback.from.id;
            let action = callback.data.unwrap_or_default();
            if action == "lang_en" || action == "lang_es" {
                let lang = if action == "lang_es" {
                    Language::Es
                } else {
                    Language::En
                };
                self.languages.write().await.insert(user_id, lang);
                return self.home(chat_id, lang).await;
            }
            let lang = self.language(user_id).await;
            return match action.as_str() {
                "home"=>self.home(chat_id,lang).await,
                "socials"=>self.send(chat_id,socials(),back_keyboard(lang)).await,
                "videos"=>self.send(chat_id,videos(),back_keyboard(lang)).await,
                "join_team"=>self.send(chat_id,text(lang,"join_promo"),join_keyboard(lang,&self.customer_service_url)).await,
                "legal"=>self.send(chat_id,text(lang,"legal"),back_keyboard(lang)).await,
                "language"=>self.send(chat_id,"🌍 Choose language / Elige idioma:",json!({"inline_keyboard":[[{"text":"English","callback_data":"lang_en"},{"text":"Español","callback_data":"lang_es"}]]})).await,
                _=>Ok(())
            };
        }
        let Some(message) = update.message else {
            return Ok(());
        };
        let chat_id = message.chat.id;
        let user_id = message.from.map(|u| u.id).unwrap_or(chat_id);
        let lang = self.language(user_id).await;
        let command = message
            .text
            .unwrap_or_default()
            .split_whitespace()
            .next()
            .unwrap_or("")
            .split('@')
            .next()
            .unwrap_or("")
            .to_lowercase();
        match command.as_str() {
            "/start"=>self.home(chat_id,lang).await,
            "/socials"=>self.send(chat_id,socials(),back_keyboard(lang)).await,
            "/join"=>self.send(chat_id,text(lang,"join_promo"),join_keyboard(lang,&self.customer_service_url)).await,
            "/legal"=>self.send(chat_id,text(lang,"legal"),back_keyboard(lang)).await,
            "/language"=>self.send(chat_id,"🌍 Choose language / Elige idioma:",json!({"inline_keyboard":[[{"text":"English","callback_data":"lang_en"},{"text":"Español","callback_data":"lang_es"}]]})).await,
            _=>self.send(chat_id,text(lang,"help"),main_keyboard(lang,&self.site)).await,
        }
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    load_env();
    let token = env::var("TELEGRAM_BOT_TOKEN").map_err(
        |_| "Missing TELEGRAM_BOT_TOKEN. Copy .env.example to .env and add the BotFather token.",
    )?;
    if token.contains("replace_with") {
        return Err("Replace the sample TELEGRAM_BOT_TOKEN in .env".into());
    }
    let bot = Bot {
        client: Client::new(),
        api: format!("https://api.telegram.org/bot{token}"),
        site: env::var("PUBLIC_SITE_URL")
            .unwrap_or_else(|_| "http://localhost:5173".into())
            .trim_end_matches('/')
            .into(),
        customer_service_url: env::var("CUSTOMER_SERVICE_URL")
            .or_else(|_| env::var("PUBLIC_SITE_URL"))
            .unwrap_or_else(|_| "http://localhost:5173".into()),
        languages: Arc::new(RwLock::new(HashMap::new())),
    };
    let _: Value=bot.request("setMyCommands",json!({"commands":[
        {"command":"start","description":"Open the main menu"},{"command":"socials","description":"Official Numerai links"},{"command":"join","description":"Join our team and get $10 mobile credit"},{"command":"legal","description":"Legal notice"},{"command":"language","description":"English / Español"},{"command":"help","description":"Help"}
    ]})).await?;
    println!("Rust Telegram bot is running. Press Ctrl+C to stop.");
    let mut offset = 0_i64;
    loop {
        tokio::select! {
            _=tokio::signal::ctrl_c()=>{println!("Stopping bot…");break;}
            result=bot.request::<Vec<Update>>("getUpdates",json!({"offset":offset,"timeout":25,"allowed_updates":["message","callback_query"]}))=>{
                match result {
                    Ok(updates)=>for update in updates {offset=update.update_id+1;if let Err(error)=bot.handle(update).await{eprintln!("Update error: {error}");}},
                    Err(error)=>{eprintln!("Polling error: {error}");tokio::time::sleep(Duration::from_secs(3)).await;}
                }
            }
        }
    }
    Ok(())
}
