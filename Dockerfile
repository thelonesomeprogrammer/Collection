FROM alpine:3.18.4

COPY ./target/release/web /bin/web
COPY ./backend/data/ /collection/backend/
COPY ./ui/dist/ /collection/ui/

CMD ["sh"]

