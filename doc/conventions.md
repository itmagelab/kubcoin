# Соглашения по разработке лендинга KubCoin

> Правила кодирования для проекта лендинг-сайта на Yew + WebAssembly.
> Основано на [техническом видении](vision.md).

## Обязательные правила

### Безопасность кода

- **РЕКОМЕНДУЕТСЯ** избегать `unwrap()` и `expect()` где возможно
- **ОБЯЗАТЕЛЬНО** обрабатывать ошибки через `Result<T, E>`
- **ОБЯЗАТЕЛЬНО** использовать `?` оператор для пробрасывания ошибок
- **ОБЯЗАТЕЛЬНО** логировать ошибки через `tracing` макросы

### Архитектурные ограничения

- **ОБЯЗАТЕЛЬНО** использовать компонентную архитектуру Yew
- **ЗАПРЕЩЕНО** хардкод контента — только через `content.yaml`
- **ОБЯЗАТЕЛЬНО** следовать существующей структуре компонентов
- **ЗАПРЕЩЕНО** вводить новые архитектурные паттерны без согласования
- **ОБЯЗАТЕЛЬНО** отделять UI от данных (separation of concerns)

### Типобезопасность

- **ОБЯЗАТЕЛЬНО** использовать строгую типизацию для Props
- **ОБЯЗАТЕЛЬНО** использовать `#[derive(Debug, Deserialize)]` для структур данных
- **ЗАПРЕЩЕНО** использовать `Any` типы без веских причин
- **ОБЯЗАТЕЛЬНО** валидация данных при парсинге YAML

## Модульная структура

### Размещение кода

- `src/main.rs` — только точка входа и App компонент
- `src/html.rs` — UI компоненты (Header, Body, Footer, etc.)
- `src/html/` — подмодули для переиспользуемых элементов (button.rs)
- `src/handler.rs` — загрузка и парсинг контента
- `static/` — статические ресурсы (изображения, YAML, SEO файлы)

### Зависимости между модулями

- **ЗАПРЕЩЕНО** циклические зависимости между модулями
- **ОБЯЗАТЕЛЬНО** `handler` должен быть независимым от UI компонентов
- **ЗАПРЕЩЕНО** импорт UI компонентов в handler

## Именование

### Файлы и модули

- snake_case для файлов и модулей: `button.rs`, `handler.rs`
- Один основной компонент на файл для сложных UI
- Группировка простых компонентов допустима (например, все кнопки в `button.rs`)
- **ЗАПРЕЩЕНО** использование `mod.rs` (использовать именованные файлы)
- **ОБЯЗАТЕЛЬНО** использовать именованные модули:
  - Папка `html/` для UI подмодулей
  - Файл `html.rs` для объявления подмодулей и основных компонентов
  - В `main.rs`: `mod html;`

### Компоненты

- PascalCase для компонентов: `Header`, `Footer`, `QA`, `ChatItem`
- Функции компонентов в snake_case: `header()`, `footer()`, `qa()`
- Атрибут `#[function_component(Name)]` для всех компонентов Yew
- Явный return type `-> Html` для всех компонентов

### Переменные и функции

- snake_case для функций и переменных
- Глаголы для функций: `get_content()`, `parse_yaml()`
- Существительные для переменных: `content`, `open_index`, `items`
- Callbacks с префиксом `on_`: `on_click`, `on_change`

### Типы и структуры

- PascalCase для структур и энумов: `Content`, `ChatItem`, `Dialog`
- Описательные имена: `QAItem` вместо `Item`
- Суффикс для Props: `HeaderProps`, `ButtonProps`

## Компоненты Yew

### Структура функционального компонента

```rust
use yew::prelude::*;

#[function_component(ComponentName)]
pub(crate) fn component_name() -> Html {
    // 1. State hooks
    let state = use_state(|| initial_value);
    
    // 2. Effect hooks (если нужно)
    use_effect_with(deps, move |_| {
        // side effects
        || ()
    });
    
    // 3. Callbacks
    let on_click = {
        let state = state.clone();
        Callback::from(move |_| {
            state.set(new_value);
        })
    };
    
    // 4. Render
    html! {
        <div class="component">
            { "Content" }
        </div>
    }
}
```

### Props для компонентов

```rust
#[derive(Properties, PartialEq)]
pub struct MyProps {
    pub title: String,
    #[prop_or_default]
    pub subtitle: Option<String>,
}

#[function_component(MyComponent)]
pub fn my_component(props: &MyProps) -> Html {
    html! {
        <div>
            <h1>{ &props.title }</h1>
            if let Some(sub) = &props.subtitle {
                <h2>{ sub }</h2>
            }
        </div>
    }
}
```

