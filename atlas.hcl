# The "local" environment represents our local testings.
locals {
  config = jsondecode(file("config.json"))
}

env "local" {
  url = "postgres://${local.config.database.username}:${local.config.database.password}@${local.config.database.host}:${local.config.database.port}/${local.config.database.database_name}?search_path=public&sslmode=disable"
  migration {
    dir = "file://migrations"
  }
}
