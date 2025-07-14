## Build tools & versions used

```toml
rust = "1.88.0"
docker = "28.3.0"
```

## Steps to run the app

```sh
docker compose up -d --build

# or

docker-compose up -d --build
```

### Execute in development

```sh
curl http://localhost:3333/status # Check if the app is running and healthy
curl http://localhost:3333/nodes  # Get all nodes
```

### Execute in production

```sh
curl https://lightning_stats.velloware.com/status # Check if the app is running
curl https://lightning_stats.velloware.com/nodes  # Get all nodes
```

## What was the reason for your focus? What problems were you trying to solve?

My main goal with this project was to demonstrate my ability to develop a robust backend application. I focused on implementing strong error handling, clean architecture and infrastructure, testing, CI/CD pipelines, DevOps practices, Docker and Docker Compose integration, PostgreSQL, and of course, Rust.

## How long did you spend on this project?

Approximately 10 to 12 hours of development.

## Did you make any trade-offs for this project? What would you have done differently with more time?

I gave a lot of thought to implementing a caching system or a more efficient way to integrate with the mempool.space API, but ultimately chose a simpler and more straightforward approach, focusing on code clarity and system robustness. With more time, I would have explored these optimizations in greater depth.

## What do you think is the weakest part of your project?

It relies on an external API (mempool.space) to fetch nodes data, which may lead to inconsistencies or delays if the API is unavailable or slow.

## Is there any other information you’d like us to know?

I aimed to showcase as much of my knowledge as possible in Rust, software architecture, testing, CI/CD, Docker, and DevOps. I believe the code is self-explanatory and well-structured. I also deployed the project to production using a VPS server with Nginx, Cloudflare, SSL, and Docker. Additionally, I implemented Continuous Integration (CI) and Continuous Deployment (CD) using GitHub Actions to run lint checks, tests, builds, and automate the deployment to production.
