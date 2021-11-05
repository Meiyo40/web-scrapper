# WHAT IS THIS PROJECT ?

It's a simple project, I want to learn Rust because the little i've seen is really enjoying to me.
So, trying to apply my little knowledge step by step by writing a simple program.

This webscrapper will be used to retrieve data from website without newletters/response alert, one of my favorite news website using Wordpress doesn't included this feature,
so, I'm trying to code my own alert system using a Rust scraper to find new article/comments, and send an event to trigger the update/mailing by a Springboot (or maybe Symfony IDK) service, sharing a database to give the opportunity to others users of the website to subscribe (NYI), with a frontend React app (Not coded already, step by step I said!)

Since I've already a lot to learn with Spring/Symfony/React and Rust is not my top priority, the project will take some times to be online and shared.

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

[setup_options]
is_debug_mode = false
```

The field last_update will store the timestamp of every update and avoid too much request (Not sure if the system will keep this)
# PLAN FOR PROJECT

![alt](https://i.ibb.co/6b3c6D2/Web-Scrapper-Def.png)
