# Deploying akasha-service at akas.ttmi.my.id

## What runs where

| Component | Listener | Unit |
| --- | --- | --- |
| `akasha-service` (Rust, this repo) | `0.0.0.0:8099` | `akasha-service.service` |
| `scripts/akasha_fetch_proxy.py` | `127.0.0.1:8097` | `akasha-fetch-proxy.service` |
| nginx | `:80`, `:443` | system nginx |

The Rust service talks to the fetch proxy instead of akasha.cv directly because
Cloudflare challenges this host's plain TLS fingerprint. The proxy is loopback
only and forwards just `GET /api/*`.

## 1. Build and start the user services (no sudo)

```sh
cd /home/ubuntu/app/akasha-rust
cargo build --release -p akasha-service
install -m 0755 target/release/akasha-service ~/.local/bin/akasha-service

mkdir -p ~/.config/systemd/user
cp deploy/systemd/akasha-fetch-proxy.service ~/.config/systemd/user/
cp deploy/systemd/akasha-service.service ~/.config/systemd/user/
systemctl --user daemon-reload
systemctl --user enable --now akasha-fetch-proxy.service akasha-service.service
```

Verify:

```sh
curl -sS http://127.0.0.1:8099/health
curl -sS http://127.0.0.1:8099/v1/users/901211014/characters | head -c 200
```

Configuration lives in `.env` (`PORT`, `AKASHA_BASE_URL`, `CACHE_TTL_SECS`).

## 2. Install the nginx site and certificate (needs sudo)

```sh
sudo /home/ubuntu/app/akasha-rust/deploy/install-akas.sh
```

The script:

1. installs `deploy/nginx/akas.ttmi.my.id.http.conf` so the ACME challenge can
   be served over plain HTTP,
2. runs `certbot certonly --webroot -w /var/www/html -d akas.ttmi.my.id`,
3. installs `deploy/nginx/akas.ttmi.my.id.conf` with TLS, redirect and rate
   limiting,
4. reloads nginx and checks `https://akas.ttmi.my.id/health`.

DNS for `akas.ttmi.my.id` already points at `43.156.92.36` (this host).

## 3. Operate

```sh
systemctl --user status akasha-service akasha-fetch-proxy
journalctl --user -u akasha-service -f
journalctl --user -u akasha-fetch-proxy -f
```

Restart after a configuration change:

```sh
systemctl --user restart akasha-fetch-proxy akasha-service
```

The services survive logout because linger is enabled for the `ubuntu` user.
