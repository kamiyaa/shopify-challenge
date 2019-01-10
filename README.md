# Shopify Challenge

This is my attempt at the Shopify Summer 2019 Developer Intern Challenge Question

## Dependencies
 - Rocket >= 0.4.0
 - Rust nightly
 - Cargo nightly

## Usage
Edit Rocket.toml to configure server settings.
e.g. setting development deployment settings
```
[development]
address = "localhost"
port = 8000
```
setting production deployment settings
```
[production]
address = "192.168.0.1"
port = 8000
```

```
~$ cargo build	# compile
~$ ROCKET_ENV=development ./target/debug/shopify-challenge	# for development deployment
~$ ROCKET_ENV=production ./target/debug/shopify-challenge	# for production deployment
```

Head to:
```
/			# landing page
/api/products		# get all products
/api/products/<title>	# get specific product
```

##
