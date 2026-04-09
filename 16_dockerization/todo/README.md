## Migrations using sqlx

```
sqlx migrate add create_todos_table
```

## TESTING

1. Create new todo
    ```
    curl -X POST http://127.0.0.1:8080/todos -H "Content-Type:application/json" -d '{"title": "Learn Rust"}'
    ```

    `Expected:`
    ```
    {"id":1,"title":"Learn Rust","completed":false}
    ```

2. Create second todo
    ```
    curl -X POST http://127.0.0.1:8080/todos -H "Content-Type: application/json" -d '{"title": "Write my own book"}'
    ```

    `Expected:`
    ```
    {"id":2,"title":"Write my own book","completed":false}
    ```


3. Get all todos
    ```
    curl http://127.0.0.1:8080/todos
    ```

    `Expected:`
    ```
    [{"id":1,"title":"Learn Rust","completed":false},{"id":2,"title":"Write my own book","completed":false}]
    ```


4. Single todo by ID
    ```
    curl http://127.0.0.1:8080/todos/1
    ```

    `Expected:`
    ```
    {"id":1,"title":"Learn Rust","completed":false}
    ```


5. Get non-existent todo
    ```
    curl -I http://127.0.0.1:8080/todos/99
    ```

    `Expected:`
    ```
    HTTP/1.1 404 Not Found
    content-length: 0
    date: Sun, 29 Mar 2026 14:29:53 GMT
    ```

6. Update a todo - mark completed
    ```
    curl -X PATCH http://127.0.0.1:8080/todos/1 -H "Content-Type: application/json" -d '{"completed": true}'
    ```

    `Expected:`
    ```
    {"id":1,"title":"Learn Rust","completed":true}
    ```


7. Verify Update
    ```
    curl http://127.0.0.1:8080/todos/1
    ```

    `Expected:`
    ```
    {"id":1,"title":"Learn Rust","completed":true}
    ```


8. Delete a todo
    `-v` for verbose output
    ```
    curl -v -X DELETE http://127.0.0.1:8080/todos/1
    ```

    `Expected:`
    ```
    *   Trying 127.0.0.1:8080...
    * Connected to 127.0.0.1 (127.0.0.1) port 8080
    > DELETE /todos/1 HTTP/1.1
    > Host: 127.0.0.1:8080
    > User-Agent: curl/8.7.1
    > Accept: */*
    > 
    * Request completely sent off
    < HTTP/1.1 204 No Content
    < date: Sun, 29 Mar 2026 14:30:45 GMT
    < 
    * Connection #0 to host 127.0.0.1 left intact
    ```

9. Verify the deletion 
    ```
    curl http://127.0.0.1:8080/todos
    ```

    `Expected:`
    ```
    [{"id":2,"title":"Write my own book","completed":false}]
    ```
    

- Check CORS

    ```
    curl -v -X OPTIONS http://localhost:8080/todos \ -H "Origin: http://localhost:8001" \ -H "Access-Control-Request-Method: POST" \ -H "Access-Control-Request-Headers: content-type"
    ```

    `Expected:`

    ```
    * Host localhost:8080 was resolved.
    * IPv6: ::1
    * IPv4: 127.0.0.1
    *   Trying [::1]:8080...
    * connect to ::1 port 8080 from ::1 port 51471 failed: Connection refused
    *   Trying 127.0.0.1:8080...
    * Connected to localhost (127.0.0.1) port 8080
    > OPTIONS /todos HTTP/1.1
    > Host: localhost:8080
    > User-Agent: curl/8.7.1
    > Accept: */*
    > 
    * Request completely sent off
    < HTTP/1.1 200 OK
    < vary: origin, access-control-request-method, access-control-request-headers
    < access-control-allow-methods: GET,POST,PATCH,DELETE
    < access-control-allow-headers: content-type
    < access-control-allow-origin: *
    < allow: GET,HEAD,POST
    < content-length: 0
    < date: Sun, 29 Mar 2026 19:10:08 GMT
    ```


## Assignment Endpoints

- Status endpoint

    ```
    curl http://127.0.0.1:8080/status
    ```
    `Expected:`
    ```
    Server Status: OK. Database Pool Ready.
    ```

- Echo endpint

    ```
    curl http://127.0.0.1:8080/echo/Hellooo....
    ```
    `Expected:`
    ```
    You sent: [Hellooo....]
    ```


## WASM Client

```bash
cd wasm_client
```

```bash
miniserve www/ --index index.html --port 8001
```