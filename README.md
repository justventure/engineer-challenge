# Auth Service

<div align="left">

  <!-- CI/CD -->
  <a href="https://github.com/vwency/engineer-challenge/actions/workflows/backend-push.yaml"><img src="https://github.com/vwency/engineer-challenge/actions/workflows/backend-push.yaml/badge.svg"/></a>
  <a href="https://github.com/vwency/engineer-challenge/actions/workflows/frontend-push.yaml"><img src="https://github.com/vwency/engineer-challenge/actions/workflows/frontend-push.yaml/badge.svg"/></a>

  <!-- Code quality -->
  <a href="https://sonarcloud.io/summary/new_code?id=vwency_engineer-challenge"><img src="https://sonarcloud.io/api/project_badges/measure?project=vwency_engineer-challenge&metric=security_rating"/></a>
  <a href="https://sonarcloud.io/summary/new_code?id=vwency_engineer-challenge"><img src="https://sonarcloud.io/api/project_badges/measure?project=vwency_engineer-challenge&metric=reliability_rating"/></a>
  <a href="https://sonarcloud.io/summary/new_code?id=vwency_engineer-challenge"><img src="https://sonarcloud.io/api/project_badges/measure?project=vwency_engineer-challenge&metric=sqale_rating"/></a>

  <!-- Meta -->
  <a href="https://github.com/vwency/engineer-challenge/blob/main/LICENSE"><img src="https://img.shields.io/github/license/vwency/engineer-challenge"/></a>
  <img src="https://img.shields.io/github/last-commit/vwency/engineer-challenge"/>

</div>

## Description
Проект реализует функции восстановление пароля, регистрация, авторизации, максимально приближенные к prod-ready решениям. С кэшированием в valkey(open source форк redis)  

---

<details>
<summary><strong>Architecture</strong></summary>  
<br>

| Паттерн | Что даёт |
|---|---|
| Hexagonal architecture | Отвязка реализации от транспорта через порты |
| DDD | Фокус на доменной логике, чёткое разделение бизнес-слоёв |
| DI | Слабая связанность, гибкость замены реализаций |
| CQRS | Разделение чтения/записи, оптимизация I/O, масштабируемость |

</details>

---  

<details>
<summary><strong>Tech stack</strong></summary>
<br>
    
| Технология | Причина выбора |
|---|---|
| REST | Поддержка `Set-Cookie` и HTTP статус-кодов в запросе |
| Yarn berry | Большое сообщество, гибкая кастомизация |
| NX | Ускорение сборки, сокращение времени CI |
| Rust | Строгая типизация, гарантия корректности, гибкость архитектуры |
| Valkey | Поддерживается AWS, Google, Oracle, Ericsson — в отличие от Redis OSS, где единственный вендор Redis Ltd. |

</details>

---

<details>
<summary><strong>Trade-offs</strong></summary>
<br>

| Решение | Причина | Когда пересмотреть |
|---|---|---|
| Дублирование стилей/tsx | Скорость прототипирования | Перед подготовкой к prod-ready |
| Redux | Скорость прототипирования + архитектура | Возможен пересмотр при разработке |
| Webpack (HMR, hot-reload) | HMR из коробки, turbopack его не поддерживает | При появлении HMR в turbopack |
| Нет подтверждения пароля по почте при регистрации | Время отладки | Рефакторинг во время разработки |
| Нет полноценного IaC | Время | При enterprise подготовке к prod |
| Cookie-based сессии вместо JWT | Один сервис, нет экосистемы | При масштабировании или добавлении новых сервисов |
| Auth-сервис как единый Bounded Context | Дробить BC — over-engineering | При выделении отдельных поддоменов |
| Ory экосистема | гибкость конфигурации и интеграция с hydra для OpenID | При enterprise+ |

</details>

---  

<details>
<summary><strong>Roadmap</strong></summary>
<br>

1. GitOps — чтение новых helm релизов и их применение.
2. Coverage тесты в CI, codecov, SonarQube.  
3. Нагрузочные тесты на GetCurrentUserQuery, Commands
4. SLA

</details>

---

<details>
<summary><strong>Diagrams</strong></summary>
<br>
Схема command запроса:

