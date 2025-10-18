# Наш перший веб3 застосунок

## Запуск

Для запуску проекту вам вам варто мати встановлений `rust` та `cargo`, зверністься до [https://rust-lang.org/learn/get-started/](https://rust-lang.org/learn/get-started/) для їх встановлення.

1. клонуйте репозиторій
   ```sh
   $ # Для користувачів git:
   $ git clone https://github.com/avramenko-ihor-chnu/our-web3-application

   $ # Для користувачів jj:
   $ jj git clone https://github.com/avramenko-ihor-chnu/our-web3-application
   ```

2. Зайдіть у директорію
   ```sh
   $ cd our-web3-application
   ```

3. Побудуйте та запустіть проект
   ```sh
   $ cargo run
   ```

## Внесок

### Проста зміна

Для того, щоб почати працювати на репозиторієм вам варто

1. клонувати його
   ```sh
   $ jj git clone https://github.com/avramenko-ihor-chnu/our-web3-application
   ```
2. ініціалізувати нову зміну (наприклад ви бажаєте змінити логотип)
   ```sh
   $ jj new -m "logo change"
   ```
3. змінити код
4. відправити вашу зміну на github
   ```sh
   $ jj git push -c @
   ```

Я отримаю повідомлення, та відреагую на нього, або я додам вашу зміну у головну гілку, або відправлю коментар до вашої зміни (слідкуйте за вашим github), ви можете відреагувати на мою відповідь кількома способами відповідно, зверністься до туторіалу на [https://steveklabnik.github.io/jujutsu-tutorial/sharing-code/updating-prs.html#responding-to-pull-request-feedback](https://steveklabnik.github.io/jujutsu-tutorial/sharing-code/updating-prs.html#responding-to-pull-request-feedback) за нагадуванням

### Синхронізування шаблонів з бекенд кодом

Фронтенд проекту використувує `htmx` що означає, що стан веб застусунку покладається на `backend` і важливо, щоб `backend` розробник як можна швидше та якісніше реагував на зміни `frontend` розробника.
Для підвищення продуктивності колаборації пропоную наступний алгоритм змін до `frontend`:

- Ви бажаєте мати новий функціонал, наприклад вивдення **топ 5 найдорожчих криптовалют**
- Ви створуєте створюєте шаблон, наприклад
  ```jinja
  <!-- server/templates/crypto-top-table.html -->
  <h2>Топ 5 надорожчих криптовлют</h2>
  <table class="table">
   <thead>
     <tr>
       <th>Місце</th>
       <th>Назва</th>
       <th>Вартість у USD</th>
     </tr>
   </thead>
   <tbody>
     {% for row in rows %}
     <tr>
       <td>{{ loop.index }}</td>
       <td>{{ row.name }}</td>
       <td>{{ row.price }}</td>
     </tr>
     {% endfor %}
   </tbody>
  </table>
  ```
- Також внесіть зміну, до `server/src/main.rs`
  - Знадіть роутер у функції `main` та додайте до неї новий слях:
  ```diff
    let app = Router::new()
        .route("/", get(index))
        .route("/hello-world", get(hello_world))
  +     .route("/crypto-top", get(crypto_top))
        .route("/favicon.ico", get(favicon))
        .nest_service("/static", ServeDir::new("server/static"));
  ```
  - ініціалізуйте функцію, яка відреагує на ваш запит та опишіть чого ви бажаєте
  ```diff
    async fn foo() {...}
  + async fn crypto_top() -> Result<Html<String> StatusCode> {
  + todo!()
  + // Створив шаблон crypto-top-table.html
  + // бажаю отримати топ криптовалют (місце, назва, ціна)
  + }
    async fn bar() {...}
  ```
  - якщо ваша функція потребує отримання данних на бекенд створіть об'єкт, що буде їх енкапсулювати, в нашому можливо
    ```rust
    #[derive(askama::Template)]
    #[template(path = "crypto-top-table.html")]
    struct CryptTopTable {
        rows: Vec<CryptTopRow>,
    }

    struct CryptTopRow {
        name: u32,
        price: u32,
    }
    ```
