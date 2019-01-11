# Shopify Challenge

This is my attempt at the Shopify Summer 2019 Developer Intern Challenge Question

## Dependencies
 - [Rocket](https://rocket.rs/) >= 0.4.0
 - Rust nightly
 - Cargo nightly

## Usage

### Configuration
Edit `Rocket.toml` to configure server settings.
e.g. setting development deployment settings
```toml
[development]
address = "localhost"
port = 8000
```

setting production deployment settings
```toml
[production]
address = "192.168.0.1"
port = 8000
```

### Running
Running the server:
```sh
~$ cargo build	# compile
~$ ROCKET_ENV=development ./target/debug/shopify-challenge	# for development deployment
~$ ROCKET_ENV=production ./target/debug/shopify-challenge	# for production deployment
```

### API Utilization
Head to:
```sh
/			# landing page
/api/products		# get all products
/api/products/<title>	# get specific product
```

#### examples
```
http://localhost:8000/api/products		# for a list of all products
http://localhost:8000/api/products/Apples	# for specific product
http://localhost:8000/api/products/Pears
http://localhost:8000/api/products/Banana
```

#### example output

Specific Product:
```json
{
  "title": "Pears",
  "price": 6.0,
  "inventory": 15
}
```
All Products:
```json
{
  "results": [
    {
      "title": "Pears",
      "price": 6.0,
      "inventory": 15
    },
    {
      "title": "Banana",
      "price": 4.0,
      "inventory": 19
    },
    {
      "title": "Apples",
      "price": 5.0,
      "inventory": 10
    }
  ]
}
```


##
