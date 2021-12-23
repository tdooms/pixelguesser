FROM rust

WORKDIR usr/src/pixelguesser

COPY backend/imager /usr/src/pixelguesser/imager/
COPY backend/sessions /usr/src/pixelguesser/sessions/
COPY data/images /usr/scr/pixelguesser/images

RUN cargo install --path ./imager
RUN cargo install --path ./sessions

EXPOSE 8901
EXPOSE 8902

CMD imager -p 8901 --folder images && sessions -p 8902
