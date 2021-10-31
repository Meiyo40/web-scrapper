# SETUP
Add a app_config.toml at the root
The struct for each website and user will be:

```
[[website]]
name = "google"
url = "http://www.google.com"

[[user_to_search]]
username = "bob"
last_update = 0
```

The field last_update will store the timestamp of every update and avoid too much request (Not sure if the system will keep this)
# PLAN FOR PROJECT

![alt](https://i.ibb.co/6b3c6D2/Web-Scrapper-Def.png)
