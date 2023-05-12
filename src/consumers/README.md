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

## Overwrite roles
Deletes current user roles and adds supplied roles
### Command name: ```overwrite_roles```

### Schema example
```json
{
    "employee_id": 1,
    "roles": ["task_manager"]
}
```