### State management

- **ИСПОЛЬЗОВАТЬ** `use_state` для локального состояния
- **КЛОНИРОВАТЬ** state перед использованием в callbacks
- **НЕ ИСПОЛЬЗОВАТЬ** глобальное состояние без необходимости
- **ПЕРЕДАВАТЬ** данные через Props где возможно

```rust
// ✅ Правильно
let counter = use_state(|| 0);
let on_click = {
    let counter = counter.clone();
    Callback::from(move |_| {
        counter.set(*counter + 1);
    })
};

// ❌ Неправильно (без клонирования)
let on_click = Callback::from(move |_| {
    counter.set(*counter + 1); // Error: use of moved value
});
```

## Обработка ошибок

### Типы ошибок

- **ИСПОЛЬЗОВАТЬ** `Result<T, E>` для функций, которые могут упасть
- **ИСПОЛЬЗОВАТЬ** `Option<T>` для необязательных значений
- **ОБЯЗАТЕЛЬНО** логировать ошибки через `tracing::error!`
- **ПОКАЗЫВАТЬ** пользователю понятные сообщения об ошибках

### Примеры обработки ошибок

```rust
// ✅ Правильно — обработка ошибки парсинга
pub(crate) fn get_content() -> Content {
    match serde_yaml::from_str(CONTENT) {
        Ok(content) => content,
        Err(e) => {
            tracing::error!("Failed to parse content.yaml: {}", e);
            // Вернуть пустую структуру или дефолтные значения
            Content {
                chats: vec![],
                qa: vec![],
            }
        }
    }
}

// ✅ Правильно — логирование в компоненте
let on_click = Callback::from(move |_| {
    match perform_action() {
        Ok(result) => {
            tracing::info!("Action completed: {:?}", result);
        }
        Err(e) => {
            tracing::error!("Action failed: {}", e);
            // Показать пользователю
        }
    }
});

// ❌ Неправильно — игнорирование ошибок
let content = serde_yaml::from_str(CONTENT).unwrap();
```

### Логирование

- `tracing::error!` для ошибок (красный в console)
- `tracing::warn!` для предупреждений (желтый)
- `tracing::info!` для информационных сообщений
- `tracing::debug!` для отладочной информации
- `tracing::trace!` для детальной трассировки (обычно выключено)

## HTML и стилизация

### HTML структура

```rust
// ✅ Правильно — семантический HTML
html! {
    <section class="hero">
        <div class="hero-body">
            <div class="container">
                <h1 class="title">{ "Title" }</h1>
                <p class="subtitle">{ "Subtitle" }</p>
            </div>
        </div>
    </section>
}

// ❌ Неправильно — div-суп
html! {
    <div class="hero">
        <div class="body">
            <div class="wrapper">
                <div class="title">{ "Title" }</div>
                <div class="subtitle">{ "Subtitle" }</div>
            </div>
        </div>
    </div>
}
```

### CSS классы

- **ИСПОЛЬЗОВАТЬ** Bulma утилиты где возможно
- **ИЗБЕГАТЬ** кастомных классов без необходимости
- **ГРУППИРОВАТЬ** классы логически
- **НЕ ИСПОЛЬЗОВАТЬ** inline стили (кроме динамических значений)

```rust
// ✅ Правильно — Bulma классы
html! {
    <div class="box has-text-centered">
        <button class="button is-primary is-large">
            { "Click me" }
        </button>
    </div>
}

// ✅ Правильно — динамический inline стиль
let is_open = *open_index == Some(idx);
html! {
    <div style={if is_open { "display: block;" } else { "display: none;" }}>
        { "Content" }
    </div>
}

// ❌ Неправильно — хардкод стилей
html! {
    <div style="margin: 20px; padding: 15px; border: 1px solid #ccc;">
        { "Content" }
    </div>
}
```

### Условный рендеринг

```rust
// ✅ Правильно — использование if
html! {
    <div>
        if show_element {
            <p>{ "Visible" }</p>
        }
    </div>
}

// ✅ Правильно — Option
html! {
    <div>
        if let Some(text) = &optional_text {
            <p>{ text }</p>
        }
    </div>
}

// ✅ Правильно — итерация
html! {
    <ul>
        { for items.iter().map(|item| html! {
            <li>{ &item.name }</li>
        }) }
    </ul>
}
```

## Управление контентом

### Структура данных

- **ВСЕ** текстовое содержимое в `static/content.yaml`
- **ЗАПРЕЩЕНО** хардкодить текст в компонентах
- **ОБЯЗАТЕЛЬНО** использовать структурированные данные