```mermaid
%%{init: {'theme': 'base', 'themeVariables': {'primaryColor': '#1E3A5F', 'primaryTextColor': '#FFFFFF', 'primaryBorderColor': '#2D5A8E', 'lineColor': '#EF4444', 'secondaryColor': '#162D4A', 'tertiaryColor': '#0F1F35', 'clusterBkg': '#0F1F35', 'clusterBorder': '#2D5A8E', 'titleColor': '#FFFFFF', 'edgeLabelBackground': '#1E3A5F', 'nodeTextColor': '#FFFFFF'}}}%%
flowchart LR
    Client[HTTP Client]
    Client -->|Email + Password| TryFrom
    subgraph Validation
        TryFrom -->|Email + Password VO| LoginCommand
        TryFrom -->|Err| RestError[REST Error 422]
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
        LoginCommandHandler -->|complete_login credentials| AuthenticationPort2[AuthenticationPort]
        AuthenticationPort2 --> KratosAuthenticationAdapter2[KratosAuthenticationAdapter]
        KratosAuthenticationAdapter2 -->|build| LoginPayload[LoginPayload Infra Model]
        LoginPayload -->|POST flow| Kratos2[Kratos]
        Kratos2 -->|SessionCookie| KratosAuthenticationAdapter2
    end
    KratosAuthenticationAdapter -->|SessionCookie| LoginCommandHandler
    LoginCommandHandler -->|session_token| Client
    Client -->|Set-Cookie| RestResponse[POST /auth/login Response]
```

Реализация кэша redis для запрос Query, что бы не загружать postgres.
```mermaid
%%{init: {'theme': 'base', 'themeVariables': {'primaryColor': '#1E3A5F', 'primaryTextColor': '#FFFFFF', 'primaryBorderColor': '#2D5A8E', 'lineColor': '#EF4444', 'secondaryColor': '#162D4A', 'tertiaryColor': '#0F1F35', 'clusterBkg': '#0F1F35', 'clusterBorder': '#2D5A8E', 'titleColor': '#FFFFFF', 'edgeLabelBackground': '#1E3A5F', 'nodeTextColor': '#FFFFFF'}}}%%
flowchart TD
    Client[HTTP Client]
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
    AuthError --> RestError[REST Error 401]
    Kratos -->|SessionResponse| KratosIdentityAdapter
    KratosIdentityAdapter -->|traits.into| UserProfile
    UserProfile -->|serde_json::to_string| RedisSet[Redis SET EX cache_ttl_secs]
    RedisSet --> RestResponse
```

Валидация входных данных:
```mermaid
%%{init: {'theme': 'base', 'themeVariables': {'primaryColor': '#1E3A5F', 'primaryTextColor': '#FFFFFF', 'primaryBorderColor': '#2D5A8E', 'lineColor': '#EF4444', 'secondaryColor': '#162D4A', 'tertiaryColor': '#0F1F35', 'clusterBkg': '#0F1F35', 'clusterBorder': '#2D5A8E', 'titleColor': '#FFFFFF', 'edgeLabelBackground': '#1E3A5F', 'nodeTextColor': '#FFFFFF'}}}%%
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
</details>

---  

<details>
<summary><strong>Запуск & тесты</strong></summary>
<br>
    
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
</details>

<details>
<summary><strong>Показатели нагрузки</strong></summary>
<br>

```bash
✓ recovery 2xx
✓ login 2xx
✓ me 200
✓ register 2xx
✗ register <500ms
 ↳  50% — ✓ 3770 / ✗ 3719
✗ send_code 2xx
 ↳  0% — ✓ 0 / ✗ 4503

checks.........................: 87.38% 56964 out of 65186
data_received..................: 34 MB  148 kB/s
data_sent......................: 20 MB  87 kB/s
dropped_iterations.............: 12175  52.905724/s
http_req_blocked...............: avg=6.45µs   min=1.18µs   med=3.54µs   max=33.62ms p(90)=4.65µs  p(95)=5.18µs
http_req_connecting............: avg=2.02µs   min=0s       med=0s       max=33.58ms p(90)=0s      p(95)=0s
✗ http_req_duration..............: avg=421.38ms min=209.9µs  med=27.75ms  max=3.47s   p(90)=1.44s   p(95)=1.71s
  { expected_response:true }...: avg=450.74ms min=209.9µs  med=27.56ms  max=3.47s   p(90)=1.48s   p(95)=1.74s
✗ http_req_failed................: 7.80%  4503 out of 57697
http_req_receiving.............: avg=57.28µs  min=8.38µs   med=37.28µs  max=87.54ms p(90)=60.95µs p(95)=110.72µs
http_req_sending...............: avg=18.8µs   min=3.81µs   med=14.01µs  max=50.93ms p(90)=19.8µs p(95)=23.81µs
http_req_tls_handshaking.......: avg=0s       min=0s       med=0s       max=0s      p(90)=0s      p(95)=0s
http_req_waiting...............: avg=421.31ms min=187.96µs med=27.67ms  max=3.47s   p(90)=1.44s   p(95)=1.71s
http_reqs......................: 57697  250.718815/s
iteration_duration.............: avg=754.4ms  min=101.27ms med=397.27ms max=3.57s   p(90)=1.72s   p(95)=1.95s
iterations.....................: 37225  161.758981/s
vus............................: 0      min=0              max=300
vus_max........................: 300    min=50             max=300
```

![bench](./art/bench.png)

</details>
