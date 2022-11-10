FROM shoothzj/base

COPY dist /opt/mysql-dashboard

WORKDIR /opt/mysql-dashboard

EXPOSE 10008

CMD ["/usr/bin/dumb-init", "/opt/mysql-dashboard/mysql-dashboard"]