```rust
// ✅ Правильно — контент из YAML
let content = get_content();
html! {
    <h1>{ &content.title }</h1>
}

// ❌ Неправильно — хардкод
html! {
    <h1>{ "KubCoin - Финансовый бот" }</h1>
}
```

### YAML структура

```yaml
# ✅ Правильно — структурированные данные
qa:
  - question: "Вопрос?"
    answer: "Ответ."

chats:
  - title: "Заголовок"
    subtitle: "Подзаголовок"
    dialogs:
      - req: "Запрос"
        res: "Ответ"

# ❌ Неправильно — плоская структура
questions:
  - "Вопрос 1?"
  - "Вопрос 2?"
answers:
  - "Ответ 1"
  - "Ответ 2"
```

## Производительность

### Оптимизация рендеринга

- **ИСПОЛЬЗОВАТЬ** `PartialEq` для Props чтобы избежать лишних re-renders
- **ИЗБЕГАТЬ** создания новых callbacks на каждый рендер
- **МЕМОИЗИРОВАТЬ** тяжелые вычисления через `use_memo`

```rust
// ✅ Правильно — callback создается один раз
let on_click = {
    let state = state.clone();
    Callback::from(move |_| {
        state.set(new_value);
    })
};

// ❌ Неправильно — callback создается на каждый рендер
html! {
    <button onclick={Callback::from(|_| { /* ... */ })}>
        { "Click" }
    </button>
}
```

### Загрузка ресурсов

- **ИСПОЛЬЗОВАТЬ** WebP для изображений
- **ОПТИМИЗИРОВАТЬ** размеры изображений
- **ДОБАВЛЯТЬ** `loading="lazy"` для изображений вне viewport
- **МИНИМИЗИРОВАТЬ** размер WASM бандла

## Accessibility

### ARIA и семантика

- **ИСПОЛЬЗОВАТЬ** семантические HTML теги
- **ДОБАВЛЯТЬ** `alt` текст для изображений
- **ИСПОЛЬЗОВАТЬ** ARIA labels где нужно
- **ОБЕСПЕЧИТЬ** keyboard navigation

```rust
// ✅ Правильно — accessibility
html! {
    <button 
        class="button"
        aria-label="Открыть меню"
        onclick={on_click}>
        { "☰" }
    </button>
    <img 
        src="image.jpg" 
        alt="Скриншот приложения KubCoin" />
}

// ❌ Неправильно — плохой accessibility
html! {
    <div onclick={on_click}>{ "☰" }</div>
    <img src="image.jpg" />
}
```

## SEO

### Meta-теги

- **ОБЯЗАТЕЛЬНО** title, description, keywords в `index.html`
- **ОБЯЗАТЕЛЬНО** Open Graph теги для социальных сетей
- **ДОБАВЛЯТЬ** structured data (JSON-LD) где применимо
- **ИСПОЛЬЗОВАТЬ** семантический HTML для поисковых систем

### Robots и sitemap

- **ОБЯЗАТЕЛЬНО** `robots.txt` в static/
- **СОЗДАТЬ** `sitemap.xml` при добавлении новых страниц
- **НЕ БЛОКИРОВАТЬ** поисковых ботов

## Тестирование

### Unit тесты

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_content_loading() {
        let content = get_content();
        assert!(!content.chats.is_empty());
        assert!(!content.qa.is_empty());
    }

    #[test]
    fn test_data_structure() {
        let content = get_content();
        for chat in &content.chats {
            assert!(!chat.title.is_empty());
            assert!(!chat.dialogs.is_empty());
        }
    }
}
```

### Ручное тестирование

Перед каждым релизом проверить:

- [ ] Все ссылки работают
- [ ] Все изображения загружаются
- [ ] FAQ аккордеон открывается/закрывается
- [ ] Кнопки ведут на правильные URL
- [ ] Мобильная версия корректна
- [ ] Нет ошибок в console
- [ ] Lighthouse score > 90

## Git workflow

### Commit messages

**ОБЯЗАТЕЛЬНО:** Все commit messages **только на английском языке**

```bash
# ✅ Правильно — conventional commits на английском
git commit -m "feat: add new testimonials section"
git commit -m "fix: resolve image loading issue"
git commit -m "docs: update README"
git commit -m "style: improve FAQ spacing"
git commit -m "refactor: extract button components to separate file"
git commit -m "perf: optimize WASM bundle size"

# ❌ Неправильно — русский язык
git commit -m "feat: добавить новую секцию"
git commit -m "fix: исправить баг"

