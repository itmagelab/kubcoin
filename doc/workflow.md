# Процесс разработки лендинг-сайта KubCoin

> Инструкция для разработчиков по работе с проектом лендинга на Yew + WebAssembly

## Основные принципы

### KISS подход
- Простые решения без излишней сложности
- Минимум абстракций, максимум ясности
- Следование существующим паттернам Yew

### Component-driven development
- Разработка переиспользуемых компонентов
- Разделение UI и бизнес-логики
- Композиция компонентов вместо наследования

### Content-first подход
- Контент отделен от кода (YAML файлы)
- Легкое обновление без перекомпиляции
- Структурированные данные для будущей i18n

## Окружение разработки

### Установка инструментов

```bash
# Установить Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Добавить WASM target
rustup target add wasm32-unknown-unknown

# Установить Trunk (build tool)
cargo install trunk

# Установить wasm-bindgen-cli (опционально, для отладки)
cargo install wasm-bindgen-cli
```

### Клонирование и настройка

```bash
# Клонировать репозиторий
git clone https://github.com/your-org/kubcoin.git
cd kubcoin

# Проверить зависимости
cargo check --target wasm32-unknown-unknown

# Запустить dev сервер
trunk serve --open
```

## Структура проекта

```
kubcoin/
├── src/
│   ├── main.rs           # Точка входа, App компонент
│   ├── html.rs           # UI компоненты
│   ├── html/
│   │   └── button.rs     # Кнопки и переиспользуемые элементы
│   └── handler.rs        # Загрузка контента
├── static/
│   ├── content.yaml      # Контент сайта (диалоги, FAQ)
│   ├── images/           # Изображения
│   └── robots.txt        # SEO файлы
├── index.html            # HTML шаблон
├── Cargo.toml            # Зависимости
└── doc/                  # Документация
    ├── idea.md
    ├── vision.md
    ├── tasklist.md
    └── workflow.md       # Этот файл
```

## Рабочий процесс

### 1. Локальная разработка

```bash
# Запуск dev сервера с hot reload
trunk serve --open

# Dev сервер будет доступен на http://127.0.0.1:8080
# Hot reload автоматически перезагружает страницу при изменениях
```

**Особенности dev режима:**
- WASM собирается в debug режиме (быстрее, но больше размер)
- Исходные карты включены для отладки
- Логи трассировки в browser console
- Автоматическая перезагрузка при изменениях в src/

### 2. Работа с компонентами

#### Создание нового компонента

```rust
// src/html.rs или отдельный файл

use yew::prelude::*;

#[function_component(MyComponent)]
pub(crate) fn my_component() -> Html {
    // State hooks
    let counter = use_state(|| 0);
    
    // Callbacks
    let on_click = {
        let counter = counter.clone();
        Callback::from(move |_| {
            counter.set(*counter + 1);
        })
    };
    
    // Render
    html! {
        <div class="box">
            <p>{ format!("Count: {}", *counter) }</p>
            <button class="button" onclick={on_click}>
                { "Increment" }
            </button>
        </div>
    }
}
```

#### Использование компонента

```rust
// В другом компоненте
html! {
    <>
        <Header />
        <MyComponent />
        <Footer />
    </>
}
```

### 3. Обновление контента

Контент обновляется без изменения кода:

```yaml
# static/content.yaml

qa:
  - question: "Новый вопрос?"
    answer: "Новый ответ!"

chats:
  - title: "Новый пример"
    subtitle: "Описание"
    dialogs:
      - req: "Запрос пользователя"
        res: "Ответ бота"
```

После изменения `content.yaml` — просто обновите страницу в браузере.

### 4. Работа со стилями

Используем Bulma CSS фреймворк:

```rust
html! {
    <section class="hero is-primary">
        <div class="hero-body">
            <div class="container">
                <h1 class="title">{ "Hello World" }</h1>
                <h2 class="subtitle">{ "My subtitle" }</h2>
            </div>
        </div>
    </section>
}
```

Кастомные стили добавляются в `<style>` блок в `index.html`:

```html
<style>
.my-custom-class {
    border-radius: 20px;
    box-shadow: 0 2px 8px rgba(0,0,0,0.1);
}
</style>
```

### 5. Отладка

#### Browser Console

```rust
// Логирование в console
tracing::info!("User clicked button");
tracing::error!("Failed to load data: {}", error);
tracing::debug!("State: {:?}", state);
```

#### DevTools

- **Console** → tracing логи и ошибки
- **Network** → загрузка WASM и ресурсов
- **Performance** → профилирование
- **Lighthouse** → аудит производительности и SEO

#### WASM отладка

```bash
# Инспекция WASM модуля
wasm-bindgen-cli inspect dist/*.wasm
```

