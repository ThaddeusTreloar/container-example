FROM python:latest

WORKDIR /opt/thermite/fake

COPY ./log.log /opt/thermite/fake/
COPY ./main.py /opt/thermite/fake/

CMD ["sh", "-c", "touch /opt/thermite/var/log && python ./main.py 1>$LOG_PATH 2>$LOG_PATH" ]