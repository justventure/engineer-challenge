# Auth Service
<div align="left">
  <a href="https://sonarcloud.io/summary/new_code?id=vwency_engineer-challenge"><img src="https://sonarcloud.io/api/project_badges/measure?project=vwency_engineer-challenge&metric=alert_status"/></a>
  <a href="https://sonarcloud.io/summary/new_code?id=vwency_engineer-challenge"><img src="https://sonarcloud.io/api/project_badges/measure?project=vwency_engineer-challenge&metric=bugs"/></a>
  <a href="https://sonarcloud.io/summary/new_code?id=vwency_engineer-challenge"><img src="https://sonarcloud.io/api/project_badges/measure?project=vwency_engineer-challenge&metric=code_smells"/></a>
  <img src="https://img.shields.io/github/license/vwency/engineer-challenge"/>
  <img src="https://img.shields.io/github/last-commit/vwency/engineer-challenge"/>
  <img src="https://img.shields.io/github/repo-size/vwency/engineer-challenge"/>
  <img src="https://img.shields.io/github/issues/vwency/engineer-challenge"/>
  <img src="https://img.shields.io/badge/rust-1.95.0--nightly-orange?logo=rust"/>
  <img src="https://img.shields.io/badge/unsafe-forbidden-success"/>
  <a href="https://github.com/vwency/engineer-challenge/actions/workflows/backend-push.yaml"><img src="https://github.com/vwency/engineer-challenge/actions/workflows/backend-push.yaml/badge.svg"/></a>
</div>

## Description 
Проект реализует функции восстановление пароля, регистрация, авторизации, максимально приближенные к prod-ready решениям. С кэшированием в valkey(open source форк redis)
 
## Architecture

**DDD**  
- Фокус на доменной логике  (entities, port/in ports/out)
- Улучшенная поддерживаемость  
- Чёткое разделение бизнес-слоёв

**DI**  
- Слабая связанность компонентов  
- Упрощённое тестирование  
- Гибкость замены реализаций

**CQRS**  
- Разделение операций чтения и записи  
- Оптимизация I/O  
- Улучшенная масштабируемость  

**ADR References:**  
- [Cookie-based Session Authentication](./docs/adr/0001-cookie-session.md)  
- [Valkey Cache for Session Profiles](./docs/adr/0003-valkey-cache.md)  

## Tech stack
1. **REST**, поскольку поддерживает в запросе `Set-Cookies`, статус коды http.
2. **Yarn berry** большое сообщество, кастомизация.  
3. **NX** время сборки, уменьшение времени на CI.  
4. **Rust** строгая типипизация, гарантия доставки, гибкость в архитектуре.
5. **Valkey**  Поддержка — Valkey поддерживается крупными компаниями: AWS, Google, Oracle, Ericsson. Redis Ltd. — единственный вендор Redis OSS.


## Trade-offs  
| Решение | Причина | Когда пересмотреть |
|---|---|---|
| Дублирование стилей/tsx | Скорость прототипирования | Перед подготовкой к prod-ready |
| Redux | Скорость прототипирования + архитектура | Возможен пересмотр при разработке |
| Webpack (HMR, hot-reload) | HMR из коробки, turbopack его не поддерживает | При появлении HMR в turbopack |
| Нет подтверждения пароля по почте при регистрации | Время отладки | Рефакторинг во время разработки |
| Нет полноценного IaC | Время | При enterprise подготовке к prod |
| Cookie-based сессии вместо JWT | Один сервис, нет экосистемы; сессия шарится cross-domain с `credentials: include` | При масштабировании или добавлении новых сервисов |
| Auth-сервис как единый Bounded Context | Дробить BC внутри одного сервиса — over-engineering без реальных причин | При выделении отдельных поддоменов в рамках роста системы |
| Ory экосистема | гибкость конфигурации и интеграция с hydra для OpenID | При enterprise+ |

### Continue
1. GitOps — чтение новых helm релизов и их применение.
2. Coverage тесты в CI, codecov, SonarQube.  
3. Нагрузочные тесты на GetCurrentUserQuery, Commands