### 6. Тестирование

#### Проверка компиляции

```bash
# Проверить без сборки WASM
cargo check

# Проверить для WASM target
cargo check --target wasm32-unknown-unknown
```

#### Линтинг и форматирование

```bash
# Форматирование кода
cargo fmt

# Проверка стиля кода
cargo fmt -- --check

# Линтинг
cargo clippy --target wasm32-unknown-unknown

# Исправление автоматических предупреждений
cargo clippy --target wasm32-unknown-unknown --fix
```

#### Unit тесты

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_content_parsing() {
        let content = get_content();
        assert!(!content.chats.is_empty());
        assert!(!content.qa.is_empty());
    }
}
```

```bash
# Запуск тестов
cargo test
```

#### E2E тестирование (опционально)

Для более сложных сценариев можно использовать:
- **wasm-pack test** — для WASM-специфичных тестов
- **Playwright/Selenium** — для браузерных тестов

### 7. Production сборка

```bash
# Сборка для production
trunk build --release

# Результат в dist/ директории
ls -lah dist/
```

**Production оптимизации:**
- WASM оптимизирован и минимизирован
- Dead code elimination
- Compression (gzip/brotli готов)
- Минимизация JS glue code

### 8. Деплой

#### Статический хостинг (Cloudflare Pages, Netlify, etc.)

```bash
# Просто загрузить содержимое dist/
trunk build --release
# Upload dist/ folder to hosting
```

#### GitHub Pages

```yaml
# .github/workflows/deploy.yml
name: Deploy to GitHub Pages

on:
  push:
    branches: [main]

jobs:
  deploy:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      
      - name: Setup Rust
        uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
          target: wasm32-unknown-unknown
      
      - name: Install Trunk
        run: cargo install trunk
      
      - name: Build
        run: trunk build --release
      
      - name: Deploy
        uses: peaceiris/actions-gh-pages@v3
        with:
          github_token: ${{ secrets.GITHUB_TOKEN }}
          publish_dir: ./dist
```

#### Nginx/Caddy

```nginx
# nginx.conf
server {
    listen 80;
    server_name kubcoin.ru;
    
    root /var/www/kubcoin/dist;
    index index.html;
    
    # Сжатие для WASM
    gzip on;
    gzip_types application/wasm;
    
    location / {
        try_files $uri $uri/ /index.html;
    }
    
    # Кеширование статики
    location ~* \.(wasm|js|css|png|jpg|jpeg|gif|ico|svg)$ {
        expires 1y;
        add_header Cache-Control "public, immutable";
    }
}
```

## Алгоритм работы с задачами

### 1. Выбор задачи

- [ ] Открыть [tasklist.md](tasklist.md)
- [ ] Найти текущую итерацию (🟡 В работе)
- [ ] Выбрать незавершенную задачу (🔲)
- [ ] Изучить требования и контекст

### 2. Планирование

- [ ] Определить, какие компоненты нужно изменить/создать
- [ ] Продумать структуру данных
- [ ] Оценить влияние на другие компоненты
- [ ] Согласовать подход (если требуется)

### 3. Реализация

```bash
# Создать feature branch
git checkout -b feature/task-name

# Запустить dev сервер
trunk serve

# Внести изменения в код
# Проверить в браузере
# Добавить/обновить контент в content.yaml
```

### 4. Проверка качества

```bash
# Форматирование
cargo fmt

# Линтинг
cargo clippy --target wasm32-unknown-unknown

# Production сборка
trunk build --release

