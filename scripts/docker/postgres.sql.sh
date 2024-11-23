docker run \
        --name axum_rs_pg \
        -e POSTGRES_PASSWORD=axum_rs \
        -e POSTGRES_USER=axum_rs \
        -e POSTGRES_DB=axum_rs \
        -e TZ=PRC \
        --restart=always \
        -e PGDATA=/var/lib/postgresql/data/pgdata \
        -v /var/docker/axum_rs_pg:/var/lib/postgresql/data \
        -p 127.0.0.1:55432:5432 \
        -d postgres:alpine

# postgres://axum_rs:axum_rs@127.0.0.1:55432/axum_rs?sslmode=disable