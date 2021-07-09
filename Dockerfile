FROM rust

ENV SQLX_OFFLINE true
WORKDIR usr/src/pixelguesser

COPY api /usr/src/pixelguesser/api/
COPY backend /usr/src/pixelguesser/backend/
COPY .env /usr/src/pixelguesser

RUN cargo install --path ./backend

EXPOSE 6561

CMD pixelguesser
