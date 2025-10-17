# Техническое видение проекта KubCoin Landing

> Основной документ для разработки лендинг-сайта Telegram-бота KubCoin

## Обзор проекта

KubCoin Landing — это **презентационный веб-сайт** для Telegram-бота по управлению личными финансами. Сайт написан на Rust с использованием фреймворка Yew и компилируется в WebAssembly для высокой производительности и безопасности.

## Технический стек

### Основные технологии
- **Язык**: Rust (edition 2024)
- **UI фреймворк**: Yew (Client-Side Rendering mode)
- **Компиляция**: WebAssembly (WASM)
- **Сборка**: Trunk — build tool для Rust WASM приложений
- **CSS фреймворк**: Bulma 1.0.4
- **Хостинг**: статический хостинг (совместим с любым CDN)

### Основные зависимости
- `yew` (git version) - React-подобный UI фреймворк для Rust
- `wasm-bindgen` - связывание Rust и JavaScript
- `web-sys` - Web API bindings
- `js-sys` - JavaScript стандартная библиотека bindings
- `serde` / `serde_yaml` - сериализация и парсинг контента
- `tracing` / `tracing-web` - логирование в browser console
- `tracing-subscriber` - управление подписчиками логов

### Архитектурные паттерны
- Component-based architecture (как в React)
- Separation of Concerns (UI, контент, обработчики)
- Data-driven content через YAML
- Статическая компиляция (SSG-подобный подход)

## Архитектурные решения

### Компонентная архитектура

Приложение построено из переиспользуемых компонентов:

- **App** — корневой компонент, собирает страницу
- **Header** — hero-секция с заголовком, описанием и CTA-кнопками
- **Body** — основной контент (Chats, QA, Usage)
- **Footer** — копирайт и контакты
- **Chats** — демонстрация диалогов с ботом
- **QA** — интерактивный FAQ с аккордеоном
- **Usage** — статистика использования бота
- **Button** — переиспользуемые кнопки (Start, Group, Channel)

### Разделение на модули

```
src/
├── main.rs           # Точка входа, инициализация приложения
├── html.rs           # UI компоненты (Header, Body, Footer, etc.)
├── html/
│   └── button.rs     # Компоненты кнопок
└── handler.rs        # Загрузка и парсинг контента
```

### Управление контентом

Весь контент вынесен в `static/content.yaml`:

```yaml
qa:                    # FAQ секция
  - question: "..."
    answer: "..."
    
chats:                 # Примеры диалогов
  - title: "..."
    subtitle: "..."
    dialogs:
      - req: "..."    # Запрос пользователя
        res: "..."    # Ответ бота
```

**Преимущества подхода:**
- Контент можно обновлять без перекомпиляции
- Легко добавлять новые примеры и вопросы
- Структурированные данные для будущей интернационализации
- Единый источник правды для всего текстового контента

### Статические ресурсы

```
static/
├── content.yaml              # Контент сайта
├── images/
│   ├── IMG_3089.JPG         # Скриншот бота
│   ├── favicon.ico          # Иконка сайта
│   └── og-image.png         # Open Graph изображение
├── robots.txt               # Для SEO
└── yandex_*.html            # Верификация Яндекс.Вебмастер
```

### Сборка и деплой

#### Trunk build system

Trunk компилирует Rust в WASM и собирает финальный HTML:

```html
<link data-trunk rel="copy-dir" href="static/images" />
<link data-trunk rel="copy-file" href="static/yandex_*.html" />
<link data-trunk rel="copy-file" href="static/robots.txt" />
```

#### Процесс сборки

1. `trunk build --release` — компиляция в production режиме
2. Генерация оптимизированного WASM
3. Копирование статических файлов
4. Минификация и оптимизация
5. Вывод в `dist/` директорию

#### Деплой

Готовая директория `dist/` может быть задеплоена на любой статический хостинг:
- Cloudflare Pages
- GitHub Pages
- Netlify
- Vercel
- AWS S3 + CloudFront
- Nginx/Caddy

## Принципы разработки

### KISS (Keep It Simple, Stupid)
- Простая компонентная структура без излишних абстракций
- Прямолинейная навигация и UX
- Минимум JavaScript — максимум производительности

### Performance First
- WebAssembly обеспечивает near-native производительность
- Ленивая загрузка изображений
- Минимальный размер бандла
- Оптимизация CSS (только используемые классы Bulma)

### SEO и доступность
- Семантический HTML
- Правильные meta-теги (title, description, keywords)
- Open Graph для социальных сетей
- Robots.txt для поисковых систем
- Альтернативный текст для изображений

### Mobile-First
- Responsive дизайн через Bulma columns
- Адаптивные изображения
- Touch-friendly интерфейс
- Viewport meta-тег для мобильных устройств

### Типобезопасность
- Rust типизация предотвращает ошибки на этапе компиляции
- Структурированные данные через Serde
- Явные типы для всех компонентов

## Модель данных

### Content структура

```rust
struct Content {
    chats: Vec<ChatItem>,
    qa: Vec<QAItem>,
}

struct ChatItem {
    title: String,
    subtitle: String,
    dialogs: Vec<Dialog>,
}

struct Dialog {
    req: String,  // Запрос пользователя
    res: String,  // Ответ бота
}

struct QAItem {
    question: String,
    answer: String,
}
```

### Component Props

Компоненты Yew используют Props для передачи данных:
- Иммутабельные по умолчанию
- Клонирование для передачи между компонентами
- Reactive updates через state hooks

## Интерактивность

### State Management

Yew предоставляет hooks для управления состоянием:

```rust
let open_index = use_state(|| None);  // Какой FAQ элемент открыт

let on_click = {
    let open_index = open_index.clone();
    Callback::from(move |_| {
        if is_open {
            open_index.set(None);  // Закрыть
        } else {
            open_index.set(Some(idx));  // Открыть
        }
    })
};
```

