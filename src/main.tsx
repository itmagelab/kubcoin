import React, { useEffect, useState } from 'react';
import { createRoot } from 'react-dom/client';
import { parse } from 'yaml';
import './style.css';
import ruRaw from '../static/i18n/ru.yaml?raw';
import enRaw from '../static/i18n/en.yaml?raw';

type Dialog = { req: string; res: string };
type Chat = { title: string; subtitle: string; dialogs: Dialog[] };
type Item = { icon: string; title: string; description: string };
type QA = { question: string; answer: string };
type Content = {
  header: { title: string; subtitle: string; cta_button: string; aria_main_actions: string };
  ui: Record<string, string>;
  features: Item[];
  security: Item[];
  chats: Chat[];
  qa: QA[];
  footer: { copyright: string; developed_by: string };
};

const translations = { ru: parse(ruRaw) as Content, en: parse(enRaw) as Content };
const socialLinks: { key: 'join_group' | 'join_channel' | 'vk_group'; href: string; icon: string }[] = [
  { key: 'join_group', href: 'https://t.me/itmagelab_ru_group', icon: '◉' },
  { key: 'join_channel', href: 'https://t.me/itmagelab_ru', icon: '↗' },
  { key: 'vk_group', href: 'https://vk.com/kubcoin_ru', icon: 'ᴠκ' },
];

