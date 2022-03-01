https://citizix.com/how-to-run-mysql-8-with-docker-and-docker-compose/

## docker 

❯ docker run -d \
    --name my-mysql \
    -p 3306:3306 \
    -v ~/apps/mysql:/var/lib/mysql \
    --user 1000:1000 \
    -e MYSQL_ROOT_PASSWORD=S3cret \
    -e MYSQL_PASSWORD=An0thrS3crt \
    -e MYSQL_USER=citizix_user \
    -e MYSQL_DATABASE=citizix_db \
    mysql:8


## docker-compose

docker-compose.yaml

~~~docker-compose

version: '3.9'

services:
  mysql:
    image: mysql:8
    ports:
      - 3306:3306
    volumes:
      - ~/apps/mysql:/var/lib/mysql
    environment:
      - MYSQL_ROOT_PASSWORD=S3cret
      - MYSQL_PASSWORD=An0thrS3crt
      - MYSQL_USER=citizix_user
      - MYSQL_DATABASE=citizix_db

~~~

## Wordpress & Docker

This file will setup Wordpress, MySQL & PHPMyAdmin with a single command. Add the code below to a file called "docker-compose.yaml" and run the command

>
        $ docker-compose up -d

        # To Tear Down
        $ docker-compose down --volumes

~~~

version: '3'

services:
  # Database
  db:
    image: mysql:5.7
    volumes:
      - db_data:/var/lib/mysql
    restart: always
    environment:
      MYSQL_ROOT_PASSWORD: password
      MYSQL_DATABASE: wordpress
      MYSQL_USER: wordpress
      MYSQL_PASSWORD: wordpress
    networks:
      - wpsite
  # phpmyadmin
  phpmyadmin:
    depends_on:
      - db
    image: phpmyadmin/phpmyadmin
    restart: always
    ports:
      - '8080:80'
    environment:
      PMA_HOST: db
      MYSQL_ROOT_PASSWORD: password 
    networks:
      - wpsite
  # Wordpress
  wordpress:
    depends_on:
      - db
    image: wordpress:latest
    ports:
      - '8000:80'
    restart: always
    volumes: ['./:/var/www/html']
    environment:
      WORDPRESS_DB_HOST: db:3306
      WORDPRESS_DB_USER: wordpress
      WORDPRESS_DB_PASSWORD: wordpress
    networks:
      - wpsite
networks:
  wpsite:
volumes:
  db_data:
~~~