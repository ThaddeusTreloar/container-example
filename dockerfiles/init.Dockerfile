
FROM alpine:latest

WORKDIR /opt/thermite/init

COPY init/init.sh /opt/thermite/init/init.sh
RUN chmod 755 /opt/thermite/init/init.sh

CMD ["./init.sh"]