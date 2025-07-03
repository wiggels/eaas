# Errors-as-a-Service

Where failure is always an option.

---

## 🤔 What is this?

A dead-simple actix-based API for developers who know staging is a lie and “100% uptime” is marketing fiction.

Use it to simulate random HTTP errors for your app — on purpose — so you can test your retry logic before your users do.

---

## ⚡ How it works

This service exposes three endpoints:

- `/api/host` — Returns random **5xx** (server-side) errors.
- `/api/client` — Returns random **4xx** (client-side) errors.
- `/api/any` — Returns either.

Each one has a `chance` query param that controls the odds you’ll get an error response.

---

## ✅ Example usage

```http
GET /api/host?chance=80
````

With `chance=80`, you have an 80% chance of seeing a random server-side error like `500`, `502`, `503`, or `504`.
The other 20% returns `200 OK` so you still think you might get lucky.

---

## 🔍 Query params

| Param  | Type | Default | Description                                    |
| ------ | ---- | ------- | ---------------------------------------------- |
| chance | u8   | 50      | Percentage chance (0-100) of getting an error. |

---

## 📚 Example responses

```text
200 OK
```

or

```text
503 SERVICE UNAVAILABLE
```

or

```text
404 NOT FOUND
```

---

## 🗂️ OpenAPI docs

This project uses [`utoipa`](https://github.com/juhaku/utoipa) for OpenAPI docs.
All endpoints are fully documented so you can point your favorite Swagger UI at it and marvel at your own chaos.

---

## 🚀 Running locally

```bash
# Clone the repo
git clone https://github.com/yourname/errors-as-a-service.git
cd errors-as-a-service

# Build and run
cargo run
```

Open `http://localhost:8080/api/any?chance=70` and enjoy the uncertainty.

---

## 🪵 Example use cases

- Load testing your frontend with realistic failure rates.
- Proving to your team that your fancy retry logic doesn’t work.
- Practicing how to write believable incident reports.

---

## ⚙️ How it works (under the hood)

- Written in **Rust** with **actix-web**.
- Uses `rand` to roll the dice each request.
- Selects a random HTTP status code from `CLIENT_ERRORS` or `SERVER_ERRORS` (or both!) depending on the endpoint.
- Defaults to a 50% chance of error if you forget to pass `chance`.
- Renders the canonical reason (e.g., `503 SERVICE UNAVAILABLE`).

Check `src/api.rs` for the sad truth.

---

## ⚡ Endpoints

```text
GET /api/any
GET /api/client
GET /api/host
```

---

## 🤝 Contributing

Pull requests welcome.
But please, keep the code readable enough that we can pretend to fix bugs later.

---

## 📜 License

MIT. Break your stuff responsibly.

---

> “If it works, we did something wrong.”
