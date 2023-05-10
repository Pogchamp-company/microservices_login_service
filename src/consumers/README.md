# RabbitMQ Consumers

## Create user
### Command name: ```create_user```

### Schema example
```json
{
    "email": "example@example.com",
    "password": "qwerty",
    "roles": ["task_manager"],
    "employee_id": 1
}
```

## Delete user
### Command name: ```delete_user```

### Schema example
```json
{
    "employee_id": 1
}
```