function App() {
  const [language, setLanguage] = useState<'ru' | 'en'>(() => {
    try {
      return localStorage.getItem('kubcoin-language') === 'en' ? 'en' : 'ru';
    } catch {
      return 'ru';
    }
  });
  const [openFaq, setOpenFaq] = useState<number | null>(0);
  useEffect(() => {
    document.documentElement.lang = language;
    try {
      localStorage.setItem('kubcoin-language', language);
    } catch {
      // Language switching still works when storage is unavailable.
    }
  }, [language]);
  const c = translations[language];
  const t = c.ui;

  return <>
    <header className="topbar wrap">
      <a className="brand" href="#top" aria-label="KubCoin"><span className="brand-mark">K</span><span>KubCoin</span></a>
      <nav className="top-nav" aria-label={language === 'ru' ? 'Навигация' : 'Navigation'}>
        <a href="#features">{t.features_section_title}</a><a href="#examples">{t.chats_section_title}</a><a href="#faq">{t.faq_section_title}</a>
      </nav>
      <div className="top-actions">
        <div className="language" aria-label="Language">
          <button className={language === 'ru' ? 'selected' : ''} onClick={() => setLanguage('ru')} aria-pressed={language === 'ru'}>RU</button>
          <button className={language === 'en' ? 'selected' : ''} onClick={() => setLanguage('en')} aria-pressed={language === 'en'}>EN</button>
        </div>
        <a className="button button-small" href="https://t.me/kubcoin_bot" target="_blank" rel="noreferrer">{t.start_bot}<span>↗</span></a>
      </div>
    </header>

    <main id="main">
      <section className="hero wrap" id="top">
        <div className="hero-copy">
          <div className="eyebrow"><span className="pulse-dot" /> {language === 'ru' ? 'УМНЫЙ УЧЁТ В TELEGRAM' : 'SMART TRACKING IN TELEGRAM'}</div>
          <h1>{language === 'ru' ? <>Деньги под контролем.<br/><em>Жизнь — без таблиц.</em></> : <>Your money, in focus.<br/><em>Life, without spreadsheets.</em></>}</h1>
          <p className="hero-lede">{c.header.subtitle}</p>
          <div className="hero-actions">
            <a className="button" href="https://t.me/kubcoin_bot" target="_blank" rel="noreferrer"><span className="telegram-icon">➤</span>{t.start_bot}<span className="button-arrow">↗</span></a>
            <a className="text-link" href="#examples">{t.learn_more}<span>↓</span></a>
          </div>
          <div className="hero-note"><div className="avatar-stack"><i>А</i><i>М</i><i>+</i></div><span>{language === 'ru' ? 'Удобный учёт — прямо в чате' : 'Effortless tracking, right in chat'}</span></div>
        </div>
        <div className="hero-art" aria-label="Пример финансового отчёта KubCoin">
          <div className="orbit orbit-one"/><div className="orbit orbit-two"/>
          <div className="floating-tag tag-top"><span>✦</span> {language === 'ru' ? 'Умная аналитика' : 'Smart insights'}</div>
          <div className="phone">
            <div className="phone-top"><span>9:41</span><span className="phone-status">●●● ▰</span></div>
            <div className="phone-heading"><div className="bot-avatar">K</div><div><strong>KubCoin</strong><small>bot · {language === 'ru' ? 'онлайн' : 'online'}</small></div><span className="dots">•••</span></div>
            <div className="chat-date">{language === 'ru' ? 'СЕГОДНЯ' : 'TODAY'}</div>
            <div className="message outgoing">−850 продукты <small>10:42 ✓✓</small></div>
            <div className="message incoming"><p>✅ {language === 'ru' ? 'Операция добавлена' : 'Transaction added'}</p><div className="message-rule"/><div className="mini-row"><span>📆 {language === 'ru' ? 'День / месяц' : 'Day / month'}</span><b>−850 ₽ / 24 560 ₽</b></div><div className="mini-row"><span>🏷️ {language === 'ru' ? 'Категория' : 'Category'}</span><b>{language === 'ru' ? 'Продукты' : 'Groceries'}</b></div></div>
            <div className="message outgoing message-short">/report <small>10:43 ✓✓</small></div>
            <div className="mini-chart"><div className="chart-copy"><small>{language === 'ru' ? 'РАСХОДЫ ЗА МЕСЯЦ' : 'MONTHLY SPENDING'}</small><strong>32 480 ₽</strong><span><b>↘ 12%</b> {language === 'ru' ? 'к прошлому месяцу' : 'vs last month'}</span></div><div className="bars"><i/><i/><i/><i/><i/><i/><i/><i/><i/><i/><i/><i/></div></div>
            <div className="phone-input"><span>{language === 'ru' ? 'Сообщение' : 'Message'}</span><i>➤</i></div>
          </div>
          <div className="floating-tag tag-bottom"><span className="tag-check">✓</span><div><strong>{language === 'ru' ? 'Запись за 2 секунды' : 'Logged in 2 seconds'}</strong><small>{language === 'ru' ? 'Никаких сложных форм' : 'No forms to fill in'}</small></div></div>
        </div>
        <div className="hero-foot"><span><b>01</b> {language === 'ru' ? 'Никаких таблиц' : 'No spreadsheets'}</span><span><b>02</b> {language === 'ru' ? 'Просто напишите боту' : 'Just message the bot'}</span><span><b>03</b> {language === 'ru' ? 'Данные — под вашим контролем' : 'Your data, your control'}</span></div>
      </section>

      <section className="social-proof"><div className="wrap social-inner"><span className="social-label">{language === 'ru' ? 'СООБЩЕСТВО KUBCOIN' : 'KUBCOIN COMMUNITY'}</span><div className="social-links">{socialLinks.map(link => <a key={link.key} href={link.href} target="_blank" rel="noreferrer"><span>{link.icon}</span>{t[link.key]}</a>)}<a href="https://t.me/kubcoin_bot" target="_blank" rel="noreferrer"><span>✳</span> Telegram bot</a></div></div></section>

      <section id="features" className="section wrap">
        <div className="section-heading"><div><div className="eyebrow">{language === 'ru' ? 'МЕНЬШЕ РУТИНЫ, БОЛЬШЕ ЯСНОСТИ' : 'LESS BUSYWORK, MORE CLARITY'}</div><h2>{t.features_section_title}<span className="heading-period">.</span></h2></div><p>{t.features_section_subtitle}</p></div>
        <div className="feature-grid">{c.features.map((item, i) => <article className={`feature-card feature-${i}`} key={item.title}><div className="feature-icon">{item.icon}</div><div className="card-index">0{i + 1}</div><h3>{item.title}</h3><p>{item.description}</p><span className="card-corner">↗</span></article>)}</div>
      </section>

      <section id="examples" className="section examples-section"><div className="wrap"><div className="section-heading"><div><div className="eyebrow">{language === 'ru' ? 'ПОНЯТНО С ПЕРВОГО СООБЩЕНИЯ' : 'EASY FROM THE FIRST MESSAGE'}</div><h2>{t.chats_section_title}<span className="heading-period">.</span></h2></div><p>{language === 'ru' ? 'Посмотрите, как обычное сообщение превращается в понятный финансовый учёт.' : 'See how a simple message becomes a clear financial record.'}</p></div>
        <div className="chat-grid">{c.chats.map((chat, i) => <article className="chat-card" key={chat.title}><div className="chat-card-head"><div><span className="chat-count">0{i + 1} / 0{c.chats.length}</span><h3>{chat.title}</h3><p>{chat.subtitle}</p></div><span className="chat-mini-logo">K</span></div><div className="chat-preview">{chat.dialogs.map((dialog, j) => <div className="chat-pair" key={`${dialog.req}-${j}`}><div className="chat-bubble user-bubble">{dialog.req}</div><div className="chat-bubble bot-bubble"><span className="bot-label"><i/> KUBCOIN</span><pre>{dialog.res}</pre></div></div>)}</div></article>)}</div>
      </div></section>

      <section id="pricing" className="section wrap"><div className="section-heading"><div><div className="eyebrow">{language === 'ru' ? 'ПРОСТО И ПРОЗРАЧНО' : 'SIMPLE AND TRANSPARENT'}</div><h2>{t.pricing_section_title}<span className="heading-period">.</span></h2></div><p>{t.pricing_section_subtitle}</p></div>
        <div className="pricing-grid"><article className="price-card"><div className="price-top"><span className="plan-icon">✳</span><span className="plan-label">{language === 'ru' ? 'ДЛЯ ЛИЧНОГО УЧЁТА' : 'FOR PERSONAL TRACKING'}</span></div><h3>{t.pricing_free_title}</h3><div className="price">{t.pricing_free_price}<small> / {t.pricing_free_period}</small></div><ul>{['feature_unlimited_operations','feature_all_basic_functions','feature_basic_analytics','feature_expense_categories','feature_math_expressions','feature_monthly_reports','feature_community_support'].map(key => <li key={key}><span>✓</span>{t[key]}</li>)}</ul><a className="button button-outline" href="https://t.me/kubcoin_bot" target="_blank" rel="noreferrer">{t.start_bot}<span>↗</span></a></article>
        <article className="price-card premium-card"><div className="premium-ribbon">{t.pricing_premium_coming_soon}</div><div className="price-top"><span className="plan-icon premium-icon">✦</span><span className="plan-label">{language === 'ru' ? 'БОЛЬШЕ ВОЗМОЖНОСТЕЙ' : 'MORE POSSIBILITIES'}</span></div><h3>{t.pricing_premium_title}</h3><div className="price">{t.pricing_premium_price}<small> / {t.pricing_premium_period}</small></div><p className="plus-copy">{t.pricing_premium_everything_plus}</p><ul>{['feature_advanced_analytics','feature_charts_visualization','feature_budget_planning','feature_multiple_currencies','feature_goals_savings','feature_priority_support','feature_extended_backups','feature_ai_text_recognition','feature_ai_voice_recognition'].map(key => <li key={key}><span>✓</span>{t[key]}</li>)}</ul><button className="button disabled-button" disabled>{t.pricing_premium_in_development}</button></article></div>
        <div className="enterprise-note"><span>⌂</span><p><strong>{language === 'ru' ? 'Для команд и компаний' : 'For teams and companies'}</strong>{' — '}{t.pricing_enterprise_text} <a href="https://t.me/itmagelab_ru_group" target="_blank" rel="noreferrer">{t.pricing_enterprise_contact} ↗</a></p></div>
      </section>

      <section id="security" className="section security-section"><div className="wrap"><div className="section-heading"><div><div className="eyebrow">{language === 'ru' ? 'ВАШИ ФИНАНСЫ — ВАШИ ПРАВИЛА' : 'YOUR FINANCES, YOUR RULES'}</div><h2>{t.security_section_title}<span className="heading-period">.</span></h2></div><p>{t.security_section_subtitle}</p></div><div className="security-grid">{c.security.map((item, i) => <article className="security-item" key={item.title}><div className="security-icon">{item.icon}</div><div><span className="card-index">0{i + 1}</span><h3>{item.title}</h3><p>{item.description}</p></div></article>)}</div><div className="security-tip"><span>✳</span><p><strong>{t.notification_tip}</strong> {t.notification_tip_text}</p></div></div></section>

      <section id="faq" className="section wrap faq-section"><div className="section-heading"><div><div className="eyebrow">{language === 'ru' ? 'ОТВЕТЫ БЕЗ ЛИШНИХ СЛОВ' : 'STRAIGHTFORWARD ANSWERS'}</div><h2>{t.faq_section_title}<span className="heading-period">.</span></h2></div><p>{language === 'ru' ? 'Не нашли ответ? Напишите нам — поможем разобраться.' : 'Still have questions? Get in touch and we’ll help.'}</p></div><div className="faq-list">{c.qa.map((item, i) => <article className={`faq-item ${openFaq === i ? 'is-open' : ''}`} key={item.question}><button onClick={() => setOpenFaq(openFaq === i ? null : i)} aria-expanded={openFaq === i}><span><i>{String(i + 1).padStart(2, '0')}</i>{item.question}</span><b>{openFaq === i ? '−' : '+'}</b></button>{openFaq === i && <div className="faq-answer">{item.answer}</div>}</article>)}</div></section>

      <section className="stats-band"><div className="wrap stats-inner"><div className="stats-intro"><div className="eyebrow">{language === 'ru' ? 'ПРОЕКТ РАСТЁТ ВМЕСТЕ С ВАМИ' : 'GROWING WITH YOU'}</div><h2>{t.stats_section_title}</h2></div><div className="stat"><strong>8</strong><span>{t.stats_total_usage}</span></div><div className="stat"><strong>1</strong><span>{t.stats_on_premise}</span></div><div className="stat"><strong>1K</strong><span>{t.stats_followers}</span></div><div className="stat"><strong>789</strong><span>{t.stats_likes}</span></div></div></section>

      <section className="final-cta wrap"><div className="cta-orb">✳</div><div><div className="eyebrow">{language === 'ru' ? 'НАЧАТЬ ПРОЩЕ, ЧЕМ КАЖЕТСЯ' : 'GET STARTED IN A MOMENT'}</div><h2>{language === 'ru' ? 'Попробуйте вести финансы по-новому.' : 'A clearer way to manage your money.'}</h2><p>{language === 'ru' ? 'Просто откройте Telegram. Остальное подскажет KubCoin.' : 'Open Telegram. KubCoin will take it from there.'}</p></div><a className="button" href="https://t.me/kubcoin_bot" target="_blank" rel="noreferrer">{t.start_bot}<span>↗</span></a></section>
    </main>

    <footer className="footer"><div className="wrap footer-inner"><a className="brand footer-brand" href="#top"><span className="brand-mark">K</span><span>KubCoin</span></a><div className="footer-copy"><p>{c.footer.copyright}</p><p>{c.footer.developed_by}</p></div><div className="footer-social"><a href="https://t.me/itmagelab_ru" target="_blank" rel="noreferrer">Telegram ↗</a><a href="https://vk.com/kubcoin_ru" target="_blank" rel="noreferrer">VK ↗</a></div></div></footer>
  </>;
}

createRoot(document.getElementById('app')!).render(<React.StrictMode><App /></React.StrictMode>);
