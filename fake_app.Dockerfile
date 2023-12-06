FROM python:latest

WORKDIR /opt/thermite/fake_app

COPY .fake_app/log.log /opt/thermite/fake_app/
COPY .fake_app/main.py /opt/thermite/fake_app/
COPY bootstrap.sh /opt/thermite/fake_app/
RUN chmod 755 /opt/thermite/fake_app/bootstrap.sh


CMD ["./bootstrap.sh", "'python ./main.py'" ]