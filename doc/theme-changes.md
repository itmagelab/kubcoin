# Изменение цветовой схемы на зелёную

## Дата изменения
2025-11-01

## Описание
Цветовая схема лендинга KubCoin изменена с синей на зелёную в соответствии с conventions.md.

## Изменённые файлы

### 1. `index.html`
**Tailwind конфигурация:**
- `primary`: `#3b82f6` → `#10b981` (синий → зелёный)
- `secondary`: `#1e40af` → `#059669` (тёмно-синий → тёмно-зелёный)
- `accent`: `#10b981` → `#34d399` (остался зелёный, но светлее)
- `light`: `#f8fafc` → `#f0fdf4` (серо-голубой → светло-зелёный)

**CSS стили:**
- `.faq-answer` border: `#3b82f6` → `#10b981`
- `.faq-answer` background: `#f8fafc` → `#f0fdf4`
- Focus indicators: `#3b82f6` → `#10b981`
- Focus shadow: `rgba(59, 130, 246, 0.25)` → `rgba(16, 185, 129, 0.25)`
- `.skip-to-main` background: `#3b82f6` → `#10b981`
- `.bg-light`: `#f8fafc` → `#f0fdf4`
- `.border-accent`: `#10b981` → `#34d399`
- `.border-primary`: `#3b82f6` → `#10b981`

### 2. `src/html/button.rs`
**Кнопки:**
- Primary button: `bg-blue-600 hover:bg-blue-700` → `bg-green-600 hover:bg-green-700`
- Focus ring: `focus:ring-blue-500` → `focus:ring-green-500`

Изменения применены к функциям:
- `link()`
- `link_medium()`

### 3. `src/html/language_switcher.rs`
**Переключатель языков:**
- Active button: `bg-blue-600 hover:bg-blue-700` → `bg-green-600 hover:bg-green-700`
- Focus ring: `focus:ring-blue-500` → `focus:ring-green-500`

### 4. `src/html.rs`
**Security раздел:**
- Info box background: `bg-blue-50` → `bg-green-50`
- Info box border: `border-blue-200` → `border-green-200`
- Info box text: `text-blue-800` → `text-green-800`

**Pricing раздел:**
- Premium card border: `border-blue-500` → `border-green-500`
- Premium badge: `bg-blue-500` → `bg-green-500`
- Price color: `text-blue-600` → `text-green-600`
- Checkmarks: `text-blue-500` → `text-green-500` (все 9 галочек)
- CTA button: `bg-blue-500` → `bg-green-500`
- Enterprise link: `text-blue-600 hover:text-blue-800` → `text-green-600 hover:text-green-800`

**FAQ раздел:**
- Active item ring: `ring-blue-500` → `ring-green-500`

**Chat раздел:**
- User message bubble: `bg-blue-500` → `bg-green-500`

### 5. `src/main.rs`
**Фоновый градиент:**
- Background: `from-blue-50 to-indigo-100` → `from-green-50 to-emerald-100`

## Цветовая палитра

### Основные цвета (зелёная тема)
```css
--primary: #10b981;      /* Основной зелёный */
--secondary: #059669;    /* Тёмно-зелёный */
--accent: #34d399;       /* Светло-зелёный (акцент) */
--dark: #1f2937;         /* Тёмно-серый (текст) */
--light: #f0fdf4;        /* Светло-зелёный (фон) */
```

### Tailwind классы
- `green-50` - очень светлый зелёный
- `green-500` - средний зелёный
- `green-600` - насыщенный зелёный (кнопки)
- `green-700` - тёмный зелёный (hover)
- `green-800` - очень тёмный зелёный (текст)
- `emerald-100` - светлый изумрудный (градиент)

## Проверка
```bash
# Сборка проекта
trunk build --release

# Результат: ✅ success
```

## Соответствие conventions.md
✅ Модульная структура сохранена
✅ Типобезопасность соблюдена
✅ pub(crate) для внутренних функций
✅ Accessibility атрибуты сохранены
✅ Focus indicators обновлены
✅ Semantic HTML не изменён
✅ ARIA labels сохранены

## Визуальные изменения
- Все интерактивные элементы (кнопки, ссылки) теперь зелёные
- Фоновый градиент изменён с сине-фиолетового на зелёный
- Акцентные элементы (badges, highlights) используют зелёную палитру
- Информационные блоки с зелёным оттенком
- FAQ активные элементы с зелёной обводкой
- Сообщения пользователя в чате с зелёным фоном

## Следующие шаги
- [ ] Проверить контрастность для accessibility (WCAG AA)
- [ ] Протестировать на разных устройствах
- [ ] Получить обратную связь от пользователей
- [ ] Обновить og:image с новой цветовой схемой (если требуется)