# Проверить размер бандла
ls -lh dist/*.wasm
```

### 5. Тестирование

- [ ] Проверить на разных разрешениях экрана
- [ ] Протестировать на мобильных (iOS, Android)
- [ ] Проверить в разных браузерах (Chrome, Firefox, Safari)
- [ ] Запустить Lighthouse audit (Performance, SEO, Accessibility)
- [ ] Проверить загрузку на медленном соединении

### 6. Commit и PR

```bash
# Commit изменений
git add .
git commit -m "feat: добавить новую секцию с примерами"

# Push в remote
git push origin feature/task-name

# Создать Pull Request
# Обновить tasklist.md (отметить задачу как выполненную)
```

### 7. Деплой

После мерджа в main:

```bash
# Автоматический деплой через CI/CD
# ИЛИ
# Ручной деплой
trunk build --release
# Upload dist/ to hosting
```

## Частые задачи

### Добавить новый пример диалога

1. Открыть `static/content.yaml`
2. Добавить в массив `chats`:

```yaml
chats:
  - title: "Заголовок примера"
    subtitle: "Описание"
    dialogs:
      - req: "Пользователь пишет"
        res: "Бот отвечает"
```

3. Обновить страницу — изменения видны сразу

### Добавить новый вопрос в FAQ

1. Открыть `static/content.yaml`
2. Добавить в массив `qa`:

```yaml
qa:
  - question: "Новый вопрос?"
    answer: "Подробный ответ на вопрос."
```

3. Компонент QA автоматически отрендерит новый элемент

### Создать новую секцию

1. Создать компонент в `src/html.rs`:

```rust
#[function_component(NewSection)]
pub(crate) fn new_section() -> Html {
    html! {
        <section class="section">
            <div class="container">
                <h2 class="title">{ "Новая секция" }</h2>
                <p>{ "Контент секции" }</p>
            </div>
        </section>
    }
}
```

2. Добавить в Body компонент:

```rust
#[function_component(Body)]
pub(crate) fn body() -> Html {
    html! {
        <div class="container">
            <Chats />
            <Split />
            <NewSection />  // <-- Новая секция
            <QA />
            <Split />
            <Usage />
        </div>
    }
}
```

### Добавить новую кнопку

1. В `src/html/button.rs`:

```rust
#[function_component(NewButton)]
pub(crate) fn new_button() -> Html {
    link("Текст кнопки", "https://example.com", true)
}
```

2. Использовать в компоненте:

```rust
use crate::html::button::NewButton;

html! {
    <NewButton />
}
```

### Изменить стили

Для глобальных стилей — редактировать `<style>` в `index.html`:

```html
<style>
.my-class {
    /* новые стили */
}
</style>
```

Для компонентных стилей — использовать inline или Bulma классы:

```rust
html! {
    <div style="border-radius: 10px; padding: 1rem;">
        { "Content" }
    </div>
}
```

## Performance чеклист

### Перед релизом

- [ ] WASM бандл < 500KB (gzipped)
- [ ] First Contentful Paint < 1.5s
- [ ] Time to Interactive < 3.0s
- [ ] Lighthouse Performance > 90
- [ ] Lighthouse SEO > 90
- [ ] Lighthouse Accessibility > 90
- [ ] Все изображения оптимизированы (WebP, правильный размер)
- [ ] Отсутствуют console.error в production

### Оптимизация WASM

```bash
# Проверить размер
ls -lh dist/*.wasm

# Если больше 500KB:
# 1. Проверить зависимости (удалить неиспользуемые)
# 2. Использовать wasm-opt
cargo install wasm-opt
wasm-opt -Oz -o output.wasm input.wasm

# 3. Включить LTO в Cargo.toml
[profile.release]
lto = true
opt-level = "z"
codegen-units = 1
```

## Troubleshooting

### WASM не загружается

- Проверить MIME type на сервере: `application/wasm`
- Проверить CORS headers
- Открыть DevTools → Network → проверить статус загрузки

### Изменения не отображаются

- Hard refresh: Ctrl+Shift+R (Cmd+Shift+R на Mac)
- Очистить кеш браузера
- Проверить, что trunk serve запущен
- Проверить console на ошибки

### Ошибки компиляции

```bash
# Очистить и пересобрать
cargo clean
cargo check --target wasm32-unknown-unknown
trunk build
```

### Performance проблемы

- Проверить размер бандла
- Использовать lazy loading для изображений
- Минимизировать количество re-renders
- Оптимизировать use_state hooks

## Best Practices

### Rust код

- ✅ Использовать `#[function_component]` для всех компонентов
- ✅ Предпочитать `use_state` вместо `use_reducer` для простых случаев
- ✅ Клонировать state перед использованием в callbacks
- ✅ Использовать `pub(crate)` для внутренних компонентов
- ❌ Избегать `unwrap()` и `expect()` где возможно

### HTML/CSS

- ✅ Использовать семантические HTML теги
- ✅ Предпочитать Bulma классы кастомным стилям
- ✅ Mobile-first подход к responsive дизайну
- ✅ Accessibility: alt текст, ARIA labels
- ❌ Избегать inline стилей (только для динамических значений)

### Контент

- ✅ Весь текст в `content.yaml`
- ✅ Структурированные данные
- ✅ Готовность к i18n
- ❌ Не хардкодить текст в компонентах

## Полезные ссылки

- [Yew Documentation](https://yew.rs/)
- [Trunk Documentation](https://trunkrs.dev/)
- [Bulma Documentation](https://bulma.io/documentation/)
- [WebAssembly MDN](https://developer.mozilla.org/en-US/docs/WebAssembly)
- [Rust WASM Book](https://rustwasm.github.io/docs/book/)

## Контакты

По вопросам разработки:
- Email: i@itmage.ru
- Telegram: @iTmagelab
- Канал: @iTmagelab_ru