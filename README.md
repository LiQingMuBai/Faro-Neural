# Faro Neural

> **Faro Neural** means “Neural Beacon” in Spanish: a beacon for exploring artificial intelligence, data science, and quantitative modeling responsibly.

[English](#english) · [Español](#español)

---

## English

### About

Faro Neural is a bilingual, responsive AI data-science experience inspired by the visual language of Numerai. It combines a React website with an asynchronous Telegram bot written entirely in Rust.

The project provides educational interfaces, official Numerai community links, videos, documentation shortcuts, and prominent responsible-use notices. It is an independent project and is not affiliated with, endorsed by, or operated by Numerai.

### Features

- Responsive dark, technical interface
- English and Spanish language switching
- Interactive terminal, data, model, and statistics sections
- Official Numerai videos and community links
- Privacy Policy and Terms of Service links to Numerai
- Rust Telegram bot with inline menus and long polling
- Strong zero-tolerance notice for unlawful activity
- Production build powered by Vite

### Technology

- Website: React 18, Vite, CSS, Lucide icons
- Telegram bot: Rust 2024, Tokio, Reqwest, Serde, Rustls
- Integrations: Telegram Bot API and official Numerai public resources

### Requirements

- Node.js 18 or newer
- npm
- A current stable Rust toolchain
- A Telegram Bot Token from `@BotFather`

### Build with Make

The project includes a Makefile that provides one interface for the website and Rust bot:

```bash
make help        # list all commands
make install     # install/fetch dependencies
make dev         # start the website
make check       # validate website and bot
make build       # production website + release Rust bot
make bot         # run the Telegram bot
make clean       # remove generated artifacts
```

For a clean production build, run `make release`.

### Website setup

```bash
npm install
npm run dev
```

Open `http://localhost:5173`.

Production build:

```bash
npm run build
npm run preview
```

### Telegram bot setup

Create the local environment file:

```bash
cp .env.example .env
```

Configure it without committing the token:

```env
TELEGRAM_BOT_TOKEN=your_botfather_token
PUBLIC_SITE_URL=https://your-public-domain.example
```

Run the bot:

```bash
npm run bot
```

Or run it directly with Cargo:

```bash
cargo run --release --manifest-path bot/Cargo.toml
```

Telegram users cannot access your computer through a `localhost` link. Set `PUBLIC_SITE_URL` to a deployed HTTPS address before publishing the bot.

### Bot commands

- `/start` — main menu
- `/legal` — legal and responsible-use notice
- `/help` — command help

### Legal and responsible use

Illegal use is strictly and absolutely prohibited. No person may use this project, website, bot, code, data, models, links, or infrastructure to commit, facilitate, plan, conceal, promote, or assist fraud, market manipulation, unauthorized access, money laundering, sanctions evasion, infringement, or any other unlawful conduct.

Any unlawful act is undertaken solely by the person involved, without this project’s authorization, participation, endorsement, or benefit. The responsible person bears responsibility for their conduct and its consequences. Access may be blocked, relevant records preserved, and competent authorities assisted where required or permitted by law.

This project is educational and does not provide financial, investment, legal, or tax advice. Nothing in this notice excludes responsibility that cannot lawfully be excluded. Obtain professional legal review before commercial deployment.

### Trademark and attribution

Numerai and related names, marks, media, and linked resources belong to their respective owners. Faro Neural is an independent interface demonstration. Official external links direct users to Numerai-controlled destinations.

The homepage background follows Numerai's referenced artwork: Trevor Paglen, *CLOUD #865 Hough Circle Transform* (2019). The image is retained locally for interface fidelity; ownership and all applicable rights remain with their respective rights holders.

---

## Español

### Acerca del proyecto

Faro Neural es una experiencia bilingüe y adaptable de ciencia de datos con IA, inspirada en el lenguaje visual de Numerai. Combina un sitio web en React con un bot asíncrono de Telegram escrito íntegramente en Rust.

El proyecto ofrece interfaces educativas, enlaces oficiales de la comunidad Numerai, vídeos, accesos a documentación y avisos destacados de uso responsable. Es un proyecto independiente y no está afiliado, respaldado ni operado por Numerai.

### Funciones

- Interfaz técnica oscura y adaptable
- Cambio de idioma entre inglés y español
- Terminal interactiva y secciones de datos, modelos y estadísticas
- Vídeos y enlaces oficiales de la comunidad Numerai
- Enlaces a la Política de privacidad y los Términos de servicio de Numerai
- Bot de Telegram en Rust con menús integrados y sondeo prolongado
- Aviso firme de tolerancia cero frente a actividades ilegales
- Compilación de producción con Vite

### Tecnología

- Sitio web: React 18, Vite, CSS e iconos Lucide
- Bot de Telegram: Rust 2024, Tokio, Reqwest, Serde y Rustls
- Integraciones: API de bots de Telegram y recursos públicos oficiales de Numerai

### Requisitos

- Node.js 18 o posterior
- npm
- Una versión estable y actual de Rust
- Un token de bot de Telegram obtenido mediante `@BotFather`

### Compilación con Make

El proyecto incluye un Makefile que unifica las tareas del sitio y del bot en Rust:

```bash
make help        # mostrar todos los comandos
make install     # instalar/descargar dependencias
make dev         # iniciar el sitio web
make check       # validar el sitio y el bot
make build       # sitio de producción + bot Rust optimizado
make bot         # ejecutar el bot de Telegram
make clean       # eliminar artefactos generados
```

Para obtener una compilación de producción limpia, ejecuta `make release`.

### Instalación del sitio web

```bash
npm install
npm run dev
```

Abre `http://localhost:5173`.

Compilación de producción:

```bash
npm run build
npm run preview
```

### Configuración del bot de Telegram

Crea el archivo de entorno local:

```bash
cp .env.example .env
```

Configúralo sin guardar el token en el repositorio:

```env
TELEGRAM_BOT_TOKEN=tu_token_de_botfather
PUBLIC_SITE_URL=https://tu-dominio-publico.example
```

Ejecuta el bot:

```bash
npm run bot
```

O ejecútalo directamente con Cargo:

```bash
cargo run --release --manifest-path bot/Cargo.toml
```

Los usuarios de Telegram no pueden acceder a tu equipo mediante un enlace `localhost`. Antes de publicar el bot, configura `PUBLIC_SITE_URL` con una dirección HTTPS desplegada.

### Comandos del bot

- `/start` — menú principal
- `/legal` — aviso legal y de uso responsable
- `/help` — ayuda de comandos

### Uso legal y responsable

El uso ilegal está estricta y absolutamente prohibido. Nadie puede utilizar este proyecto, sitio, bot, código, datos, modelos, enlaces o infraestructura para cometer, facilitar, planificar, ocultar, promover o ayudar en fraudes, manipulación del mercado, acceso no autorizado, blanqueo de capitales, evasión de sanciones, infracciones u otras conductas ilícitas.

Todo acto ilegal es realizado exclusivamente por la persona implicada, sin autorización, participación, aprobación ni beneficio de este proyecto. La persona responsable responde de su conducta y de sus consecuencias. Se podrá bloquear el acceso, conservar los registros pertinentes y colaborar con las autoridades competentes cuando la ley lo exija o permita.

Este proyecto es educativo y no proporciona asesoramiento financiero, de inversión, jurídico ni fiscal. Nada de este aviso excluye responsabilidades que legalmente no puedan excluirse. Obtén una revisión jurídica profesional antes de un despliegue comercial.

### Marcas y atribución

Numerai y sus nombres, marcas, contenidos multimedia y recursos enlazados pertenecen a sus respectivos propietarios. Faro Neural es una demostración independiente de interfaz. Los enlaces externos oficiales dirigen a destinos controlados por Numerai.

El fondo de la página principal sigue la obra referenciada por Numerai: Trevor Paglen, *CLOUD #865 Hough Circle Transform* (2019). La imagen se conserva localmente para mantener la fidelidad de la interfaz; la titularidad y todos los derechos aplicables pertenecen a sus respectivos titulares.

---

## License / Licencia

No open-source license has been granted yet. All rights are reserved unless a separate license file states otherwise.

Todavía no se ha concedido una licencia de código abierto. Todos los derechos están reservados, salvo que un archivo de licencia independiente indique lo contrario.
