#!/usr/bin/env bash
# Install the akas.ttmi.my.id nginx site and issue its certificate.
#
# Run from anywhere:
#   /home/ubuntu/app/akasha-rust/deploy/install-akas.sh
#
# Needs sudo. The service itself does not: akasha-service.service and
# akasha-fetch-proxy.service are user units that must already be running.
set -euo pipefail

DOMAIN=akas.ttmi.my.id
HERE="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
AVAILABLE="/etc/nginx/sites-available/$DOMAIN.conf"
ENABLED="/etc/nginx/sites-enabled/$DOMAIN.conf"

install_site() {
    sudo install -m 0644 "$1" "$AVAILABLE"
    sudo ln -sfn "$AVAILABLE" "$ENABLED"
    sudo nginx -t
    sudo systemctl reload nginx
}

# 1. Serve the ACME challenge and proxy over plain HTTP while there is no cert.
if [ ! -f "/etc/letsencrypt/live/$DOMAIN/fullchain.pem" ]; then
    install_site "$HERE/nginx/$DOMAIN.http.conf"

    # 2. Issue the certificate without letting certbot rewrite the site file.
    sudo certbot certonly --webroot -w /var/www/html -d "$DOMAIN" \
        --non-interactive --keep-until-expiring --agree-tos
else
    echo "certificate for $DOMAIN already exists"
fi

# 3. Install the final TLS config.
install_site "$HERE/nginx/$DOMAIN.conf"

# 4. Verify through the public hostname.
curl -fsS "https://$DOMAIN/health"
echo
echo "done: https://$DOMAIN/health"