# ❌ Неправильно — неинформативные сообщения
git commit -m "updates"
git commit -m "fix bug"
git commit -m "wip"
```

**Формат:** `<type>: <description>`

**Типы коммитов:**
- `feat:` — новая функциональность
- `fix:` — исправление бага
- `docs:` — изменения в документации
- `style:` — форматирование, отступы (не влияет на код)
- `refactor:` — рефакторинг кода
- `perf:` — улучшение производительности
- `test:` — добавление тестов
- `chore:` — обновление зависимостей, конфигурации
```

### Branches

- `main` — production код
- `feature/*` — новые фичи
- `fix/*` — исправления багов
- `docs/*` — обновление документации

## Инструменты разработки

### Обязательные проверки перед commit

```bash
# Форматирование
cargo fmt

# Линтинг
cargo clippy --target wasm32-unknown-unknown

# Компиляция
cargo check --target wasm32-unknown-unknown
```

### CI/CD чеклист

- [ ] `cargo fmt -- --check` проходит
- [ ] `cargo clippy` без warnings
- [ ] `cargo test` все тесты проходят
- [ ] `trunk build --release` успешен
- [ ] WASM bundle < 500KB
- [ ] Lighthouse CI > 90

## Документация

### Код

- **ОБЯЗАТЕЛЬНО** doc-комментарии для публичных функций и типов
- **ДОБАВЛЯТЬ** примеры использования в doc-комментариях
- **ОПИСЫВАТЬ** сложную логику inline комментариями

```rust
/// Загружает и парсит контент из YAML файла.
///
/// # Returns
///
/// Структуру `Content` с данными для отображения на сайте.
///
/// # Panics
///
/// Паникует если YAML файл не может быть распарсен.
/// В production использовать graceful error handling.
pub(crate) fn get_content() -> Content {
    serde_yaml::from_str(CONTENT).unwrap()
}
```

### README и документация

- **ОБНОВЛЯТЬ** README при изменении процесса разработки
- **ДОКУМЕНТИРОВАТЬ** архитектурные решения в `doc/`
- **ПОДДЕРЖИВАТЬ** актуальность `tasklist.md`

## Безопасность

### Внешние ссылки

```rust
// ✅ Правильно — безопасные ссылки
html! {
    <a href="https://t.me/kubcoin_bot"
       target="_blank"
       rel="noopener noreferrer">
        { "Открыть бота" }
    </a>
}

// ❌ Неправильно — небезопасно
html! {
    <a href="https://example.com" target="_blank">
        { "Link" }
    </a>
}
```

### XSS защита

- Yew автоматически экранирует текст в шаблонах
- **НЕ ИСПОЛЬЗОВАТЬ** небезопасные конструкции без необходимости
- **ВАЛИДИРОВАТЬ** пользовательский ввод (если будет)

## Основные команды

### Разработка

```bash
# Запуск dev сервера
trunk serve --open

# Dev сервер с custom портом
trunk serve --port 3000

# Проверка компиляции
cargo check --target wasm32-unknown-unknown

# Линтинг
cargo clippy --target wasm32-unknown-unknown

# Форматирование
cargo fmt
```

### Production

```bash
# Сборка для production
trunk build --release

# Проверка размера бандла
ls -lh dist/*.wasm

# Запуск локально собранного production
cd dist && python -m http.server 8000
```

### Тестирование

```bash
# Unit тесты
cargo test

# Lighthouse audit
lighthouse http://localhost:8080 --view

# Проверка accessibility
# Открыть в браузере, F12 -> Lighthouse -> Accessibility
```

## Контрольный список перед релизом

- [ ] Все задачи из итерации выполнены
- [ ] `cargo fmt` применен
- [ ] `cargo clippy` без warnings
- [ ] Все тесты проходят
- [ ] Production сборка успешна
- [ ] WASM бандл < 500KB
- [ ] Lighthouse score > 90 (все метрики)
- [ ] Проверка на мобильных устройствах
- [ ] Проверка в Chrome, Firefox, Safari
- [ ] Все ссылки работают
- [ ] Нет console errors
- [ ] `content.yaml` валиден
- [ ] Meta-теги актуальны
- [ ] CHANGELOG обновлен
- [ ] Git tag создан
- [ ] Деплой на production
- [ ] Проверка работы на production

## Полезные ссылки

- [Yew Documentation](https://yew.rs/)
- [Yew Examples](https://github.com/yewstack/yew/tree/master/examples)
- [Trunk Documentation](https://trunkrs.dev/)
- [Bulma CSS](https://bulma.io/documentation/)
- [Rust WASM Book](https://rustwasm.github.io/docs/book/)
- [MDN WebAssembly](https://developer.mozilla.org/en-US/docs/WebAssembly)