Схема command запроса:
Схема command запроса:
```mermaid
flowchart LR
    Client[HTTP Client]
    Client -->|UserIp X-Forwarded-For| RateLimit{RateLimiter}
    RateLimit -->|Exceeded| RestError[REST Error 429]
    RateLimit -->|OK| TryFrom

    subgraph Validation
        TryFrom -->|Email + Password VO| LoginCommand
        TryFrom -->|Err| RestError
    end

    subgraph Application
        LoginCommand --> LoginCommandHandler
    end

    subgraph Initiate
        LoginCommandHandler -->|initiate_login cookie| AuthenticationPort
        AuthenticationPort --> KratosAuthenticationAdapter
        KratosAuthenticationAdapter -->|whoami| Kratos
        Kratos -->|SessionStatus| KratosAuthenticationAdapter
        KratosAuthenticationAdapter -->|fetch_flow| Kratos
        Kratos -->|flow_id + csrf_token| KratosAuthenticationAdapter
    end

    subgraph Complete
        LoginCommandHandler -->|complete_login credentials| AuthenticationPort
        AuthenticationPort --> KratosAuthenticationAdapter
        KratosAuthenticationAdapter -->|build| LoginPayload[LoginPayload Infra Model]
        LoginPayload -->|POST flow| Kratos
        Kratos -->|SessionCookie| KratosAuthenticationAdapter
    end

    KratosAuthenticationAdapter -->|SessionCookie| LoginCommandHandler
    LoginCommandHandler -->|session_token| Client
    Client -->|Set-Cookie| RestResponse[POST /auth/login Response]
```

Реализация кэша redis для запрос Query, что бы не загружать postgres.
```mermaid
flowchart TD
    Client[HTTP Client]
    Client -->|UserIp from X-Forwarded-For| RateLimit{RateLimiter}
    RateLimit -->|Exceeded| RestError[REST Error 429]
    RateLimit -->|OK| GetCurrentUserQuery
    Client -->|cookie from request| GetCurrentUserQuery

    GetCurrentUserQuery -->|cookie Option| GetCurrentUserQueryHandler
    GetCurrentUserQueryHandler -->|extract session token| CacheKey[cache_key: user_profile:token]

    CacheKey --> RedisLookup{Redis GET}
    RedisLookup -->|HIT| Deserialize[serde_json::from_str]
    Deserialize -->|UserProfile| RestResponse[GET /auth/me Response]

    RedisLookup -->|MISS| IdentityPort
    IdentityPort -->|get_current_user cookie| KratosIdentityAdapter
    KratosIdentityAdapter -->|GET /sessions/whoami| Kratos
    Kratos -->|401 Unauthorized| AuthError[AuthError::NotAuthenticated]
    AuthError --> RestError

    Kratos -->|SessionResponse| KratosIdentityAdapter
    KratosIdentityAdapter -->|traits.into| UserProfile
    UserProfile -->|serde_json::to_string| RedisSet[Redis SET EX cache_ttl_secs]
    RedisSet --> RestResponse
```

Валидация входных данных:
```mermaid
flowchart LR
    Input[REST Input Body]
    Input --> TryFrom[TryFrom]

    TryFrom --> VO[VO Email / Password]

    VO -->|Ok| Domain[Domain Object]
    VO -->|Err| Error[REST Error 422]

    Domain --> Handler[CommandHandler]
    Handler --> Adapter[KratosAdapter]

    Adapter --> Models[Infra Models]
    Models --> Kratos[Kratos]

    Kratos --> Response[FlowResult / PostFlowResult]

    Response --> Adapter
    Adapter --> Handler
    Handler --> RestResponse[REST Response]
```

## Running  
```bash
make up
```

## Testing  

Для запуска тестов в kratos требуется поднятие инфры (kratos, postgres, mailhog, redis):
```bash
cd web/backend/rust_kratos && make infra-up && cargo test ; cd ../../../
```

На фронтенде:
```bash
cd web/frontend && yarn install && yarn test ; cd ../../
```
