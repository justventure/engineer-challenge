# Auth Service

## Запуск

```bash
make up
```

## Функционал

1. Регистрация
2. Авторизация
3. Смена пароля
4. Восстановление по почте

## ADR

### [backend](./backend)

GraphQL поддерживает `Set-Cookies`.
Паттерны **DDD** и **DI**.

### [frontend](./frontend)

### Trade-offs

1. Есть дублирование стилей/tsx. (скорость прототипирования)
2. Использование redux. (скорость прототипирования + архитектура)
3. Неявный FSD. (скорость прототипирования)
4. Webpack (HMR)

Монорепозиторий на **webpack** (поддержка HMR), **Nx**, **Next.js**.
**Redux**, позволяет поддерживать порядок в коде.

### Проблемные места

1. Нет маппинга ошибок от server в redux.
2. После login, registeration нет редиректов на homepage.
3. Нет ошибки которая показывает что пользователь уже авторизирован.
4. Нету rate-limiting.
5. Hardcode

### Pinterest

- [moodboard](https://ru.pinterest.com/veniaminshp/moodboard/)
- [antimoodboard](https://ru.pinterest.com/veniaminshp/antimoodboard/)
