# Prismatic-diary
## About this
[こちらの作品](https://github.com/hrm1810884/prismatic-diary-socket)のバックエンド・コードです  
Rustで書かれており、以下のコマンドで起動します
```sh
make
```
## connect to DB
mysqlをDBとして使用します
```sh
mysql -u root -p
CREATE DATABASE prisma_database;
CREATE USER 'prismatic'@'localhost' IDENTIFIED BY 'your_password_here';
GRANT ALL PRIVILEGES ON prisma_database.* TO 'prismatic'@'localhost';
FLUSH PRIVILEGES;
```