### Анимации и эффекты

CSS анимации для плавных переходов:
- Fade-in для FAQ ответов
- Phone screenshot с градиентным затемнением по краям
- Hover эффекты для кнопок (через Bulma)

## Аналитика и мониторинг

### Yandex.Metrika

Интеграция счетчика (ID: 104580346):
- Webvisor для записи сессий
- Click map для анализа кликов
- Accurate track bounce для точного показателя отказов
- Track links для отслеживания внешних ссылок
- E-commerce data layer (зарезервировано)

### Трейсинг в браузере

```rust
let fmt_layer = tracing_subscriber::fmt::layer()
    .with_ansi(false)
    .with_writer(tracing_web::MakeConsoleWriter)
    .without_time()
    .with_span_events(FmtSpan::ACTIVE);

let perf_layer = tracing_web::performance_layer()
    .with_details_from_fields(Pretty::default());
```

Все логи и performance metrics отображаются в browser console.

## Сценарии использования

### Первое посещение

1. Пользователь открывает https://kubcoin.ru
2. Загружается WASM модуль (кеширование в браузере)
3. Рендерится страница с Hero секцией
4. Пользователь видит CTA кнопки
5. Прокрутка до примеров диалогов
6. Клик на "Начать" → переход в @kubcoin_bot

### Изучение возможностей

1. Просмотр интерактивных примеров в Chat секции
2. Чтение FAQ с раскрытием ответов
3. Просмотр статистики использования
4. Переход в группу или канал

### Mobile Experience

1. Адаптивная верстка для всех экранов
2. Touch-friendly элементы управления
3. Оптимизированные изображения для мобильных
4. Быстрая загрузка благодаря WASM

## Технические ограничения

### WebAssembly

- **Размер бандла**: ~500KB (WASM + JS glue code)
- **Поддержка браузеров**: все современные (Chrome 57+, Firefox 52+, Safari 11+)
- **SSR**: не поддерживается (только CSR)
- **SEO**: требует pre-rendering или использование meta-тегов

### Yew CSR mode

- Первоначальный рендер происходит на клиенте
- Требуется загрузка WASM перед отображением контента
- Не подходит для SEO-критичных многостраничных сайтов
- Идеально для SPA и landing pages

## Безопасность

### Rust гарантии

- Memory safety без garbage collector
- Отсутствие null pointer exceptions
- Thread safety (в будущем для Web Workers)
- Строгая типизация предотвращает XSS через шаблоны

### Best Practices

- Все внешние ссылки с `rel="noopener noreferrer"`
- HTTPS-only (через meta-теги и хостинг)
- Content Security Policy (настраивается на хостинге)
- Минимизация зависимостей для уменьшения attack surface

## Развитие проекта

### Текущая версия (v0.1.0)

- ✅ Статический лендинг с основными секциями
- ✅ Интерактивные примеры диалогов
- ✅ FAQ с аккордеоном
- ✅ Интеграция с Yandex.Metrika
- ✅ SEO оптимизация
- ✅ Responsive дизайн

### Краткосрочные планы

- [ ] Добавить больше примеров диалогов из `content.yaml`
- [ ] Секция с отзывами пользователей
- [ ] Анимированная демонстрация функций бота
- [ ] Улучшение performance metrics

### Долгосрочные планы

- [ ] Интернационализация (English, etc.)
- [ ] Документация API для разработчиков
- [ ] Блог на отдельном поддомене
- [ ] Web Workers для фоновых задач
- [ ] Интеграция с backend для real-time статистики

## Инструменты разработки

### Локальная разработка

```bash
# Установка Trunk
cargo install trunk

# Установка wasm target
rustup target add wasm32-unknown-unknown

# Запуск dev сервера с hot reload
trunk serve --open

# Production сборка
trunk build --release
```

### Отладка

- Browser DevTools → Console для tracing логов
- Browser DevTools → Network для анализа загрузки WASM
- Browser DevTools → Performance для профилирования
- `RUST_LOG=debug` для детального логирования

### Линтинг и форматирование

```bash
# Проверка кода
cargo check --target wasm32-unknown-unknown

# Форматирование
cargo fmt

# Линтинг
cargo clippy --target wasm32-unknown-unknown
```

## Соглашения по коду

### Именование компонентов

- PascalCase для функциональных компонентов: `Header`, `Footer`, `QA`
- snake_case для функций-хелперов: `get_content()`, `chat()`
- Атрибут `#[function_component(Name)]` для Yew компонентов

### Структура компонентов

```rust
#[function_component(ComponentName)]
pub(crate) fn component_name() -> Html {
    // Hooks (use_state, use_effect, etc.)
    let state = use_state(|| initial_value);
    
    // Callbacks
    let on_click = Callback::from(move |_| { /* ... */ });
    
    // Render
    html! {
        <div>{ "content" }</div>
    }
}
```

### CSS классы

- Использовать Bulma утилиты где возможно
- Кастомные стили через inline `<style>` в `index.html`
- Семантические классы: `.hero`, `.section`, `.container`, `.box`

## Метрики успеха

### Performance

- **First Contentful Paint**: < 1.5s
- **Time to Interactive**: < 3.0s
- **WASM bundle size**: < 500KB (gzipped)
- **Lighthouse score**: > 90

### Engagement

- **Bounce rate**: < 50%
- **Average session duration**: > 2 минут
- **Click-through rate** (на бота): > 20%
- **Return visitors**: > 30%

### Technical

- **Build time**: < 30s
- **Zero runtime errors** в production
- **Cross-browser compatibility**: 99%
- **Mobile responsiveness**: 100%