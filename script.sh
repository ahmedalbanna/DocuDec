sudo mysql;
CREATE DATABASE docudec;
CREATE USER 'ahmed'@'localhost' IDENTIFIED BY 'admin';
GRANT ALL PRIVILEGES ON * . * TO 'ahmed'@'localhost';
FLUSH PRIVILEGES;
exit;
sudo service mysql restart