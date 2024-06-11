# Bader Sett
small api to access the badger db

## Setup
    cargo install
    cargo install sqlx-cli --no-default-features --features sqlite

## Update sqlx query definitions
This is needed to update the typing information for the query strings

    DATABASE_URL="sqlite:$(pwd)/badger.db" cargo sqlx prepare

## Usage
To start the server run

    cargo run

## REST interface

### `GET /member/<fobId>`

Retrieves the entry for a specific member using his fob ID.

#### Example
    curl http://127.0.0.1:8000/api/v1/member/abcd1234
```json
{
  "fobId": "abcd1234",
  "name": "Peter Mustermann",
  "contactData": "Call me at 0800123123"
}
```

### `POST /member`

Creates a new entry for a member.

#### Example

    curl -v -X POST -d '{ "fobId": "abcd1234", "name": "Peter Mustermann", "contactData": "Call me at 0800123123" }' -H 'Content-Type: application/json' http://127.0.0.1:8000/api/v1/member

```json
{
  "fobId": "abcd1234",
  "name": "Peter Mustermann",
  "contactData": "Call me at 0800123123"
}
```


### `PUT /member/<fobId>`

Updates an existing entry for a member.

#### Example
curl -v -X PUT -d '{ "fobId": "abcd1234", "name": "Peter Mustermann", "contactData": "Email me: peter@mustermann.de" }' -H 'Content-Type: application/json' http://127.0.0.1:8000/api/v1/member/abcd1234
```json
{
  "fobId": "abc1234",
  "name": "Peter Mustermann",
  "contactData": "Email me: peter@mustermann.de"
}
```

### `DELETE /member/<fobId>`

Remove an entry for a member.

#### Example

curl -v -X DELETE http://127.0.0.1:8000/api/v1/member/abcd1234
```json
{ }
```
